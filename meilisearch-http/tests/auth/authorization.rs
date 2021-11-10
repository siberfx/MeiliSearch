use crate::common::Server;
use maplit::hashmap;
use once_cell::sync::Lazy;
use serde_json::json;
use std::collections::HashMap;

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
            ("GET",     "/indexes/products/stats") =>                          "stats",
            ("GET",     "/stats") =>                                           "stats",
            ("POST",    "/dumps") =>                                           "dumps",
            ("GET",     "/dumps/0") =>                                         "dumps",
        }
    });

#[actix_rt::test]
async fn error_access_expired_key() {
    let mut server = Server::new_auth().await;
    server.use_api_key("MASTER_KEY");

    let content = json!({
        "indexes": ["products"],
        "actions": [
            "search",
            "documents.add",
            "documents.get",
            "documents.delete",
            "indexes.add",
            "indexes.get",
            "indexes.update",
            "indexes.delete",
            "tasks.get",
            "settings.get",
            "settings.update",
            "stats",
            "dumps"
        ],
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
