#![allow(unused_imports)]

extern crate actix_web;
extern crate $name_snake_case$;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate diesel;

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate serde_urlencoded;
#[macro_use]
extern crate lazy_static;

use actix_web::{
    http::{header::HeaderValue, HeaderMap},
    test::TestServer,
    App,
};
use reqwest::{Client, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::{env, fmt};

use $name_snake_case$::{
    api::{self, ApiAccess, ApiAggregator},
    ID,
    service,
};

pub mod helper;

pub use crate::helper::{ApiHelper, TestHelper};

/// Kind of API service.
///
#[derive(Debug, Clone, Copy)]
pub enum ApiKind {
    /// `api/system` endpoints
    System,
    /// `api/auth` endpoints. Mengarah ke servis [Auth].
    Auth,
    /// `api/$param.service_name_snake_case$` endpoints. Mengarah ke servis [$param.service_name_pascal_case$].
    $param.service_name_pascal_case$,
    /// Gunakan ini apabila ada servis khusus (user).
    Service(&'static str),
}

impl fmt::Display for ApiKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiKind::System => write!(f, "api/system"),
            ApiKind::Auth => write!(f, "api/auth"),
            ApiKind::$param.service_name_pascal_case$ => write!(f, "api/$param.service_name_snake_case$"),
            ApiKind::Service(name) => write!(f, "api/{}", name),
        }
    }
}

#[derive(Clone)]
pub struct TestKit {}

impl TestKit {
    pub fn new() -> Self {
        Self {}
    }

    pub fn api(&self) -> TestKitApi {
        TestKitApi::new(self)
    }

    pub fn helper(&self) -> TestHelper {
        TestHelper::new(self)
    }

    pub fn api_helper(&self) -> ApiHelper {
        ApiHelper::new(self)
    }
}

pub struct TestKitApi {
    testkit: TestKit,
    test_server: TestServer,
    test_client: Client,
}

impl TestKitApi {
    pub fn new(testkit: &TestKit) -> Self {
        TestKitApi {
            testkit: testkit.clone(),
            test_server: create_test_server(),
            test_client: Client::new(),
        }
    }

    /// Creates a requests builder for the public API scope.
    pub fn public(&self, kind: impl fmt::Display) -> RequestBuilder {
        RequestBuilder::new(
            self.test_server.url(""),
            &self.test_client,
            ApiAccess::Public,
            kind.to_string(),
        )
    }

    /// Creates a requests builder for the private API scope.
    pub fn private(&self, kind: impl fmt::Display) -> RequestBuilder {
        RequestBuilder::new(
            self.test_server.url(""),
            &self.test_client,
            ApiAccess::Private,
            kind.to_string(),
        )
    }

    /// Cara pintas untuk meng-otorisasi $param.service_name_pascal_case$,
    /// atau dengan kata lain me-login-kan sehingga
    /// nanti http client akan meng-embed X-Access-Token secara otomatis.
    pub fn authorize(&mut self, $param.service_name_snake_case$_id: ID) {
        let mut headers = HeaderMap::new();
        let token = self
            .testkit
            .helper()
            .gen_access_token_for($param.service_name_snake_case$_id)
            .expect("Cannot generate access token");
        headers.insert("X-Access-Token", HeaderValue::from_str(&token.token).unwrap());
        self.test_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Cannot build http client");
    }

    /// Assert json result from API,
    /// akan gagal apabila hasil dari ApResult berisi kode error / status="error".
    pub fn assert_success(&self, rv: &serde_json::Value) {
        assert_eq!(rv, &json!({"code": 0, "status":"success", "description":""}));
    }
}

/// An HTTP requests builder. This type can be used to send requests to
/// the appropriate `TestKitApi` handlers.
pub struct RequestBuilder<'a, 'b, Q = ()>
where
    Q: 'b,
{
    test_server_url: String,
    test_client: &'a Client,
    access: ApiAccess,
    prefix: String,
    query: Option<&'b Q>,
}

impl<'a, 'b, Q> fmt::Debug for RequestBuilder<'a, 'b, Q>
where
    Q: 'b + fmt::Debug + Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("RequestBuilder")
            .field("access", &self.access)
            .field("prefix", &self.prefix)
            .field("query", &self.query)
            .finish()
    }
}

impl<'a, 'b, Q> RequestBuilder<'a, 'b, Q>
where
    Q: 'b + Serialize,
{
    fn new(test_server_url: String, test_client: &'a Client, access: ApiAccess, prefix: String) -> Self {
        RequestBuilder {
            test_server_url,
            test_client,
            access,
            prefix,
            query: None,
        }
    }

    /// Sets a query data of the current request.
    pub fn query<T>(&'a self, query: &'b T) -> RequestBuilder<'a, 'b, T> {
        RequestBuilder {
            test_server_url: self.test_server_url.clone(),
            test_client: self.test_client,
            access: self.access,
            prefix: self.prefix.clone(),
            query: Some(query),
        }
    }

    /// Sends a get request to the testing API endpoint and decodes response as
    /// the corresponding type.
    pub fn get<R>(&self, endpoint: &str) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        let params = self
            .query
            .as_ref()
            .map(|query| {
                format!(
                    "?{}",
                    serde_urlencoded::to_string(query).expect("Unable to serialize query.")
                )
            })
            .unwrap_or_default();
        let url = format!(
            "{url}{access}/{prefix}/{endpoint}{query}",
            url = self.test_server_url,
            access = format!("{}", self.access).to_lowercase(),
            prefix = self.prefix,
            endpoint = endpoint,
            query = params
        );

        trace!("GET {}", url);

        let response = self.test_client.get(&url).send().expect("Unable to send request");
        Self::response_to_api_result(response)
    }

    /// Sends a post request to the testing API endpoint and decodes response as
    /// the corresponding type.
    pub fn post<R>(&self, endpoint: &str) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        let url = format!(
            "{url}{access}/{prefix}/{endpoint}",
            url = self.test_server_url,
            access = format!("{}", self.access).to_lowercase(),
            prefix = self.prefix,
            endpoint = endpoint
        );

        trace!("POST {}", url);

        let builder = self.test_client.post(&url);
        let builder = if let Some(ref query) = self.query.as_ref() {
            trace!("Body: {}", serde_json::to_string_pretty(&query).unwrap());
            builder.json(query)
        } else {
            builder.json(&serde_json::Value::Null)
        };
        let response = builder.send().expect("Unable to send request");
        Self::response_to_api_result(response)
    }

    /// Converts reqwest Response to api::Result.
    fn response_to_api_result<R>(mut response: Response) -> api::Result<R>
    where
        R: DeserializeOwned + 'static,
    {
        trace!("Response status: {}", response.status());

        fn extract_description(body: &str) -> Option<String> {
            trace!("Error: {}", body);
            match serde_json::from_str::<serde_json::Value>(body).ok()? {
                serde_json::Value::Object(ref object) if object.contains_key("description") => {
                    Some(object["description"].as_str()?.to_owned())
                }
                serde_json::Value::String(string) => Some(string),
                _ => None,
            }
        }

        fn error(mut response: Response) -> String {
            let body = response.text().expect("Unable to get response text");
            extract_description(&body).unwrap_or(body)
        }

        match response.status() {
            StatusCode::OK => Ok({
                let body = response.text().expect("Unable to get response text");
                eprintln!("error body: {}", body);
                serde_json::from_str(&body).expect("Unable to deserialize body")
            }),
            StatusCode::FORBIDDEN => Err(api::Error::Unauthorized),
            StatusCode::BAD_REQUEST => Err(api::Error::BadRequest(400, error(response))),
            StatusCode::NOT_FOUND => Err(api::Error::NotFound(404, error(response))),
            s if s.is_server_error() => {
                Err(api::Error::InternalError(500, format_err!("{}", error(response))))
            }
            s => {
                let body = response.text().expect("Unable to get response text");
                eprintln!("error body: {}", body);
                panic!(
                    "Received non-error response status: {} ({})",
                    s.as_u16(),
                    error(response)
                )
            }
        }
    }
}

pub fn setup() {
    let _ = env_logger::try_init();
}

pub fn create_test_server() -> TestServer {
    setup();

    let service = service::$param.service_name_pascal_case$Service::new();
    let system_service = service::SystemService::new();

    let agg = ApiAggregator::new(vec![service, system_service]);

    let server = TestServer::with_factory(move || {
        let state = api::AppState::new();
        App::with_state(state.clone())
            .scope("public/api", |scope| {
                trace!("Create public API");
                agg.extend(ApiAccess::Public, scope)
            })
            .scope("private/api", |scope| {
                trace!("Create private API");
                agg.extend(ApiAccess::Private, scope)
            })
    });

    info!("Test server created on {}", server.addr());

    server
}
