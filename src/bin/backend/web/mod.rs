use axum::Router;
use tempusalert_be::backend_core::features::WebFeature;
use tokio::join;
use crate::{config::WebConfig, AppResult};

pub struct WebTask {
    pub config: WebConfig,
    tcp: tokio::net::TcpListener,
    features: Vec<Box<dyn WebFeature + Send>>,
    router: Router,
}

impl WebTask {
    pub async fn create(
        mut config: WebConfig,
        features: Vec<Box<dyn WebFeature + Send>>,
    ) -> AppResult<Self> {
        let tcp = tokio::net::TcpListener::bind(config.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;
        config.port = addr.port();
        let router = Router::new();
        Ok(Self {
            config,
            tcp,
            features,
            router,
        })
    }

    pub async fn run(mut self) -> AppResult {
        for feat in &mut self.features {
            self.router = self.router.nest("/", feat.create_router())
        }
        tokio::spawn(async { axum::serve(self.tcp, self.router).await });
        let mut join_handles = vec![];
        for mut feat in self.features {
            join_handles.push(tokio::spawn(async move { feat.run_loop().await }));
        }
        for handle in join_handles {
            handle.await.unwrap()
        }
        Ok(())
    }
}
