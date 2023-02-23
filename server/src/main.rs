use actix_web::{App, HttpServer};
use cats::{add_cat, create_cat_scope, get_cats};

use greeter::greet;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(create_cat_scope())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
