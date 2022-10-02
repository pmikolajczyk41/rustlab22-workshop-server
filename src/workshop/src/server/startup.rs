use std::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;

use axum::{routing::get, Extension, Router};

use crate::{
    server::taller_route::taller, settings::Settings, swapi::SwapiClient, taller::YodaTaller,
};

pub struct Application {
    pub settings: Settings,
    tcp_listener: TcpListener,
}

impl Application {
    pub fn bind(settings: Settings) -> anyhow::Result<Self> {
        let address = format!("127.0.0.1:{}", settings.application.port);
        let tcp_listener = TcpListener::bind(address).unwrap();

        Ok(Application {
            settings,
            tcp_listener,
        })
    }

    pub fn tcp_listener(&self) -> &TcpListener {
        &self.tcp_listener
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let client = SwapiClient::new(
            self.settings.swapi.base_url.clone(),
            Duration::from_millis(self.settings.swapi.timeout_milliseconds as u64),
        )
        .unwrap();
        let yoda_taller = Arc::new(YodaTaller { client });

        let app = Router::new()
            .route("/health_check", get(|| async {}))
            .route("/taller/:name", get(taller))
            .layer(Extension(yoda_taller));
        let server = axum::Server::from_tcp(self.tcp_listener).unwrap();
        server.serve(app.into_make_service()).await.unwrap();
        Ok(())
    }
}
