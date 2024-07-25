use crate::grpc::cast_add_body::Parent;
use crate::grpc::reaction_body::Target;
pub use crate::grpc::{message_data::Body, Message, MessageData};
use crate::utils::farcaster_timestamp_to_datetime_with_tz;
use entity::sea_orm::ActiveValue::Set;
use serde_json::json;
use std::string::String;

pub fn cast_message_to_entity(message: Message) -> Option<entity::casts::ActiveModel> {
    let mut active_model = entity::casts::ActiveModel::default();
    if let Ok(s) = String::from_utf8(message.hash) {
        active_model.hash = Set(s);
    }

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(message_body) = message_data.body {
            if let Body::CastAddBody(cast_add_body) = message_body {
                if let Some(parent) = cast_add_body.parent {
                    match parent {
                        Parent::ParentCastId(parent_cast_id) => {
                            active_model.parent_fid = Set(Some(parent_cast_id.fid as i64));
                        }
                        Parent::ParentUrl(parenturl) => {
                            active_model.parent_url = Set(Some(parenturl))
                        }
                    }
                }

                active_model.text = Set(cast_add_body.text);
                active_model.embeds = Set(json!(cast_add_body.embeds));
                active_model.mentions = Set(json!(cast_add_body.mentions));
                active_model.mentions_positions = Set(json!(cast_add_body.mentions_positions));
            }
        }
    }

    Some(active_model)
}

pub fn reaction_message_to_entity(message: Message) -> Option<entity::reactions::ActiveModel> {
    let mut active_model = entity::reactions::ActiveModel::default();
    if let Ok(s) = String::from_utf8(message.hash) {
        active_model.hash = Set(s);
    }

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(message_body) = message_data.body {
            if let Body::ReactionBody(body) = message_body {
                if let Some(target) = body.target {
                    match target {
                        Target::TargetCastId(target_cast_id) => {
                            active_model.target_cast_fid = Set(Some(target_cast_id.fid as i64));
                            active_model.target_cast_hash = Set(Some(
                                String::from_utf8(target_cast_id.hash).unwrap_or("".to_string()),
                            ));
                        }
                        Target::TargetUrl(target_url) => {
                            active_model.target_url = Set(Some(target_url));
                        }
                    }
                }

                active_model.r#type = Set(body.r#type);
            }
        }
    }

    Some(active_model)
}
