use actix_web::web::*;
use actix_web::{get, post, HttpResponse, Responder, Scope};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

struct Cats {
    cats: Arc<Mutex<Vec<Cat>>>,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, serde::Deserialize)]
struct Cat {
    name: String,
    age: u8,
}

#[get("")]
async fn get_cats(data: Data<Cats>) -> impl Responder {
    let cats = data.cats.lock().unwrap();

    println!("Cats {:?}", cats.clone());

    HttpResponse::Ok().body(format!("{:?}", cats.clone()))
}

#[post("/add")]
async fn add_cat(cat: Json<Cat>, data: Data<Cats>) -> impl Responder {
    let mut cats = data.cats.lock().unwrap();

    println!("Adding {:?}", cat.clone());

    cats.push(cat.into_inner());

    HttpResponse::Ok()
}

pub fn create_cat_scope() -> Scope {
    let cat_state = Data::new(Cats {
        cats: Arc::new(Mutex::new(Vec::new())),
    });
    scope("/cats")
        .app_data(cat_state.clone())
        .service(add_cat)
        .service(get_cats)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
