use crate::mutation::Mutation;
use entity::storage;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DbConn, EntityTrait};
use std::vec;

impl Mutation {
    pub async fn insert_storage(db: &DbConn, stg: storage::ActiveModel) -> anyhow::Result<()> {
        let _ = storage::Entity::insert(stg)
            .on_conflict(
                OnConflict::columns(vec![storage::Column::Fid, storage::Column::ExpiresAt])
                    .update_columns(vec![
                        storage::Column::Units,
                        storage::Column::Payer,
                        storage::Column::ExpiresAt,
                        storage::Column::RentedAt,
                        storage::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
