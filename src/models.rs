use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub name: String,
    pub ticker: String,
    pub asset_type: String,
    pub quantity: f64,
    pub unit_value: f64,
}

impl Asset {
    pub fn total_value(&self) -> f64 {
        self.quantity * self.unit_value
    }
}

pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}
