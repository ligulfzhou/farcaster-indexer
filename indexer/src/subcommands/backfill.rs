use crate::rpc::client::get_all_fids;

pub async fn run(max_fid: i32) -> anyhow::Result<()> {
    let fids = get_all_fids(max_fid).await?;

    todo!()
}

fn make_latest_fid() {}
