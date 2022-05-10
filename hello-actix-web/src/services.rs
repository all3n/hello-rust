use actix_web::{get, HttpResponse, Responder};
pub mod services{
    #[get("/test")]
    async pub fn hello_test() -> impl Responder {
        HttpResponse::Ok().body("Hello test!")
    }
}