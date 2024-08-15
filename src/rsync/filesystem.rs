use chrono::{DateTime, Utc};

pub struct Folder {
    name: String,
    path: String,
    last_modified: DateTime<Utc>,
}

pub struct File {
    name: String,
    path: String,
    extension: String,
    last_modified: DateTime<Utc>,

}