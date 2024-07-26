use farcaster_client::client::Client;
use farcaster_client::to_entity::{
    cast_message_to_entity, link_message_to_entity, reaction_message_to_entity,
    user_data_messages_to_entity,
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

        let registration = hub_client.get_all_registration_by_fid(fid).await?;

        let signers = hub_client.get_all_signers_by_fid(fid).await?;

        let storage = hub_client.get_all_storage_by_fid(fid).await?;

        for entity in casts_entities {
            service::mutation::Mutation::insert_cast(db, entity).await?;
        }
        service::mutation::Mutation::insert_reactions(db, reactions_entities).await?;
        service::mutation::Mutation::insert_links(db, links_entities).await?;
        service::mutation::Mutation::insert_user_data(db, user_data_entities).await?;
    }

    Ok(())
}
