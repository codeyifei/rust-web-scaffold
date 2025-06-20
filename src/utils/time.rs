use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn naive_to_local(t: NaiveDateTime) -> DateTime<Local> {
    DateTime::<Utc>::from_naive_utc_and_offset(t, Utc).with_timezone(&Local)
}
