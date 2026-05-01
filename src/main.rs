use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    id: String,
    name: String,
    description: String,
    price: f64,
    stock: i32,
    category: String,
}

#[get("/api/products")]
async fn get_products() -> impl Responder {
    HttpResponse::Ok().json(Vec::<Product>::new())
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_products).service(health))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: "1".to_string(),
            name: "Test".to_string(),
            description: "A test product".to_string(),
            price: 9.99,
            stock: 10,
            category: "test".to_string(),
        };
        let json = serde_json::to_string(&product).unwrap();
        assert!(json.contains("Test"));
    }
}
