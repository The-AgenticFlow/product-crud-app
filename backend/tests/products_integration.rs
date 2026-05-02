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
    error: ErrorBody,
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(default)]
    details: Vec<FieldError>,
}

/// Test helper to create a test app
async fn create_test_app() -> (PgPool, DatabaseConfig) {
    // Load .env file for test environment
    dotenvy::dotenv().ok();

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
                .uri(format!("/products/{}", product.id))
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
                .uri(format!("/products/{}", non_existent_uuid))
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
    assert!(error.error.message.contains("not found"));

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
    assert!(error.error.message.contains("Invalid UUID"));

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
                .uri(format!("/products/{}", product1.id))
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
                .uri(format!("/products/{}", product2.id))
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

// ==================== POST /api/products Tests ====================

/// Helper struct for created response
#[derive(Debug, Deserialize)]
struct CreatedResponse {
    data: Product,
    message: String,
}

#[derive(Debug, Deserialize)]
struct FieldError {
    field: String,
    message: String,
}

#[tokio::test]
async fn test_create_product_valid_request() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with valid product data
    let product_json = serde_json::json!({
        "name": "New Product",
        "description": "A new product description",
        "price": "29.99",
        "stock": 50,
        "category": "Electronics",
        "image_url": "https://example.com/new-product.jpg"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 201 CREATED
    assert_eq!(response.status(), StatusCode::CREATED);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let created: CreatedResponse = serde_json::from_slice(&body).unwrap();

    // Assert response structure
    assert_eq!(created.message, "Product created successfully");
    assert_eq!(created.data.name, "New Product");
    assert_eq!(created.data.description, Some("A new product description".to_string()));
    assert_eq!(created.data.price, rust_decimal::Decimal::new(2999, 2)); // 29.99
    assert_eq!(created.data.stock, 50);
    assert_eq!(created.data.category, Some("Electronics".to_string()));
    assert_eq!(created.data.image_url, Some("https://example.com/new-product.jpg".to_string()));

    // Verify auto-generated fields
    assert!(!created.data.id.is_nil());
    assert!(created.data.created_at.timestamp() > 0);
    assert!(created.data.updated_at.timestamp() > 0);

    // Verify product was actually saved in database
    let saved_product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, stock, category, image_url, created_at, updated_at FROM products WHERE id = $1"
    )
    .bind(created.data.id)
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch saved product");

    assert_eq!(saved_product.name, "New Product");

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_missing_required_fields() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with missing name (required field)
    let product_json = serde_json::json!({
        "price": "29.99",
        "stock": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 422 UNPROCESSABLE ENTITY (JSON deserialization failed)
    // When required fields are missing from JSON, Axum's Json extractor returns 422
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    // The body content is not important for this test - we just verify the status code
    // indicates that JSON deserialization failed due to missing required fields

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_empty_name() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with empty name
    let product_json = serde_json::json!({
        "name": "",
        "price": "29.99",
        "stock": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for name field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "name" && e.message.contains("required")));

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_negative_price() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with negative price
    let product_json = serde_json::json!({
        "name": "Test Product",
        "price": "-10.00",
        "stock": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for price field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "price" && e.message.contains("greater than or equal to 0")));

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_negative_stock() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with negative stock
    let product_json = serde_json::json!({
        "name": "Test Product",
        "price": "29.99",
        "stock": -5
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for stock field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "stock" && e.message.contains("greater than or equal to 0")));

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_invalid_url_format() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with invalid URL format
    let product_json = serde_json::json!({
        "name": "Test Product",
        "price": "29.99",
        "stock": 50,
        "image_url": "invalid-url"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for image_url field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "image_url" && e.message.contains("http:// or https://")));

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_name_too_long() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with name exceeding 255 characters
    let long_name = "x".repeat(256);
    let product_json = serde_json::json!({
        "name": long_name,
        "price": "29.99",
        "stock": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for name field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "name" && e.message.contains("255")));

    // Cleanup
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_product_description_too_long() {
    // Setup
    let (pool, _config) = create_test_app().await;
    cleanup_test_data(&pool).await;

    // Create router
    let app = create_router(pool.clone());

    // Create request with description exceeding 1000 characters
    let long_description = "x".repeat(1001);
    let product_json = serde_json::json!({
        "name": "Test Product",
        "description": long_description,
        "price": "29.99",
        "stock": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/products")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_vec(&product_json).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert status code is 400 BAD REQUEST
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Parse response body
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let error: ErrorResponse = serde_json::from_slice(&body).unwrap();

    // Assert validation error for description field
    assert!(!error.error.details.is_empty());
    assert!(error.error.details.iter().any(|e| e.field == "description" && e.message.contains("1000")));

    // Cleanup
    cleanup_test_data(&pool).await;
}
