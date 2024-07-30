use crate::grpc::cast_add_body::Parent;
use crate::grpc::embed::Embed as InnerEmbed;
use crate::grpc::on_chain_event::Body as OnChainEventBody;
use crate::grpc::reaction_body::Target;
use crate::grpc::{link_body, Embed, OnChainEvent, SignerEventType};
pub use crate::grpc::{message_data::Body, Message, MessageData};
use crate::utils::{farcaster_timestamp_to_datetime_with_tz, vec_u8_to_hex_string};
use chrono::Utc;
use entity::sea_orm::ActiveValue::Set;
use ethereum_abi::{Param as EthParam, Type, Value as EthValue};
use serde_json::json;
use std::collections::HashMap;
use std::string::String;

fn format_embeds(embeds: Vec<Embed>) -> Vec<String> {
    let value_array = embeds
        .into_iter()
        .filter_map(|embed| {
            if let Some(inner) = embed.embed {
                match inner {
                    InnerEmbed::Url(url) => Some(
                        json!({
                            "url": url,
                        })
                        .to_string(),
                    ),
                    InnerEmbed::CastId(cast_id) => Some(
                        json!({
                            "fid": cast_id.fid,
                            "hash": vec_u8_to_hex_string(&cast_id.hash)
                        })
                        .to_string(),
                    ),
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    value_array
}
pub fn cast_message_to_entity(message: Message) -> Option<entity::casts::ActiveModel> {
    let mut active_model = entity::casts::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        active_model.parent_fid = Set(None);
        active_model.parent_hash = Set(None);
        active_model.parent_url = Set(None);

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
            active_model.embeds = Set(format_embeds(cast_add_body.embeds));
            active_model.mentions = Set(cast_add_body
                .mentions
                .into_iter()
                .map(|i| i as i32)
                .collect());
            active_model.mentions_positions = Set(cast_add_body
                .mentions_positions
                .into_iter()
                .map(|i| i as i32)
                .collect());
        }
    } else {
        return None;
    }

    Some(active_model)
}

pub fn reaction_message_to_entity(message: Message) -> Option<entity::reactions::ActiveModel> {
    let mut active_model = entity::reactions::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        target_cast_fid: Set(None),
        target_cast_hash: Set(None),
        target_url: Set(None),
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
    } else {
        return None;
    }

    Some(active_model)
}

pub fn link_message_to_entity(message: Message) -> Option<entity::links::ActiveModel> {
    let mut active_model = entity::links::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        r#type: Set("".to_string()),
        target_fid: Set(0i64),
        display_timestamp: Set(None),
        ..Default::default()
    };

    if let Some(message_data) = message.data {
        active_model.fid = Set(message_data.fid as i64);
        active_model.timestamp = Set(farcaster_timestamp_to_datetime_with_tz(
            message_data.timestamp,
        ));
        if let Some(Body::LinkBody(body)) = message_data.body {
            active_model.r#type = Set(body.r#type);
            if let Some(link_body::Target::TargetFid(target_fid)) = body.target {
                active_model.target_fid = Set(target_fid as i64);
            }
            if let Some(ts) = body.display_timestamp {
                active_model.display_timestamp =
                    Set(Some(farcaster_timestamp_to_datetime_with_tz(ts)));
            }
        }
    } else {
        return None;
    }

    Some(active_model)
}

pub fn user_data_message_to_entity(message: Message) -> Option<entity::user_data::ActiveModel> {
    let mut active_model = entity::user_data::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        r#type: Set(0i32),
        value: Set("".to_string()),
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
    } else {
        return None;
    }

    Some(active_model)
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
        .filter_map(user_data_message_to_entity)
        .collect::<Vec<_>>()
}

pub fn verification_message_to_entity(
    message: Message,
) -> Option<entity::verifications::ActiveModel> {
    let mut active_model = entity::verifications::ActiveModel {
        hash: Set(vec_u8_to_hex_string(&message.hash)),
        signature: Set("".to_string()),
        block_hash: Set("".to_string()),
        signer_address: Set("".to_string()),
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
    } else {
        return None;
    }

    Some(active_model)
}

pub fn registration_message_to_entity(event: OnChainEvent) -> Option<entity::fids::ActiveModel> {
    let mut active_model = entity::fids::ActiveModel {
        fid: Set(event.fid as i64),
        custody_address: Set("".to_string()),
        recovery_address: Set("".to_string()),
        register_at: Set(farcaster_timestamp_to_datetime_with_tz(
            event.block_timestamp as u32,
        )),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    };

    if let Some(OnChainEventBody::IdRegisterEventBody(body)) = event.body {
        active_model.custody_address = Set(vec_u8_to_hex_string(&body.to));
        active_model.recovery_address = Set(vec_u8_to_hex_string(&body.recovery_address));
    } else {
        return None;
    }

    Some(active_model)
}

fn decode_abi_parameters(metadata: &[u8]) -> (i64, String, String, i64) {
    let param: EthParam = EthParam {
        name: "SignedKeyRequest".to_string(),
        type_: Type::Tuple(vec![
            ("requestFid".to_string(), Type::Uint(256)),
            ("requestSigner".to_string(), Type::Address),
            ("signature".to_string(), Type::Bytes),
            ("deadline".to_string(), Type::Uint(256)),
        ]),
        indexed: None,
    };
    dbg!(&param);

    let decoded =
        EthValue::decode_from_slice(metadata, &[param.type_]).expect("TODO: panic message");
    dbg!(&decoded);

    let metadata = decoded[0].clone();

    let mut t_rfid = 0i64;
    let mut t_rsigner = "".to_string();
    let mut t_signature = "".to_string();
    let mut t_deadline = 0i64;

    if let EthValue::Tuple(tt) = metadata {
        if let EthValue::Uint(rfid, 256) = tt[0].clone().1 {
            t_rfid = rfid.as_u64() as i64;
        }

        if let EthValue::Address(addr) = tt[1].clone().1 {
            t_rsigner = addr.to_string();
        }

        if let EthValue::Bytes(signature) = tt[2].clone().1 {
            t_signature = vec_u8_to_hex_string(&signature);
        }

        if let EthValue::Uint(deadline, 256) = tt[3].clone().1 {
            t_deadline = deadline.as_u64() as i64;
        }
    };

    (t_rfid, t_rsigner, t_signature, t_deadline)
}

// todo: parse abi parameters from metadata
pub fn signer_message_to_entity(event: OnChainEvent) -> Option<entity::signers::ActiveModel> {
    let mut active_model = entity::signers::ActiveModel {
        fid: Set(event.fid as i64),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    };
    let timestamp = farcaster_timestamp_to_datetime_with_tz(event.block_timestamp as u32);

    if let Some(OnChainEventBody::SignerEventBody(body)) = event.body {
        if let Ok(event_type) = SignerEventType::try_from(body.event_type) {
            match event_type {
                SignerEventType::None => {}
                SignerEventType::Add => {
                    let encoded_input = vec_u8_to_hex_string(&body.metadata);
                    dbg!(&encoded_input);

                    // process abi parameters
                    let (request_fid, request_signer, signature, deadline) =
                        decode_abi_parameters(&body.metadata);

                    active_model.metadata = Set(json!({
                        "request_fid": request_fid,
                        "request_signer": request_signer,
                        "signature": signature,
                        "deadline": deadline
                    }));
                    active_model.requester_fid = Set(request_fid);
                    active_model.key = Set(vec_u8_to_hex_string(&body.key));
                    active_model.key_type = Set(body.key_type as i32);
                    active_model.metadata_type = Set(body.metadata_type as i32);
                    active_model.added_at = Set(timestamp.clone());
                    active_model.updated_at = Set(timestamp);
                }
                SignerEventType::Remove => {
                    active_model.key = Set(vec_u8_to_hex_string(&body.key));
                    active_model.removed_at = Set(Some(timestamp.clone()));
                    active_model.updated_at = Set(timestamp);
                }
                SignerEventType::AdminReset => {}
            }
        }
    } else {
        return None;
    }

    Some(active_model)
}

pub fn storage_message_to_entity(event: OnChainEvent) -> Option<entity::storage::ActiveModel> {
    let mut active_model = entity::storage::ActiveModel {
        fid: Set(event.fid as i64),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    };
    let timestamp = farcaster_timestamp_to_datetime_with_tz(event.block_timestamp as u32);
    active_model.rented_at = Set(timestamp);

    if let Some(OnChainEventBody::StorageRentEventBody(body)) = event.body {
        active_model.units = Set(body.units as i32);
        active_model.payer = Set(vec_u8_to_hex_string(&body.payer));
        active_model.expires_at = Set(farcaster_timestamp_to_datetime_with_tz(body.expiry));
    } else {
        return None;
    }

    Some(active_model)
}
