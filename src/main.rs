mod models;

use crate::models::Status;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use std::io;
// async fn status() -> impl Responder {
//     web::Json::Ok().json(Status {
//         status: "Ok".to_string(),
//     })
// }

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080/");
    println!(r#"Hello world!"#);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r#"Hello world!"#);

        Ok(())
    }
}