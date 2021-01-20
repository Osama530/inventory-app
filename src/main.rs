#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    price: u32,
}

impl Product {
    fn new(id: String, name: String, price: u32)-> Product {
        Product{
            id,
            name,
            price,
        }
    }
}

// get request to pre-defuned json object
#[get("/product")]
fn items() -> Json<Product> {
    Json(
        Product::new(
            "edfsdf4fsf54".to_string(),
            "mobile".to_string(),
            525)
    )
}

// get request to qurry param as id
#[get("/product?<id>")]
// http://localhost:8111/inventory/product?id=5sd5s5s
fn qurry_item(id: String)->Json<Product>{
    
    Json(
        Product::new(
            id.to_string(),
            "qurry product".to_string(),
            445 )
    )
}

//creating a product from browser
#[post("/product/<id>?<name>&<price>")]
fn create_product(id: String, name: String, price: u32)->Json<Product>{
    Json(
        Product::new(
            id.to_string(),
            name.to_string(),
            price )
    )
}

fn main() {
    rocket::ignite().mount("/inventory", routes![items, qurry_item, create_product]).launch();
}