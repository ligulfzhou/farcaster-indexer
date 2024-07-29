use crate::grpc::{
    hub_service_client::HubServiceClient, FidRequest, FidsRequest, HubEvent, HubEventType,
    LinksByFidRequest, Message, OnChainEvent, OnChainEventRequest, OnChainEventType,
    ReactionsByFidRequest, SubscribeRequest,
};
use crate::MAX_PAGE_SIZE;
use lapin::{options::BasicPublishOptions, BasicProperties, Channel, Queue};
use prost::Message as ProstMessage;
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

    pub async fn get_all_fids(&mut self, max_id: i32) -> anyhow::Result<Vec<u64>> {
        let max_fid = self.get_max_fid().await?;
        dbg!(max_id);

        Ok((1..=max_fid).collect())
    }
}

impl Client {
    pub async fn get_user_data_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<Message>> {
        let mut page_token = None;
        let mut all_messages: Vec<Message> = vec![];

        loop {
            let res = self
                .0
                .get_user_data_by_fid(FidRequest {
                    fid,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.messages.clone());
            if message_response.messages.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }
        Ok(all_messages)
    }

    pub async fn get_user_verification_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<Message>> {
        let mut page_token = None;
        let mut all_messages: Vec<Message> = vec![];
        loop {
            let res = self
                .0
                .get_verifications_by_fid(FidRequest {
                    fid,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.messages.clone());
            if message_response.messages.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }
        Ok(all_messages)
    }

    pub async fn get_all_casts_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<Message>> {
        let mut page_token = None;
        let mut all_messages: Vec<Message> = vec![];
        loop {
            let res = self
                .0
                .get_casts_by_fid(FidRequest {
                    fid,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.messages.clone());
            if message_response.messages.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        Ok(all_messages)
    }

    pub async fn get_all_reactions_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<Message>> {
        let mut page_token = None;
        let mut all_messages: Vec<Message> = vec![];
        loop {
            let res = self
                .0
                .get_reactions_by_fid(ReactionsByFidRequest {
                    fid,
                    reaction_type: None,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.messages.clone());
            if message_response.messages.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        Ok(all_messages)
    }

    pub async fn get_all_links_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<Message>> {
        let mut page_token = None;
        let mut all_messages: Vec<Message> = vec![];
        loop {
            let res = self
                .0
                .get_links_by_fid(LinksByFidRequest {
                    fid,
                    link_type: None,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.messages.clone());
            if message_response.messages.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        Ok(all_messages)
    }

    pub async fn get_all_registration_by_fid(
        &mut self,
        fid: u64,
    ) -> anyhow::Result<Vec<OnChainEvent>> {
        let mut page_token = None;
        let mut all_messages: Vec<OnChainEvent> = vec![];

        loop {
            let res = self
                .0
                .get_on_chain_events(OnChainEventRequest {
                    fid,
                    event_type: OnChainEventType::EventTypeIdRegister as i32,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.events.clone());
            if message_response.events.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        all_messages.sort_unstable_by_key(|msg| (msg.block_number, msg.log_index));
        Ok(all_messages)
    }

    pub async fn get_all_signers_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<OnChainEvent>> {
        let mut page_token = None;
        let mut all_messages: Vec<OnChainEvent> = vec![];

        loop {
            let res = self
                .0
                .get_on_chain_events(OnChainEventRequest {
                    fid,
                    event_type: OnChainEventType::EventTypeSigner as i32,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.events.clone());
            if message_response.events.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        all_messages.sort_unstable_by_key(|msg| (msg.block_number, msg.log_index));
        Ok(all_messages)
    }

    pub async fn get_all_storage_by_fid(&mut self, fid: u64) -> anyhow::Result<Vec<OnChainEvent>> {
        let mut page_token = None;
        let mut all_messages: Vec<OnChainEvent> = vec![];

        loop {
            let res = self
                .0
                .get_on_chain_events(OnChainEventRequest {
                    fid,
                    event_type: OnChainEventType::EventTypeStorageRent as i32,
                    page_size: Some(MAX_PAGE_SIZE),
                    page_token,
                    reverse: None,
                })
                .await?;

            let message_response = res.into_inner();
            page_token = message_response.next_page_token;

            all_messages.extend(message_response.events.clone());
            if message_response.events.len() < MAX_PAGE_SIZE as usize {
                break;
            }
        }

        all_messages.sort_unstable_by_key(|msg| (msg.block_number, msg.log_index));
        Ok(all_messages)
    }
}
