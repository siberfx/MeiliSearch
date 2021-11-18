use actix_web::{web, HttpRequest, HttpResponse};
use log::debug;
use meilisearch_lib::MeiliSearch;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::analytics::Analytics;
use crate::error::ResponseError;
use crate::extractors::authentication::{policies::*, GuardedData};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(create_api_key))
            .route(web::get().to(list_api_keys)),
    )
    .service(
        web::resource("/{api_key}")
            .route(web::get().to(get_api_key))
            .route(web::patch().to(patch_api_key))
            .route(web::delete().to(delete_api_key)),
    );
}

pub async fn create_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    body: web::Json<Value>,
    _req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = meilisearch.create_key(body.into_inner()).await?;

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Created().json(res))
}

pub async fn list_api_keys(
    meilisearch: GuardedData<Private, MeiliSearch>,
    _req: HttpRequest,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = meilisearch.list_keys().await?;

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    path: web::Path<AuthParam>,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = meilisearch.get_key(&path.api_key).await?;

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn patch_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    body: web::Json<Value>,
    path: web::Path<AuthParam>,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    let res = meilisearch
        .update_key(&path.api_key, body.into_inner())
        .await?;

    debug!("returns: {:?}", res);
    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete_api_key(
    meilisearch: GuardedData<Private, MeiliSearch>,
    path: web::Path<AuthParam>,
    analytics: web::Data<dyn Analytics>,
) -> Result<HttpResponse, ResponseError> {
    meilisearch.delete_key(&path.api_key).await?;

    Ok(HttpResponse::NoContent().json(()))
}

#[derive(Deserialize)]
pub struct AuthParam {
    api_key: String,
}
