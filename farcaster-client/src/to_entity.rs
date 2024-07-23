pub use crate::grpc::{message_data::Body as MessageBody, Message, MessageData};
use entity::sea_orm::{ActiveValue, Set};

pub fn cast_add_message_data_to_casts_entity(
    message: Message,
) -> Option<entity::casts::ActiveModel> {
    let mut active_model = entity::casts::ActiveModel::default();
    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        // active_model.timestamp =
        if let Some(message_body) = message_data.body {
            if let MessageBody::CastAddBody(cast_add_body) = message_body {
                // active_model.create_at
            }
        }
    }

    let active_model = entity::casts::ActiveModel {
        id: ActiveValue::Set(0),
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
    };
    Some(active_model)
}
