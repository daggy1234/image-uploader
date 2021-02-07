use crate::AUTH_TOKEN;
use actix_web::dev::ServiceRequest;
use actix_web::web::HttpResponse;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[allow(dead_code)]
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if credentials.token() == *AUTH_TOKEN {
        Ok(req)
    } else {
        Err(HttpResponse::Unauthorized().body("Unauthorized").into())
    }
}
