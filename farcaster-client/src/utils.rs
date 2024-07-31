use chrono::{TimeZone, Utc};
use entity::sea_orm::prelude::DateTimeWithTimeZone;

pub fn farcaster_timestamp_to_realworld_timestamp(timestamp: i64) -> i64 {
    1609430400 + timestamp
}

pub fn farcaster_timestamp_to_datetime_with_tz(timestamp: i64) -> DateTimeWithTimeZone {
    let ts = farcaster_timestamp_to_realworld_timestamp(timestamp);

    Utc.timestamp_opt(ts, 0).unwrap().into()
}

pub fn vec_u8_to_hex_string(vec_u8: &[u8]) -> String {
    let hex = vec_u8
        .iter()
        .map(|u| format!("{:x}", u))
        .collect::<Vec<_>>()
        .join("");

    format!("0x{hex}")
}
