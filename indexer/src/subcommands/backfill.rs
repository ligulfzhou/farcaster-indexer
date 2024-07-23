use farcaster_client::client::Client;
use service::sea_orm::DbConn;

pub async fn run(db: &DbConn, mut hub_client: Client, max_fid: i32) -> anyhow::Result<()> {
    let max_fid_to_iterate = match max_fid {
        0 => hub_client.get_max_fid().await?,
        _ => max_fid as u64,
    };

    (1..=max_fid_to_iterate).for_each(|fid| {
        hub_client.get_all_casts_by_fid(fid);
    });

    todo!()
}

fn make_latest_fid() {}
