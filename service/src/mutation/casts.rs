use crate::mutation::Mutation;
use entity::prelude;
use entity::prelude::Casts;
use sea_orm::sea_query::any;
use sea_orm::{DbConn, EntityTrait};

impl Mutation {
    pub async fn delete_cast(db: &DbConn, id: i32) -> anyhow::Result<()> {
        let cast = Casts::find_by_id(id).one(db).await?;

        todo!()
    }
}
