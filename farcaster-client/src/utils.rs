use chrono::{TimeZone, Utc};
use entity::sea_orm::prelude::DateTimeWithTimeZone;

pub fn farcaster_timestamp_to_realworld_timestamp(timestamp: u32) -> u32 {
    1609430400 + timestamp
}

pub fn farcaster_timestamp_to_datetime_with_tz(timestamp: u32) -> DateTimeWithTimeZone {
    let ts = farcaster_timestamp_to_realworld_timestamp(timestamp);

    Utc.timestamp_opt(ts as i64, 0).unwrap().into()
}
