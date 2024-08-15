use chrono::{DateTime, Utc};

pub struct Folder {
    name: String,
    last_modified: DateTime<Utc>,
}

pub struct File {
    name: String,
    last_modified: DateTime<Utc>,

}