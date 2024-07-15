use crate::rabbitmq::{get_consumer, get_mq_queue_channel};
use bytes::{Buf, Bytes};
use farcaster_client::client::Client;
use farcaster_client::grpc::hub_event::Body as EventBody;
use farcaster_client::grpc::message_data::Body as MessageDataBody;
use farcaster_client::grpc::{HubEvent, HubEventType};
use lapin::message::DeliveryResult;
use lapin::{Connection, ConsumerDelegate};
use prost::Message;
use service::sea_orm::DbConn;
use service::{mutation::Mutation, query::Query};
use std::future::Future;
use std::io::Read;
use std::pin::Pin;
use tokio::sync::mpsc;

pub async fn run(db: &DbConn) -> anyhow::Result<()> {
    let url = "http://[::1]:2283";

    let mut client = Client::new(url.to_string()).await?;

    // let (tx, mut rx) = mpsc::channel::<HubEvent>(2048);
    // tokio::spawn(async move {
    //     client
    //         .subscribe_to_mpsc(0, tx)
    //         .await
    //         .expect("subscribe to farcaster node should work");
    // });

    let (_, queue, chan) = get_mq_queue_channel().await;
    tokio::spawn(async move {
        client
            .subscribe_to_mq(0, queue, chan)
            .await
            .expect("subscribe to farcaster node with MQ");
    });

    let (conn, consumer) = get_consumer().await;
    let delegate = Delegate { db: db.clone() };
    consumer.set_delegate(delegate);
    conn.run();

    // while let Some(event) = rx.recv().await {
    //     let encoded = event.encode_to_vec();
    //     let cl = encoded.clone();
    //
    //     let buf = Bytes::from(cl);
    //     let rs = HubEvent::decode(buf).unwrap();
    //
    //     println!("before: {:?}, after: {:?}", event, rs);
    //     process_event(rs, db).await;
    // }

    Ok(())
}

struct Delegate {
    db: DbConn,
}

impl Delegate {
    pub async fn process_event(event: HubEvent) {
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
                                        // Mutation::insert_cast(db)
                                        //     .await
                                        //     .expect("insert cast should work.");
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
}

impl ConsumerDelegate for Delegate {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Ok(Some(deliveried)) = delivery {
                let data = deliveried.data;
                let buf = Bytes::from(data);
                let rs = HubEvent::decode(buf).expect("decode data");

                println!("rs: {:?}", rs);
                Self::process_event(rs).await;
            }
        })
    }

    fn drop_prefetched_messages(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async { println!("drop_prefetched_messages...") })
    }
}