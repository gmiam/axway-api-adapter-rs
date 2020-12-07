use actix_web::body::Body;
use actix_web::HttpResponse;

pub trait AdapterHttpResponses {
    fn ok_200(self) -> HttpResponse;
}

impl<T: Into<Body>> AdapterHttpResponses for T {
    fn ok_200(self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(self)
    }
}

pub fn error_500() -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("text/html; charset=utf-8")
        .body("Internal Server Error")
}
