//! Integration tests for GET /api/products/:id endpoint

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use backend::{
    config::DatabaseConfig,
    db::{create_pool, run_migrations},
    models::Product,
    routes::products::products_routes,
};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;

/// Helper struct for error responses
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Test helper to create a test app
async fn create_test_app() -> (PgPool, DatabaseConfig) {
    // Load config from environment
    let config = DatabaseConfig::from_env().expect("Failed to load config");

    // Create database pool
    let pool = create_pool(&config).await.expect("Failed to create pool");

    // Run migrations
    run_migrations(&pool).await.expect("Failed to run migrations");

    (pool, config)
}

/// Test helper to create the router
fn create_router(pool: PgPool) -> axum::Router {
    products_routes(pool)
}

/// Clean up test data
async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query("DELETE FROM products")
        .execute(pool)
        .await
        .expect("Failed to clean up test data");
}

/// Insert a test product into the database
async fn insert_test_product(pool: &PgPool) -> Product {
    sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, description, price, stock, category, image_url)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, description, price, stock, category, image_url, created_at, updated_at
        "#,
    )
    .bind("Test Product")
    .bind(Some("A test product description"))
    .bind(rust_decimal::Decimal::new(1999, 2)) // 19.99
    .bind(100)
    .bind(Some("Electronics"))
    .bind(Some("https://example.com/image.jpg"))
    .fetch_one(pool)
    .await
    .expect("Failed to insert test product")
}

#[tokio::test]
async fn test_get_product_by_valid_id() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Insert a test product
    let product = insert_test_product(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/products/{}", product.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 200 OK
    assert_eq!(response.status(), StatusCode::OK);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Assert response structure
    assert!(json.is_object());
    assert!(json.get("data").is_some());

    let data = &json["data"];
    assert_eq!(data["id"], product.id.to_string());
    assert_eq!(data["name"], "Test Product");
    assert_eq!(data["description"], "A test product description");
    assert_eq!(data["price"], "19.99");
    assert_eq!(data["stock"], 100);
    assert_eq!(data["category"], "Electronics");
    assert_eq!(data["image_url"], "https://example.com/image.jpg");

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_product_not_found() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with a non-existent UUID
    let non_existent_uuid = uuid::Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/products/{}", non_existent_uuid))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 404 NOT FOUND
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert error message
    assert_eq!(error.error, "Product not found");

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_product_invalid_uuid_format() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with an invalid UUID
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/products/not-a-valid-uuid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert error message
    assert_eq!(error.error, "Invalid UUID format");

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_product_with_multiple_products() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Insert multiple test products
    let product1 = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, description, price, stock, category)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, price, stock, category, image_url, created_at, updated_at
        "#,
    )
    .bind("Product 1")
    .bind(Some("First product"))
    .bind(rust_decimal::Decimal::new(1000, 2)) // 10.00
    .bind(50)
    .bind(Some("Books"))
    .fetch_one(&pool)
    .await
    .expect("Failed to insert product 1");

    let product2 = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (name, description, price, stock, category)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, description, price, stock, category, image_url, created_at, updated_at
        "#,
    )
    .bind("Product 2")
    .bind(Some("Second product"))
    .bind(rust_decimal::Decimal::new(2000, 2)) // 20.00
    .bind(30)
    .bind(Some("Electronics"))
    .fetch_one(&pool)
    .await
    .expect("Failed to insert product 2");

    // Create router
    let app = create_router(pool.clone());

    // Fetch first product
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/products/{}", product1.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["id"], product1.id.to_string());
    assert_eq!(json["data"]["name"], "Product 1");

    // Fetch second product
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(&format!("/products/{}", product2.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["data"]["id"], product2.id.to_string());
    assert_eq!(json["data"]["name"], "Product 2");

    // Cleanup
    cleanup_test_data(&pool).await;
}
