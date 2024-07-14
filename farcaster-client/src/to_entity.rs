use crate::grpc::CastAddBody;

// event message_body to entity
impl Into<entity::casts::Model> for CastAddBody {
    fn into(self) -> entity::casts::Model {
        entity::casts::Model {
            id: Default::default(),
            fid: Default::default(),
            parent_id: Default::default(),
            hash: Default::default(),
            root_parent_hash: Default::default(),
            parent_hash: Default::default(),
            root_parent_url: Default::default(),
            parent_url: Default::default(),
            text: Default::default(),
            embeds: Default::default(),
            mentions: Default::default(),
            mentions_positions: Default::default(),
            create_at: Default::default(),
            updated_at: Default::default(),
            timestamp: Default::default(),
            deleted_at: Default::default(),
            pruned_at: Default::default(),
        }
    }
}
