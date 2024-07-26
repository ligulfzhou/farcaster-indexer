use crate::grpc::cast_add_body::Parent;
use crate::grpc::embed::Embed as InnerEmbed;
use crate::grpc::reaction_body::Target;
use crate::grpc::{link_body, Embed};
pub use crate::grpc::{message_data::Body, Message, MessageData};
use crate::utils::{farcaster_timestamp_to_datetime_with_tz, vec_u8_to_hex_string};
use entity::sea_orm::ActiveValue::Set;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::string::String;

fn format_embeds(embeds: Vec<Embed>) -> Value {
    let value_array = embeds
        .into_iter()
        .filter_map(|embed| {
            if let Some(inner) = embed.embed {
                match inner {
                    InnerEmbed::Url(url) => Some(json!({
                        "url": url
                    })),
                    InnerEmbed::CastId(cast_id) => Some(json!({
                        "fid": cast_id.fid,
                        "hash": vec_u8_to_hex_string(&cast_id.hash)
                    })),
                }
            } else {
                None
            }
        })
        .collect::<Vec<Value>>();

    Value::Array(value_array)
}
pub fn cast_message_to_entity(message: Message) -> entity::casts::ActiveModel {
    let mut active_model = entity::casts::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::CastAddBody(cast_add_body)) = message_data.body {
            if let Some(parent) = cast_add_body.parent {
                match parent {
                    Parent::ParentCastId(parent_cast_id) => {
                        active_model.parent_fid = Set(Some(parent_cast_id.fid as i64));
                        active_model.parent_hash =
                            Set(Some(vec_u8_to_hex_string(&parent_cast_id.hash)));
                    }
                    Parent::ParentUrl(parenturl) => active_model.parent_url = Set(Some(parenturl)),
                }
            }

            active_model.text = Set(cast_add_body.text);
            if !cast_add_body.embeds.is_empty() {
                active_model.embeds = Set(format_embeds(cast_add_body.embeds));
            }
            if !cast_add_body.mentions.is_empty() {
                active_model.mentions = Set(json!(cast_add_body.mentions));
            }
            if !cast_add_body.mentions_positions.is_empty() {
                active_model.mentions_positions = Set(json!(cast_add_body.mentions_positions));
            }
        }
    }

    active_model
}

pub fn reaction_message_to_entity(message: Message) -> entity::reactions::ActiveModel {
    let mut active_model = entity::reactions::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::ReactionBody(body)) = message_data.body {
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

    active_model
}

pub fn link_message_to_entity(message: Message) -> entity::links::ActiveModel {
    let mut active_model = entity::links::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::LinkBody(body)) = message_data.body {
            if let Some(link_body::Target::TargetFid(target_fid)) = body.target {
                active_model.target_fid = Set(target_fid as i64);
            }
            if let Some(ts) = body.display_timestamp {
                active_model.display_timestamp =
                    Set(Some(farcaster_timestamp_to_datetime_with_tz(ts)));
            }
        }
    }

    active_model
}

pub fn user_data_message_to_entity(message: Message) -> entity::user_data::ActiveModel {
    let mut active_model = entity::user_data::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::UserDataBody(body)) = message_data.body {
            active_model.r#type = Set(body.r#type);
            active_model.value = Set(body.value);
        }
    }

    active_model
}

fn get_user_data_type(message: Message) -> i32 {
    if let Some(message_data) = message.data {
        if let Some(Body::UserDataBody(body)) = message_data.body {
            return body.r#type;
        }
    }

    0
}

// also need to deduplicate
pub fn user_data_messages_to_entity(messages: Vec<Message>) -> Vec<entity::user_data::ActiveModel> {
    let mut type_to_message = HashMap::new();
    for msg in messages {
        type_to_message.insert(get_user_data_type(msg.clone()), msg);
    }

    type_to_message
        .into_values()
        .map(user_data_message_to_entity)
        .collect::<Vec<_>>()
}

pub fn verification_message_to_entity(message: Message) -> entity::verifications::ActiveModel {
    let mut active_model = entity::verifications::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::VerificationAddAddressBody(body)) = message_data.body {
            active_model.signature = Set(vec_u8_to_hex_string(&body.claim_signature));
            active_model.block_hash = Set(vec_u8_to_hex_string(&body.block_hash));
            active_model.signer_address = Set(vec_u8_to_hex_string(&body.address));
        }
    }

    active_model
}
