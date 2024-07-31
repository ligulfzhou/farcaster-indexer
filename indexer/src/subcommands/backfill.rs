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
use tracing::info;

const CHUNK_SIZE: usize = 1000;

pub async fn run(db: &DbConn, mut hub_client: Client, max_fid: i32) -> anyhow::Result<()> {
    let client_clone = hub_client.clone();
    let max_fid_to_iterate = match max_fid {
        0 => hub_client.get_max_fid().await?,
        _ => max_fid as u64,
    };

    for fid in 2163..=max_fid_to_iterate {
        backfill_fid(db, client_clone.clone(), fid).await?;
    }

    Ok(())
}

pub async fn backfill_fid(db: &DbConn, mut hub_client: Client, fid: u64) -> anyhow::Result<()> {
    info!("..............process fid#{:}................", fid);
    let casts_entities = hub_client
        .get_all_casts_by_fid(fid)
        .await?
        .into_iter()
        .filter_map(cast_message_to_entity)
        .collect::<Vec<_>>();

    let reactions_entities = hub_client
        .get_all_reactions_by_fid(fid)
        .await?
        .into_iter()
        .filter_map(reaction_message_to_entity)
        .collect::<Vec<_>>();

    let links_entities = hub_client
        .get_all_links_by_fid(fid)
        .await?
        .into_iter()
        .filter_map(link_message_to_entity)
        .collect::<Vec<_>>();

    let user_data = hub_client.get_user_data_by_fid(fid).await?;
    let user_data_entities = user_data_messages_to_entity(user_data);

    let verifications = hub_client
        .get_user_verification_by_fid(fid)
        .await?
        .into_iter()
        .filter_map(verification_message_to_entity)
        .collect::<Vec<_>>();
    let registrations = hub_client.get_all_registration_by_fid(fid).await?;
    let signers = hub_client.get_all_signers_by_fid(fid).await?;
    let storages = hub_client
        .get_all_storage_by_fid(fid)
        .await?
        .into_iter()
        .filter_map(storage_message_to_entity)
        .collect::<Vec<_>>();

    info!("casts: {:}", &casts_entities.len());
    for chunk in casts_entities.chunks(CHUNK_SIZE) {
        service::mutation::Mutation::insert_casts(db, chunk.to_vec()).await?;
    }
    info!("reaction: {:}", &reactions_entities.len());
    for chunk in reactions_entities.chunks(CHUNK_SIZE) {
        service::mutation::Mutation::insert_reactions(db, chunk.to_vec()).await?;
    }
    info!("links: {:}", &links_entities.len());
    for chunk in links_entities.chunks(CHUNK_SIZE) {
        service::mutation::Mutation::insert_links(db, chunk.to_vec()).await?;
    }
    info!("user_data: {:}", &user_data_entities.len());
    for chunk in user_data_entities.chunks(CHUNK_SIZE) {
        service::mutation::Mutation::insert_user_data(db, chunk.to_vec()).await?;
    }
    info!("verification: {:}", &verifications.len());
    for chunk in verifications.chunks(CHUNK_SIZE) {
        service::mutation::Mutation::insert_verfications(db, chunk.to_vec()).await?;
    }
    info!("registrations: {:}", &registrations.len());
    for registration in registrations {
        if let Some(entity) = registration_message_to_entity(registration.clone()) {
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
    }

    info!("signers: {:}", &signers.len());
    for signer in signers {
        if let Some(entity) = signer_message_to_entity(signer.clone()) {
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
    }

    info!("storages: {:}", &storages.len());
    for storage in storages {
        service::mutation::Mutation::insert_storage(db, storage).await?;
    }

    Ok(())
}
