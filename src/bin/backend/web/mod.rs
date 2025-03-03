mod auth_apis;
mod doc;
mod feature_apis;
mod logout_api;
mod middlewares;
mod push_apis;
mod register_api;
mod room_apis;
mod utils;

use std::{str::FromStr, sync::Arc};

use aide::{
    axum::ApiRouter,
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    clonable_wrapper::ClonableWrapper, config::WebConfig, types::WebFeatureDyn, AppResult,
};
use axum::{
    http::{header::CONTENT_TYPE, HeaderName, HeaderValue, Method, StatusCode, Uri},
    Extension, Json,
};

use self::{
    doc::docs_routes, middlewares::auth_middleware::set_username_from_token_in_request_middleware,
};

pub struct WebTask {
    pub config: WebConfig,
    tcp: tokio::net::TcpListener,
    features: Vec<ClonableWrapper<WebFeatureDyn>>,
    router: ApiRouter,
}

impl WebTask {
    pub async fn create(
        mut config: WebConfig,
        features: Vec<ClonableWrapper<WebFeatureDyn>>,
    ) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        config.port = addr.port();
        let router = ApiRouter::new();
        Ok(Self {
            config,
            tcp,
            features,
            router,
        })
    }

    pub async fn run(mut self) -> AppResult {
        tracing_subscriber::fmt::init();

        aide::gen::on_error(|error| {
            println!("{error}");
        });

        aide::gen::extract_schemas(true);

        let mut api = OpenApi::default();

        // authentication routes
        self.router = self
            .router
            .fallback(|uri: Uri| async move {
                (StatusCode::NOT_FOUND, Json(format!("No route for {uri}")))
            })
            .nest_api_service("/auth/iot", auth_apis::iot_auth_routes())
            .nest_api_service("/auth/web", auth_apis::web_auth_routes())
            .nest_api_service("/auth/logout", logout_api::logout_routes())
            .nest_api_service("/auth/register", register_api::register_routes())
            .nest_api_service("/api/push-credential", push_apis::push_routes())
            .nest_api_service("/api/features", feature_apis::features_route())
            .nest_api_service("/api/rooms", room_apis::room_routes());

        for feat in &mut self.features {
            self.router = self.router.nest_api_service(
                format!("/api/{}", feat.clone().get_module_name()).as_str(),
                feat.clone().create_router(),
            )
        }

        let router = self
            .router
            .nest_api_service("/docs", docs_routes())
            .finish_api_with(&mut api, WebTask::api_docs)
            .layer(Extension(Arc::new(api)))
            .layer(axum::middleware::from_fn(
                set_username_from_token_in_request_middleware,
            ))
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT, Method::DELETE])
                    .allow_credentials(true)
                    .allow_headers([CONTENT_TYPE, HeaderName::from_str("jwt").unwrap()])
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()),
            ) // TODO: Whitelist additional origins
            .into_make_service();
        tokio::spawn(async move {
            println!(
                "Web server ready to server on {}://{}:{}",
                self.config.protocol, self.config.addr, self.config.port
            );
            println!(
                "Check web server doc at {}://{}:{}/docs",
                self.config.protocol, self.config.addr, self.config.port
            );
            axum::serve(self.tcp, router).await
        });

        Ok(())
    }

    fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
        api.title("Tempusalert Open API")
            .summary("Crates for server apps in the backend: the IoT server, the MQTT broker and the Web server")
            .tag(Tag {
                name: "tempusalert".into(),
                description: Some("Smart Home Guard".into()),
                ..Default::default()
            })
            .security_scheme(
                "ApiKey",
                aide::openapi::SecurityScheme::ApiKey {
                    location: aide::openapi::ApiKeyLocation::Header,
                    name: "X-Auth-Key".into(),
                    description: Some("A key that is ignored.".into()),
                    extensions: Default::default(),
                },
            )
    }
}
