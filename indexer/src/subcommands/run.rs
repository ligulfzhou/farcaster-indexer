use farcaster_client::client::Client;
use farcaster_client::grpc::hub_event::Body as EventBody;
use farcaster_client::grpc::message_data::Body as MessageDataBody;
use farcaster_client::grpc::{HubEvent, HubEventType};
use prost::Message;
use service::sea_orm::DbConn;
use service::{mutation::Mutation, query::Query};

use tokio::sync::mpsc;

pub async fn run(db: &DbConn) -> anyhow::Result<()> {
    let url = "http://[::1]:2283";

    let mut client = Client::new(url.to_string()).await?;

    let (tx, mut rx) = mpsc::channel::<HubEvent>(2048);

    tokio::spawn(async move {
        client
            .subscribe(0, tx)
            .await
            .expect("subscribe to farcaster node should work");
    });

    while let Some(event) = rx.recv().await {
        let encoded = event.encode_to_vec();
        let event = HubEvent::decode(encoded);
        process_event(event, db).await;
    }

    Ok(())
}

async fn process_event(event: HubEvent, db: &DbConn) {
    let event_type = event.r#type();
    let event_body = event.body.unwrap();

    match event_type {
        HubEventType::MergeMessage => {
            if let EventBody::MergeMessageBody(msg_body) = event_body {
                if let Some(message) = msg_body.message {
                    if let Some(message_data) = message.data {
                        if let Some(message_body) = message_data.body {
                            match message_body {
                                MessageDataBody::CastAddBody(cab) => {
                                    Mutation::insert_cast(db)
                                        .await
                                        .expect("insert cast should work.");
                                }
                                MessageDataBody::CastRemoveBody(crb) => {
                                    println!("crb: {:?}", crb);
                                }
                                _ => {
                                    println!("tttttt");
                                }
                            }
                        }
                    }
                }
            }
        }
        HubEventType::PruneMessage => {}
        HubEventType::RevokeMessage => {}
        HubEventType::MergeOnChainEvent => {}
        _ => {
            dbg!("UNHANDLED HUB EVENT, ", event.id);
        }
    }
}
