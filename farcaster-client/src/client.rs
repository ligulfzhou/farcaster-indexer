use crate::grpc::{
    hub_service_client::HubServiceClient, FidsRequest, HubEvent, HubEventType, SubscribeRequest,
};
use lapin::options::BasicPublishOptions;
use lapin::{BasicProperties, Channel, Queue};
use prost::Message;
use tokio::sync::mpsc::Sender;
use tonic::codegen::tokio_stream::StreamExt;

pub struct Client(pub HubServiceClient<tonic::transport::Channel>);

impl Client {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let client = HubServiceClient::connect(addr.clone()).await?;
        Ok(Self(client))
    }
}

impl Client {
    pub async fn subscribe_to_mq(
        &mut self,
        start_event_id: u64,
        queue: Queue,
        chan: Channel,
    ) -> anyhow::Result<()> {
        self.subscribe(start_event_id, Some(queue), Some(chan), None)
            .await
    }

    pub async fn subscribe_to_channel(
        &mut self,
        start_event_id: u64,
        tx: Sender<HubEvent>,
    ) -> anyhow::Result<()> {
        self.subscribe(start_event_id, None, None, Some(tx)).await
    }

    async fn subscribe(
        &mut self,
        start_event_id: u64,
        queue: Option<Queue>,         // subscribe to mq
        chan: Option<Channel>,        // subscribe to mq
        tx: Option<Sender<HubEvent>>, // subscribe to channel
    ) -> anyhow::Result<()> {
        let response = self
            .0
            .subscribe(SubscribeRequest {
                event_types: vec![
                    HubEventType::MergeMessage as i32,
                    HubEventType::PruneMessage as i32,
                    HubEventType::RevokeMessage as i32,
                    HubEventType::MergeOnChainEvent as i32,
                ],
                from_id: Some(start_event_id),
                total_shards: None,
                shard_index: None,
            })
            .await?;

        let mut stream = response.into_inner();

        // subscribe to mq
        if let (Some(queue), Some(chan)) = (queue, chan) {
            while let Some(Ok(event)) = stream.next().await {
                let encoded = event.encode_to_vec();
                chan.basic_publish(
                    "",
                    queue.name().as_str(),
                    BasicPublishOptions::default(),
                    &encoded,
                    BasicProperties::default(),
                )
                .await
                .expect("publish data to mq");
            }
        }

        if let Some(tx) = tx {
            while let Some(Ok(event)) = stream.next().await {
                println!("\treceived message: `{:?}`", event);

                tx.send(event).await.expect("send data to channel");
            }
        }

        Ok(())
    }
}

impl Client {
    pub async fn get_max_fid(&mut self) -> anyhow::Result<u64> {
        let max_fid_res = self
            .0
            .get_fids(FidsRequest {
                page_size: Some(1),
                page_token: None,
                reverse: Some(true),
            })
            .await?;

        Ok(max_fid_res.into_inner().fids[0])
    }
}

pub async fn get_all_fids(client: &mut Client, max_id: i32) -> anyhow::Result<Vec<u64>> {
    let max_fid = client.get_max_fid().await?;

    todo!()
}

#[cfg(test)]
mod tests {
    use crate::client::Client;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let client = Client::new("".to_string()).await?;

        todo!()
    }
}
