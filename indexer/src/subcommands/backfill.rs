use farcaster_client::client::Client;
use farcaster_client::to_entity::cast_message_to_entity;
use service::sea_orm::DbConn;

pub async fn run(db: &DbConn, mut hub_client: Client, max_fid: i32) -> anyhow::Result<()> {
    let max_fid_to_iterate = match max_fid {
        0 => hub_client.get_max_fid().await?,
        _ => max_fid as u64,
    };

    for fid in 1..=max_fid_to_iterate {
        let casts = hub_client.get_all_casts_by_fid(fid).await?;
        println!("{:?}", casts);

        let entities = casts
            .into_iter()
            .filter_map(|cast| cast_message_to_entity(cast))
            .collect::<Vec<entity::casts::ActiveModel>>();

        println!("{:?}", entities);

        service::mutation::Mutation::insert_casts(db, entities).await?;
    }

    // todo!()
    Ok(())
}
