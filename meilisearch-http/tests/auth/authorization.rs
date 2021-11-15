use crate::common::Server;
use maplit::hashmap;
use once_cell::sync::Lazy;
use serde_json::json;
use std::collections::{HashMap, HashSet};

static AUTHORIZATIONS: Lazy<HashMap<(&'static str, &'static str), &'static str>> =
    Lazy::new(|| {
        hashmap! {
            ("POST",    "/indexes/products/search") =>                         "search",
            ("GET",     "/indexes/products/search") =>                         "search",
            ("POST",    "/indexes/products/documents") =>                      "documents.add",
            ("GET",     "/indexes/products/documents") =>                      "documents.get",
            ("GET",     "/indexes/products/documents/0") =>                    "documents.get",
            ("DELETE",  "/indexes/products/documents/0") =>                    "documents.delete",
            ("POST",    "/indexes/products/updates") =>                        "tasks.get",
            ("POST",    "/indexes/products/updates/0") =>                      "tasks.get",
            ("PUT",     "/indexes/products/") =>                               "indexes.update",
            ("GET",     "/indexes/products/") =>                               "indexes.get",
            ("DELETE",  "/indexes/products/") =>                               "indexes.delete",
            ("POST",    "/indexes") =>                                         "indexes.add",
            ("GET",     "/indexes") =>                                         "indexes.get",
            ("GET",     "/indexes/products/settings") =>                       "settings.get",
            ("GET",     "/indexes/products/settings/displayedAttributes") =>   "settings.get",
            ("GET",     "/indexes/products/settings/distinctAttribute") =>     "settings.get",
            ("GET",     "/indexes/products/settings/filterableAttributes") =>  "settings.get",
            ("GET",     "/indexes/products/settings/rankingRules") =>          "settings.get",
            ("GET",     "/indexes/products/settings/searchableAttributes") =>  "settings.get",
            ("GET",     "/indexes/products/settings/sortableAttributes") =>    "settings.get",
            ("GET",     "/indexes/products/settings/stopWords") =>             "settings.get",
            ("GET",     "/indexes/products/settings/synonyms") =>              "settings.get",
            ("DELETE",  "/indexes/products/settings") =>                       "settings.update",
            ("POST",    "/indexes/products/settings") =>                       "settings.update",
            ("POST",    "/indexes/products/settings/displayedAttributes") =>   "settings.update",
            ("POST",    "/indexes/products/settings/distinctAttribute") =>     "settings.update",
            ("POST",    "/indexes/products/settings/filterableAttributes") =>  "settings.update",
            ("POST",    "/indexes/products/settings/rankingRules") =>          "settings.update",
            ("POST",    "/indexes/products/settings/searchableAttributes") =>  "settings.update",
            ("POST",    "/indexes/products/settings/sortableAttributes") =>    "settings.update",
            ("POST",    "/indexes/products/settings/stopWords") =>             "settings.update",
            ("POST",    "/indexes/products/settings/synonyms") =>              "settings.update",
            ("GET",     "/indexes/products/stats") =>                          "stats.get",
            ("GET",     "/stats") =>                                           "stats.get",
            ("POST",    "/dumps") =>                                           "dumps.create",
            ("GET",     "/dumps/0") =>                                         "dumps.get",
        }
    });

static ALL_ACTIONS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| AUTHORIZATIONS.values().cloned().collect());

#[actix_rt::test]
async fn error_access_expired_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": ALL_ACTIONS.clone(),
        "expiresAt": "2020-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();
    server.use_api_key(&key);

    for (method, route) in AUTHORIZATIONS.keys() {
        let (response, code) = server.dummy_request(method, route).await;

        let expected_response = json!({
            "message": "The provided API key is invalid.",
            "code": "invalid_api_key",
            "type": "auth",
            "link": "https://docs.meilisearch.com/errors#invalid_api_key"
        });

        assert_eq!(response, expected_response);
        assert_eq!(code, 403);
    }
}

#[actix_rt::test]
async fn error_access_unauthorized_index() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["sales"],
        "actions": ALL_ACTIONS.clone(),
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();
    server.use_api_key(&key);

    for (method, route) in AUTHORIZATIONS
        .keys()
        // filter `products` index routes
        .filter(|(_, route)| route.starts_with("/indexes/products"))
    {
        let (response, code) = server.dummy_request(method, route).await;

        let expected_response = json!({
            "message": "The provided API key is invalid.",
            "code": "invalid_api_key",
            "type": "auth",
            "link": "https://docs.meilisearch.com/errors#invalid_api_key"
        });

        assert_eq!(response, expected_response);
        assert_eq!(code, 403);
    }
}

#[actix_rt::test]
async fn error_access_unauthorized_action() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": [],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();
    server.use_api_key(&key);

    for ((method, route), action) in AUTHORIZATIONS.iter() {
        server.use_api_key("MASTER_KEY");

        // Patch API key letting all rights but the needed one.
        let content = json!({
            "actions": ALL_ACTIONS.iter().cloned().filter(|a| a != action).collect::<Vec<_>>(),
        });
        let (_, code) = server.patch_api_key(&key, content).await;
        assert_eq!(code, 200);

        server.use_api_key(&key);
        let (response, code) = server.dummy_request(method, route).await;

        let expected_response = json!({
            "message": "The provided API key is invalid.",
            "code": "invalid_api_key",
            "type": "auth",
            "link": "https://docs.meilisearch.com/errors#invalid_api_key"
        });

        assert_eq!(response, expected_response);
        assert_eq!(code, 403);
    }
}

#[actix_rt::test]
async fn access_authorized_action() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": [],
        "expiresAt": "2050-11-13T00:00:00Z"
    });

    let (response, code) = server.add_api_key(content).await;
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    let key = response["key"].as_str().unwrap();
    server.use_api_key(&key);

    for ((method, route), action) in AUTHORIZATIONS.iter() {
        server.use_api_key("MASTER_KEY");

        // Patch API key letting only the needed action.
        let content = json!({
            "actions": [action],
        });
        let (_, code) = server.patch_api_key(&key, content).await;
        assert_eq!(code, 200);

        server.use_api_key(&key);
        let (response, code) = server.dummy_request(method, route).await;

        let unexpected_response = json!({
            "message": "The provided API key is invalid.",
            "code": "invalid_api_key",
            "type": "auth",
            "link": "https://docs.meilisearch.com/errors#invalid_api_key"
        });

        assert_ne!(response, unexpected_response);
        assert_ne!(code, 403);
    }
}

#[actix_rt::test]
async fn access_authorized_stats() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    // create index `test`
    let index = server.index("test");
    let (_, code) = index.create(Some("id")).await;
    assert_eq!(code, 201);
    // create index `products`
    let index = server.index("products");
    let (_, code) = index.create(Some("product_id")).await;
    assert_eq!(code, 201);

    // create key with access on `products` index only.
    let content = json!({
        "indexes": ["products"],
        "actions": ALL_ACTIONS.clone(),
        "expiresAt": "2050-11-13T00:00:00Z"
    });
    let (response, code) = server.add_api_key(content).await;
    assert_eq!(code, 201);
    assert!(response["key"].is_string());

    // use created key.
    let key = response["key"].as_str().unwrap();
    server.use_api_key(&key);

    let (response, code) = server.stats().await;
    assert_eq!(code, 200);

    // key should have access on `products` index.
    assert!(response["indexes"].get("products").is_some());

    // key should not have access on `test` index.
    assert!(response["indexes"].get("test").is_none());
}
