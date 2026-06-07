#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}
