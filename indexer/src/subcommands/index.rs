use crate::rabbitmq::{get_consumer, get_mq_queue_channel};
use bytes::Bytes;
use farcaster_client::grpc::{MessageType, OnChainEventType};
use farcaster_client::to_entity::verification_message_to_entity;
use farcaster_client::utils::farcaster_timestamp_to_datetime_with_tz;
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
                    let message = msg_body.message.expect("get message from MergeMessageBody");
                    let message_clone = message.clone();

                    let message_data = message.data.expect("message data");
                    let fid = message_data.fid as i64;
                    let timestamp =
                        farcaster_timestamp_to_datetime_with_tz(message_data.timestamp.into());
                    let message_type = MessageType::try_from(message_data.r#type)?;
                    let message_body = message_data.body.expect("message body should be there");

                    match message_type {
                        MessageType::CastAdd => {
                            if let Some(entity) = cast_message_to_entity(message_clone) {
                                service::mutation::Mutation::insert_cast(&db, entity).await?;
                            }
                        }
                        MessageType::CastRemove => {
                            if let MessageDataBody::CastRemoveBody(body) = message_body {
                                let hash = vec_u8_to_hex_string(&body.target_hash);
                                service::mutation::Mutation::delete_cast_by_hash(
                                    &db, &hash, timestamp,
                                )
                                .await?;
                            }
                        }
                        MessageType::ReactionAdd => {}
                        MessageType::ReactionRemove => {}
                        MessageType::LinkAdd => {}
                        MessageType::LinkRemove => {}
                        MessageType::VerificationAddEthAddress => {
                            if let Some(entity) = verification_message_to_entity(message_clone) {
                                service::mutation::Mutation::insert_verfications(&db, vec![entity])
                                    .await?;
                            }
                        }
                        MessageType::VerificationRemove => {}
                        MessageType::UserDataAdd => {}
                        MessageType::UsernameProof => {}
                        MessageType::FrameAction => {}
                        MessageType::LinkCompactState => {}
                        _ => {}
                    }
                }
            }
            HubEventType::PruneMessage => {
                if let EventBody::PruneMessageBody(msg_body) = event_body {
                    let message = msg_body.message.expect("get message from PruneMessageBody");
                    let message_clone = message.clone();

                    let message_data = message.data.expect("message data");
                    let fid = message_data.fid as i64;
                    let timestamp =
                        farcaster_timestamp_to_datetime_with_tz(message_data.timestamp.into());
                    let message_type = MessageType::try_from(message_data.r#type)?;
                    let message_body = message_data.body.expect("message body should be there");

                    match message_type {
                        MessageType::CastAdd => {}
                        MessageType::ReactionAdd => {}
                        MessageType::LinkAdd => {}
                        _ => {}
                    }
                }
            }
            HubEventType::RevokeMessage => {
                // Events are emitted when a signer that was used to create a message is removed
                // TODO: handle revoking messages
            }
            HubEventType::MergeOnChainEvent => {
                if let EventBody::MergeOnChainEventBody(body) = event_body {
                    let on_chain_event = body.on_chain_event.expect("get on-chain-event");
                    let event_type = OnChainEventType::try_from(on_chain_event.r#type)?;

                    match event_type {
                        OnChainEventType::EventTypeSigner => {}
                        OnChainEventType::EventTypeSignerMigrated => {}
                        OnChainEventType::EventTypeIdRegister => {}
                        OnChainEventType::EventTypeStorageRent => {}
                        _ => {}
                    }
                }
            }
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
                Self::process_event(rs).await.expect("process event");

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
