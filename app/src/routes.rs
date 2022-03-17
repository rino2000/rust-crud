use actix_web::{get, web::{self, Json}, HttpResponse, post, delete, put, Responder, http::header::ContentType};
use futures::StreamExt;
use mongodb::{Client, Collection, bson::{Document, doc, DateTime, oid::ObjectId}};

use crate::models::{Post,DeletePost,UpdatePost};

#[get("/")]
pub async fn index(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<Document> = client.database("app").collection("posts");

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(_) => {
                HttpResponse::Ok().body("No results found");
            }
        }
    }
    println!("{:#?}", results);

    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(results)
}

#[post("/new")]
pub async fn create(client: web::Data<Client>,form: Json<Post>) -> HttpResponse {

    let id = ObjectId::new();
    let title = form.title.to_owned();
    let content = form.content.to_owned();
    let created_at = DateTime::now().to_string();

    let serialized = doc! {
        "_id": id,
        "title": title,
        "content": content,
        "created_at": created_at,
    };

    let collection: Collection<Document> = client.database("app").collection("posts");
    
    let _insert = collection.insert_one(serialized, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(_) => {
                HttpResponse::Ok().body("No results");
            }
        }
    }
    println!("{:#?}", results);

    HttpResponse::Ok().json(results)
}

#[delete("/delete")]
pub async fn delete(client: web::Data<Client>,form: Json<DeletePost>) -> HttpResponse {

    let id = form.id;

    let serialized = doc! {"_id": id};

    let collection: Collection<Document> = client.database("app").collection("posts");
    
    let _insert = collection.delete_one(serialized, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(_) => {
                HttpResponse::Ok().body("No results");
            }
        }
    }
    println!("{:#?}", results);

    HttpResponse::Ok().json(results)
}

#[put("/update")]
pub async fn update(client: web::Data<Client>,form: Json<UpdatePost>) -> HttpResponse {

    let id = form.id;
    let title = form.title.to_owned();
    let content = form.content.to_owned();

    let serialized = doc! {"_id": id}; // Filter by ID
    let new_data = doc! {"$set": {"title":title,"content":content}}; // update query

    let collection: Collection<Document> = client.database("app").collection("posts");
    
    let _insert = collection.update_one(serialized,new_data,None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();

    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            Err(_) => {
                HttpResponse::Ok().body("No results");
            }
        }
    }
    println!("{:#?}", results);

    HttpResponse::Ok().json(results)
}