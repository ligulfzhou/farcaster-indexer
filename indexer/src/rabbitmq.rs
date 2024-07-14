use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties, Consumer, Queue};

pub async fn get_mq_queue_channel() -> (Queue, Channel) {
    let addr = dotenv::var("AMQP_ADDR").expect("AMQP_ADDR not found");
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let conn = Connection::connect(&addr, options)
        .await
        .expect("rabbitmq connection error");

    let chan = conn.create_channel().await.expect("create channel failed");

    let queue = chan
        .queue_declare(
            "farcaster",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("declare queue..");

    (queue, chan)
}

pub async fn get_consumer() -> Consumer {
    let (queue, chan) = get_mq_queue_channel().await;

    chan.basic_consume(
        queue.name().as_str(),
        "consumer",
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await
    .expect("get consumer")
}
