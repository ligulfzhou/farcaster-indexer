use farcaster_client::grpc::SignerEventType;
use farcaster_client::to_entity::{
    cast_message_to_entity, link_message_to_entity, reaction_message_to_entity,
    registration_message_to_entity, signer_message_to_entity, storage_message_to_entity,
    user_data_messages_to_entity, verification_message_to_entity,
};
use farcaster_client::{
    client::Client,
    grpc::{on_chain_event::Body as OnChainEventBody, IdRegisterEventType},
};
use service::sea_orm::DbConn;

pub async fn run(db: &DbConn, mut hub_client: Client, max_fid: i32) -> anyhow::Result<()> {
    let max_fid_to_iterate = match max_fid {
        0 => hub_client.get_max_fid().await?,
        _ => max_fid as u64,
    };

    for fid in 1..=max_fid_to_iterate {
        let casts_entities = hub_client
            .get_all_casts_by_fid(fid)
            .await?
            .into_iter()
            .map(cast_message_to_entity)
            .collect::<Vec<_>>();
        dbg!(&casts_entities);

        let reactions_entities = hub_client
            .get_all_reactions_by_fid(fid)
            .await?
            .into_iter()
            .map(reaction_message_to_entity)
            .collect::<Vec<_>>();
        dbg!(&reactions_entities);

        let links_entities = hub_client
            .get_all_links_by_fid(fid)
            .await?
            .into_iter()
            .map(link_message_to_entity)
            .collect::<Vec<_>>();
        dbg!(&links_entities);

        let user_data = hub_client.get_user_data_by_fid(fid).await?;
        let user_data_entities = user_data_messages_to_entity(user_data);

        let verifications = hub_client
            .get_user_verification_by_fid(fid)
            .await?
            .into_iter()
            .map(verification_message_to_entity)
            .collect::<Vec<_>>();
        let registrations = hub_client.get_all_registration_by_fid(fid).await?;
        let signers = hub_client.get_all_signers_by_fid(fid).await?;
        let storages = hub_client
            .get_all_storage_by_fid(fid)
            .await?
            .into_iter()
            .map(storage_message_to_entity)
            .collect::<Vec<_>>();

        for entity in casts_entities {
            service::mutation::Mutation::insert_cast(db, entity).await?;
        }
        service::mutation::Mutation::insert_reactions(db, reactions_entities).await?;
        service::mutation::Mutation::insert_links(db, links_entities).await?;
        service::mutation::Mutation::insert_user_data(db, user_data_entities).await?;
        service::mutation::Mutation::insert_verfications(db, verifications).await?;

        for registration in registrations {
            let entity = registration_message_to_entity(registration.clone());
            if let Some(OnChainEventBody::IdRegisterEventBody(registration_body)) =
                registration.body
            {
                if let Ok(event_type) = IdRegisterEventType::try_from(registration_body.event_type)
                {
                    match event_type {
                        IdRegisterEventType::None => {}
                        IdRegisterEventType::Register => {
                            service::mutation::Mutation::insert_fid(db, entity).await?;
                        }
                        IdRegisterEventType::Transfer => {
                            service::mutation::Mutation::transfer(db, entity).await?;
                        }
                        IdRegisterEventType::ChangeRecovery => {
                            service::mutation::Mutation::change_recovery(db, entity).await?;
                        }
                    }
                }
            }
        }

        for signer in signers {
            let entity = signer_message_to_entity(signer.clone());
            if let Some(OnChainEventBody::SignerEventBody(signer_body)) = signer.body {
                if let Ok(event_type) = SignerEventType::try_from(signer_body.event_type) {
                    match event_type {
                        SignerEventType::Add => {
                            service::mutation::Mutation::insert_signer(db, entity).await?;
                        }
                        SignerEventType::Remove => {
                            service::mutation::Mutation::remove_signer(db, entity).await?;
                        }
                        _ => {}
                    }
                }
            }
        }
        for storage in storages {
            service::mutation::Mutation::insert_storage(db, storage).await?;
        }
    }

    Ok(())
}
