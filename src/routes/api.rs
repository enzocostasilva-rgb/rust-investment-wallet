use axum::{Json, Router, routing::get};
use serde::Deserialize;

use crate::{
    app::AppState,
    auth::admin::Admin,
    error::AppError,
    models::Asset,
    repository::Repository,
};

pub fn router() -> Router<AppState> {
    Router::new().route(
        "/assets",
        get(list_assets).post(create_asset).patch(update_asset),
    )
}

#[tracing::instrument(skip_all)]
async fn list_assets(repository: Repository) -> Result<Json<Vec<Asset>>, AppError> {
    let assets = repository.list_assets().await?;
    Ok(Json(assets))
}

#[derive(Deserialize)]
struct CreateAssetRequest {
    name: String,
    ticker: String,
    asset_type: String,
    quantity: f64,
    unit_value: f64,
}

#[tracing::instrument(skip_all)]
async fn create_asset(
    _: Admin,
    repository: Repository,
    Json(request): Json<CreateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    let new_asset = repository
        .create_asset(
            request.name,
            request.ticker,
            request.asset_type,
            request.quantity,
            request.unit_value,
        )
        .await?;

    Ok(Json(new_asset))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    id: i64,
    name: Option<String>,
    ticker: Option<String>,
    asset_type: Option<String>,
    quantity: Option<f64>,
    unit_value: Option<f64>,
}

#[tracing::instrument(skip_all)]
async fn update_asset(
    _: Admin,
    repository: Repository,
    Json(request): Json<UpdateAssetRequest>,
) -> Result<Json<Asset>, AppError> {
    match repository
        .update_asset(
            request.id,
            request.name,
            request.ticker,
            request.asset_type,
            request.quantity,
            request.unit_value,
        )
        .await?
    {
        Some(updated_asset) => Ok(Json(updated_asset)),
        None => Err(AppError::AssetDoesNotExist),
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;

    #[sqlx::test]
    async fn test_create_asset(db: PgPool) {
        let request = CreateAssetRequest {
            name: "Bitcoin".to_string(),
            ticker: "BTC".to_string(),
            asset_type: "CRYPTO".to_string(),
            quantity: 0.5,
            unit_value: 100_000.0,
        };

        let Json(new_asset) = create_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(new_asset.id, 1);
        assert_eq!(new_asset.name, "Bitcoin");
        assert_eq!(new_asset.ticker, "BTC");
        assert_eq!(new_asset.asset_type, "CRYPTO");
        assert_eq!(new_asset.quantity, 0.5);
        assert_eq!(new_asset.unit_value, 100_000.0);

        insta::assert_json_snapshot!(new_asset);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_list_assets(db: PgPool) {
        let Json(assets) = list_assets(db.into()).await.expect("success");

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].name, "Bitcoin");
        assert_eq!(assets[0].ticker, "BTC");
        assert_eq!(assets[0].asset_type, "CRYPTO");
        assert_eq!(assets[0].quantity, 0.5);
        assert_eq!(assets[0].unit_value, 100_000.0);

        insta::assert_json_snapshot!(assets);
    }

    #[sqlx::test(fixtures("bitcoin_asset"))]
    async fn test_update_asset(db: PgPool) {
        let request = UpdateAssetRequest {
            id: 1,
            name: Some("Ethereum".to_string()),
            ticker: Some("ETH".to_string()),
            asset_type: Some("CRYPTO".to_string()),
            quantity: Some(2.0),
            unit_value: Some(20_000.0),
        };

        let Json(updated_asset) = update_asset(Admin, db.into(), Json(request))
            .await
            .expect("success");

        assert_eq!(updated_asset.id, 1);
        assert_eq!(updated_asset.name, "Ethereum");
        assert_eq!(updated_asset.ticker, "ETH");
        assert_eq!(updated_asset.asset_type, "CRYPTO");
        assert_eq!(updated_asset.quantity, 2.0);
        assert_eq!(updated_asset.unit_value, 20_000.0);

        insta::assert_json_snapshot!(updated_asset);
    }
}
