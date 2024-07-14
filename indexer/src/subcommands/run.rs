use farcaster_client::client::Client;
use farcaster_client::grpc::hub_event::Body as EventBody;
use farcaster_client::grpc::message_data::Body as MessageDataBody;
use farcaster_client::grpc::{HubEvent, HubEventType};
use service::sea_orm::DbConn;
use tokio::sync::mpsc;

pub async fn run(db: &DbConn) -> anyhow::Result<()> {
    let url = "http://[::1]:2283";

    let mut client = Client::new(url.to_string()).await?;

    let (tx, mut rx) = mpsc::channel::<HubEvent>(2048);
    client.subscribe(0, tx).await?;

    while let Some(event) = rx.recv().await {
        println!("rx get event: {:?}", event);

        let event_type = event.r#type();
        let event_body = event.body.unwrap();

        match event_type {
            HubEventType::MergeMessage => {
                if let EventBody::MergeMessageBody(msg_body) = event_body {
                    if let Some(message) = msg_body.message {
                        if let Some(message_data) = message.data {
                            if let Some(message_body) = message_data.body {
                                match message_body {
                                    MessageDataBody::CastAddBody(_) => {}
                                    MessageDataBody::CastRemoveBody(_) => {}
                                    _ => todo!(),
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

    Ok(())
}
