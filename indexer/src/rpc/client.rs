use tokio::sync::mpsc::Sender;
use tonic::codegen::tokio_stream::StreamExt;
use crate::rpc::farcaster_grpc::{hub_service_client::HubServiceClient, SubscribeRequest, HubEventType, FidsRequest, HubEvent};

pub struct Client {
    addr: String,
    pub client: HubServiceClient<tonic::transport::Channel>,
}


impl Client {
    pub async fn new(addr: String) -> anyhow::Result<Self> {
        let mut client = HubServiceClient::connect(addr.clone()).await?;
        Ok(Self {
            addr,
            client,
        })
    }
}

impl Client {
    pub async fn subscribe(&mut self, start_event_id: u64, tx: Sender<HubEvent>) -> anyhow::Result<()> {
        let response = self.client.subscribe(SubscribeRequest {
            event_types: vec![
                HubEventType::MergeMessage as i32,
                HubEventType::PruneMessage as i32,
                HubEventType::RevokeMessage as i32,
                HubEventType::MergeOnChainEvent as i32,
            ],
            from_id: Some(start_event_id),
            total_shards: None,
            shard_index: None,
        }).await?;

        let mut stream = response.into_inner();

        while let Some(received) = stream.next().await {
            println!("\treceived message: `{:?}`", received);

            let event = received.unwrap();
            println!("event: {:?}", event.body);
            tx.send(event).await?;
            // if let Some(body) = event.body {
            //     match body {
            //         Body::MergeMessageBody(message) => {}
            //         Body::PruneMessageBody(message) => {}
            //         Body::RevokeMessageBody(message) => {}
            //         Body::MergeUsernameProofBody(message) => {}
            //         Body::MergeOnChainEventBody(message) => {}
            //     }
            // }
        }

        Ok(())
    }
}

impl Client {
    pub async fn get_max_fid(&mut self) -> anyhow::Result<u64> {
        let max_fid_res = self.client.get_fids(FidsRequest {
            page_size: Some(1),
            page_token: None,
            reverse: Some(true),
        }).await?;

        Ok(max_fid_res.into_inner().fids[0])
    }
}


pub async  fn get_all_fids(client: &mut Client)-> anyhow::Result<Vec<u64>> {
    let max_fid = client.get_max_fid().await?;

    todo!()
}
