use farcaster_client::client::get_all_fids;
use service::sea_orm::DbConn;

pub async fn run(db: &DbConn, max_fid: i32) -> anyhow::Result<()> {
    // let fids = get_all_fids(max_fid).await?;

    todo!()
}

fn make_latest_fid() {}
