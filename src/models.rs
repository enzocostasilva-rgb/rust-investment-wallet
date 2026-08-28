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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_total_value() {
        let asset = Asset {
            id: 1,
            name: "Bitcoin".to_string(),
            ticker: "BTC".to_string(),
            asset_type: "CRYPTO".to_string(),
            quantity: 0.5,
            unit_value: 100_000.0,
        };

        assert_eq!(asset.total_value(), 50_000.0);
    }

    #[test]
    fn test_asset_total_value_with_multiple_units() {
        let asset = Asset {
            id: 2,
            name: "Petrobras".to_string(),
            ticker: "PETR4".to_string(),
            asset_type: "STOCK".to_string(),
            quantity: 100.0,
            unit_value: 35.50,
        };

        assert_eq!(asset.total_value(), 3550.0);
    }
}

