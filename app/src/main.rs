use actix_web::{HttpServer, App,web};

mod routes;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let client_options =
        mongodb::options::ClientOptions::parse("mongodb+srv://{DB_USERNAME}:{DB_PASSWORD}@test.gt1ft.mongodb.net/{DB_NAME}?retryWrites=true&w=majority").await.unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(client.clone()))
            .service(routes::index)
            .service(routes::delete)
            .service(routes::create)
            .service(routes::update)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}