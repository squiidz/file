use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{Metadata};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    created: NaiveDateTime,
    accessed: NaiveDateTime,
    modified: NaiveDateTime,
    size: u64,
    // permission: Permissions,
}

impl From<Metadata> for Meta {
    fn from(meta_data: Metadata) -> Meta {
        Meta {
            created: convert_system_time(meta_data.created().unwrap()),
            accessed: convert_system_time(meta_data.accessed().unwrap()),
            modified: convert_system_time(meta_data.modified().unwrap()),
            size: meta_data.len(),
            // permission: meta_data.permissions(),
        }
    }
}

fn convert_system_time(sys_time: SystemTime) -> NaiveDateTime {
    let duration = match sys_time.duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => panic!(),
    };
    NaiveDateTime::from_timestamp(duration.as_secs() as i64, 0)
}
