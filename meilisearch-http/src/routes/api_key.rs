use actix_web::{web, HttpRequest, HttpResponse};
use log::debug;
use meilisearch_lib::MeiliSearch;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::analytics::Analytics;
use crate::error::ResponseError;
use crate::extractors::authentication::{policies::*, GuardedData};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::post().to(create_api_key)))
        .service(web::resource("").route(web::get().to(list_api_keys)))
        .service(web::resource("/{api_key}").route(web::get().to(get_api_key)))
        .service(web::resource("/{api_key}").route(web::put().to(put_api_key)))
        .service(web::resource("/{api_key}").route(web::patch().to(patch_api_key)))
        .service(web::resource("/{api_key}").route(web::delete().to(delete_api_key)));
}

pub async fn create_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    // analytics.publish("API key Created".to_string(), json!({}), Some(&req));

    // let res = meilisearch.create_api_key().await?;

    let res = json!("create_api_key unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}

pub async fn list_api_keys(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = json!("list_api_keys unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}

pub async fn get_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = json!("get_api_key unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}

pub async fn put_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = json!("put_api_key unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}

pub async fn patch_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = json!("patch_api_key unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}

pub async fn delete_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = json!("delete_api_key unimplemented");

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Accepted().json(res))
}
