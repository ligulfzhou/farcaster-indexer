use crate::mutation::Mutation;
use entity::verifications;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DbConn, EntityTrait};

impl Mutation {
    pub async fn insert_verfications(
        db: &DbConn,
        data: Vec<verifications::ActiveModel>,
    ) -> anyhow::Result<()> {
        let _ = verifications::Entity::insert_many(data)
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .exec(db)
            .await?;

        Ok(())
    }
}
