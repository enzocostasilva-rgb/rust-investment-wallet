use std::convert::Infallible;

use axum::extract::FromRequestParts;
use sqlx::PgPool;

use crate::{
    app::AppState,
    models::{Asset, UserRecord},
};

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub async fn list_assets(&self) -> sqlx::Result<Vec<Asset>> {
        sqlx::query_as!(
            Asset,
            "SELECT id, name, ticker, asset_type, quantity, unit_value
             FROM assets
             ORDER BY id;"
        )
        .fetch_all(&self.db)
        .await
    }

    pub async fn create_asset(
        &self,
        name: String,
        ticker: String,
        asset_type: String,
        quantity: f64,
        unit_value: f64,
    ) -> sqlx::Result<Asset> {
        sqlx::query_as!(
            Asset,
            "INSERT INTO assets (name, ticker, asset_type, quantity, unit_value)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, name, ticker, asset_type, quantity, unit_value;",
            name,
            ticker,
            asset_type,
            quantity,
            unit_value
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn update_asset(
        &self,
        asset_id: i64,
        name: Option<String>,
        ticker: Option<String>,
        asset_type: Option<String>,
        quantity: Option<f64>,
        unit_value: Option<f64>,
    ) -> sqlx::Result<Option<Asset>> {
        sqlx::query_as!(
            Asset,
            "UPDATE assets
             SET name=COALESCE($2, name),
                 ticker=COALESCE($3, ticker),
                 asset_type=COALESCE($4, asset_type),
                 quantity=COALESCE($5, quantity),
                 unit_value=COALESCE($6, unit_value)
             WHERE id=$1
             RETURNING id, name, ticker, asset_type, quantity, unit_value;",
            asset_id,
            name,
            ticker,
            asset_type,
            quantity,
            unit_value
        )
        .fetch_optional(&self.db)
        .await
    }

    pub async fn add_user(&self, username: &str, password_hash: &str) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "INSERT INTO users (username, password_hash)
             VALUES ($1, $2)
             RETURNING id, username, password_hash;",
            username,
            password_hash,
        )
        .fetch_one(&self.db)
        .await
    }

    pub async fn get_user_by_name(&self, username: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "SELECT id, username, password_hash
             FROM users
             WHERE username = $1;",
            username
        )
        .fetch_optional(&self.db)
        .await
    }
}

impl FromRequestParts<AppState> for Repository {
    type Rejection = Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            db: state.db.clone(),
        })
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
