use crate::{
    constants::{FARCASTER_QUEUE, FID_BACKFILL_QUEUE},
    rabbitmq::get_consumer,
};
use bytes::Bytes;
use farcaster_client::grpc::HubEvent;
use lapin::{message::DeliveryResult, options::BasicAckOptions, ConsumerDelegate};
use prost::Message;
use std::{future::Future, pin::Pin};

pub async fn run() {
    let (conn, consumer) = get_consumer(FID_BACKFILL_QUEUE).await;
    consumer.set_delegate(Delegate);
    conn.run().expect("run consumer forever");

    let (conn, consumer) = get_consumer(FARCASTER_QUEUE).await;
    consumer.set_delegate(Delegate);
    conn.run().expect("run consumer forever");
}

struct Delegate;

impl ConsumerDelegate for Delegate {
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Ok(Some(deliveried)) = delivery {
                deliveried
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
