use actix_web::{App, test};
use actix_web::http::StatusCode;
use actix_web::test::read_body_json;
use surf::StatusCode as SurfStatusCode;
use wiremock::{MockServer, ResponseTemplate};

use shakespearemon::settings::Settings;
use shakespearemon::shakespeare::shakespeare_translation_response::ShakespeareTranslationResponse;
use shakespearemon::translation_service::ShakespearemonResponse;
use shakespearemon::translation_service::translate_pokemon_description_by_shakespeare;

use crate::helpers::{generate_poke_species_response, get_application, mock_poke_client_request, mock_shakespeare_client_request, UndefinedResponse};

#[actix_rt::test]
async fn returns_500_if_poke_api_returns_undefined_response() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    let response = UndefinedResponse {
        message: "message".to_owned()
    };

    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::Ok).set_body_json(response), "/ozer".to_owned()).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/ozer").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn returns_404_pokemon_named_not_found() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::NotFound), "/ozer".to_owned()).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/ozer").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
async fn returns_500_if_poke_api_sends_too_many_requests() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::TooManyRequests), "/ozer".to_owned()).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/ozer").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn returns_500_if_shakespeare_translator_api_returns_undefined_response() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::Ok), "/ozer".to_owned()).await;

    let undefined_response = UndefinedResponse {
        message: "undefined".to_owned()
    };

    mock_shakespeare_client_request(&mock_server, ResponseTemplate::new(StatusCode::OK).set_body_json(undefined_response)).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/ozer").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn returns_500_if_shakespeare_translator_api_returns_too_many_requests() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    let poke_species_response = generate_poke_species_response("en".to_owned());

    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::Ok).set_body_json(poke_species_response), "/ozer".to_owned()).await;

    mock_shakespeare_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::TooManyRequests)).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/ozer").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_rt::test]
async fn gets_translation_of_pokemon_by_shakespeare() {
    let mock_server = MockServer::start().await;

    let application = get_application(mock_server.uri());

    let poke_species_response = generate_poke_species_response("en".to_owned());
    mock_poke_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::Ok).set_body_json(poke_species_response), "/pikachu".to_owned()).await;

    let translation = ShakespeareTranslationResponse::new(String::from("translated"), String::from("text"), String::from("translation"));
    mock_shakespeare_client_request(&mock_server, ResponseTemplate::new(SurfStatusCode::Ok).set_body_json(translation)).await;

    let mut app = test::init_service(App::new()
        .data(Settings {
            application
        })
        .service(translate_pokemon_description_by_shakespeare)).await;

    let req = test::TestRequest::get()
        .uri("/pokemon/pikachu").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let shakespearemon_response: ShakespearemonResponse = read_body_json(resp).await;
    assert_eq!(shakespearemon_response.name, "pikachu");
    assert_eq!(shakespearemon_response.description, "translated");
}
