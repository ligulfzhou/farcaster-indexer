use farcaster_client::client::Client;
use farcaster_client::to_entity::{
    cast_message_to_entity, link_message_to_entity, reaction_message_to_entity,
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

        for entity in casts_entities {
            service::mutation::Mutation::insert_cast(db, entity).await?;
        }
    }

    Ok(())
}
