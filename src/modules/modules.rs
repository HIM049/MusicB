
#[derive(Serialize, Deserialize, Debug)]
pub struct FileChange {
    pub kind: String,
    pub path: String,
}
