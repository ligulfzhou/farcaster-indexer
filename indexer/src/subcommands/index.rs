use crate::rabbitmq::{get_consumer, get_mq_queue_channel};
use bytes::Bytes;
use farcaster_client::{
    client::Client,
    grpc::{
        hub_event::Body as EventBody, message_data::Body as MessageDataBody, HubEvent, HubEventType,
    },
    to_entity::cast_message_to_entity,
    utils::vec_u8_to_hex_string,
};
use lapin::{message::DeliveryResult, options::BasicAckOptions, ConsumerDelegate};
use prost::Message;
use service::sea_orm::{Database, DbConn};
use std::future::Future;
use std::pin::Pin;

pub async fn run(db: &DbConn, mut hub_client: Client) -> anyhow::Result<()> {
    let (_, queue, chan) = get_mq_queue_channel().await;
    tokio::spawn(async move {
        hub_client
            .subscribe_to_mq(0, queue, chan)
            .await
            .expect("subscribe to farcaster node with MQ");
    });

    let (conn, consumer) = get_consumer().await;
    let delegate = Delegate;
    consumer.set_delegate(delegate);
    conn.run().expect("consume message forever");
    Ok(())
}

struct Delegate;

impl Delegate {
    pub async fn process_event(event: HubEvent) -> anyhow::Result<()> {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found.");
        let db = Database::connect(database_url)
            .await
            .expect("database connection failed.");

        let event_type = event.r#type();
        let event_body = event.body.unwrap();

        match event_type {
            HubEventType::MergeMessage => {
                if let EventBody::MergeMessageBody(msg_body) = event_body {
                    if let Some(message) = msg_body.message {
                        let message_clone = message.clone();
                        if let Some(message_data) = message.data {
                            if let Some(message_body) = message_data.body {
                                match message_body {
                                    MessageDataBody::CastAddBody(_body) => {
                                        if let Some(entity) = cast_message_to_entity(message_clone)
                                        {
                                            service::mutation::Mutation::insert_cast(&db, entity)
                                                .await?;
                                        }
                                    }
                                    MessageDataBody::CastRemoveBody(crb) => {
                                        println!("crb: {:?}", crb);
                                        let hash = vec_u8_to_hex_string(&crb.target_hash);
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

        todo!()
    }
}

impl ConsumerDelegate for Delegate {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Ok(Some(deliveried)) = delivery {
                let deliveried_clone = deliveried.clone();

                let data = deliveried.data;
                let buf = Bytes::from(data);
                let rs = HubEvent::decode(buf).expect("decode data");

                println!("rs: {:?}", rs);
                // println!("db: {:?}", self.db);
                Self::process_event(rs).await;

                deliveried_clone
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic ack");
            }
        })
    }

    fn drop_prefetched_messages(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async { println!("drop_prefetched_messages...") })
    }
}
