use crate::mutation::Mutation;
use entity::user_data;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DbConn, EntityTrait};

impl Mutation {
    pub async fn insert_user_data(
        db: &DbConn,
        data: Vec<user_data::ActiveModel>,
    ) -> anyhow::Result<()> {
        let _ = user_data::Entity::insert_many(data)
            .on_conflict(
                OnConflict::columns(vec![user_data::Column::Fid, user_data::Column::Type])
                    .update_columns(vec![user_data::Column::Hash, user_data::Column::Value])
                    .to_owned(),
            )
            .on_conflict(
                OnConflict::column(user_data::Column::Hash)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
