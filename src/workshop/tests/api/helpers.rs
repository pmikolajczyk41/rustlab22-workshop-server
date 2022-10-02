pub mod people {
    use workshop::swapi::Person;

    pub fn luke() -> Person {
        Person {
            name: "Luke Skywalker".to_string(),
            height: "172".to_string(),
        }
    }

    pub fn yoda() -> Person {
        Person {
            name: "Yoda".to_string(),
            height: "66".to_string(),
        }
    }

    pub fn yaddle() -> Person {
        Person {
            name: "Yaddle".to_string(),
            height: "61".to_string(),
        }
    }

    pub fn arvel() -> Person {
        Person {
            name: "Arvel Crynyd".to_string(),
            height: "unknown".to_string(),
        }
    }
}

pub mod swapi_mock {
    use std::time::Duration;

    use serde_json::{json, Value};
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use workshop::swapi::Person;

    pub struct SwapiMock {
        server: MockServer,
    }

    impl SwapiMock {
        pub async fn start() -> Self {
            let server = MockServer::start().await;
            SwapiMock { server }
        }

        fn mock_search(name: &str, response: ResponseTemplate) -> Mock {
            Mock::given(method("GET"))
                .and(path("/api/people/"))
                .and(query_param("search", name))
                .respond_with(response)
        }

        pub async fn mock_people_query(&self, name: &str, response: Value) {
            let response = ResponseTemplate::new(200).set_body_json(response);
            self.server
                .register(Self::mock_search(name, response))
                .await;
        }

        pub async fn mock_people_query_with_delay(
            &self,
            name: &str,
            response: Value,
            delay: Duration,
        ) {
            let response = ResponseTemplate::new(200)
                .set_body_json(response)
                .set_delay(delay);
            self.server
                .register(Self::mock_search(name, response))
                .await;
        }

        pub fn uri(&self) -> String {
            self.server.uri()
        }
    }

    pub fn person_query_result(person: &Person) -> Value {
        json!({ "results": [person] })
    }

    pub fn empty_query_result() -> Value {
        json!({
            "results": []
        })
    }
}

pub mod test_app {
    use std::time::Duration;

    use reqwest::Response;

    use workshop::server::startup::Application;
    use workshop::settings::{ApplicationSettings, Settings, SwapiSettings};
    use workshop::swapi::SwapiClient;
    use workshop::taller::YodaTaller;

    use crate::helpers::swapi_mock::SwapiMock;

    pub const SWAPI_TIMEOUT: Duration = Duration::from_secs(2);

    pub struct TestApp {
        pub swapi_server: SwapiMock,
        pub swapi_client: SwapiClient,
        pub yoda_taller: YodaTaller,
        pub port: u16,
    }

    impl TestApp {
        pub async fn spawn() -> Self {
            let swapi_server = SwapiMock::start().await;
            let swapi_client = SwapiClient::new(swapi_server.uri(), SWAPI_TIMEOUT).unwrap();
            let yoda_taller = YodaTaller {
                client: swapi_client.clone(),
            };

            let settings = Settings {
                application: ApplicationSettings { port: 0 },
                swapi: SwapiSettings {
                    base_url: swapi_server.uri(),
                    timeout_milliseconds: SWAPI_TIMEOUT.as_millis() as u32,
                },
            };
            let app = Application::bind(settings).unwrap();
            let port = app.tcp_listener().local_addr().unwrap().port();
            let _ = tokio::spawn(app.run());

            TestApp {
                swapi_client,
                swapi_server,
                yoda_taller,
                port,
            }
        }

        pub fn server_address(&self) -> String {
            format!("http://127.0.0.1:{}", self.port)
        }

        pub async fn send_taller_req(&self, name: &str) -> Response {
            reqwest::Client::new()
                .get(&format!("{}/taller/{}", self.server_address(), name))
                .send()
                .await
                .expect("Failed to execute request.")
        }
    }
}
