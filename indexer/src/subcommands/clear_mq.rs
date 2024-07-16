use crate::rabbitmq::get_consumer;
use bytes::Bytes;
use farcaster_client::grpc::HubEvent;
use lapin::message::DeliveryResult;
use lapin::options::BasicAckOptions;
use lapin::ConsumerDelegate;
use prost::Message;
use std::future::Future;
use std::pin::Pin;

pub async fn run() {
    let (conn, consumer) = get_consumer().await;
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
                let deliveried_clone = deliveried.clone();
                let data = deliveried.data;
                let buf = Bytes::from(data);
                let rs = HubEvent::decode(buf).expect("decode data");

                println!("rs: {:?}", rs);

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
