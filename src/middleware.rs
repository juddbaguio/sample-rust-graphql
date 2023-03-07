use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest},
    Request, ServerError, ServerResult,
};
use axum::{async_trait, http::HeaderMap};

pub struct HeaderMiddleware;

#[derive(Debug, Default, Clone)]
pub struct AuthHeaderContainer {
    x_api_header: Option<String>,
}

impl AuthHeaderContainer {
    pub fn extract(headers: &HeaderMap) -> AuthHeaderContainer {
        let mut auth_header_container = AuthHeaderContainer::default();
        if let Some(token) = headers.get("X-APP-REVDESINA-RUST") {
            if token.to_str().is_ok() {
                auth_header_container.x_api_header = Some(token.to_str().unwrap().to_string());
            }
        }

        auth_header_container
    }
}

impl ExtensionFactory for HeaderMiddleware {
    fn create(&self) -> std::sync::Arc<dyn Extension> {
        Arc::new(HeaderMiddleware)
    }
}

#[async_trait]
impl Extension for HeaderMiddleware {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let default_headers = AuthHeaderContainer::default();
        let headers = ctx
            .data::<AuthHeaderContainer>()
            .unwrap_or(&default_headers);

        if let Some(val) = &headers.x_api_header {
            if *val != "this-is-a-sample-rust-graphql-app" {
                return Err(ServerError::new("header key is invalid", None));
            }
        } else {
            return Err(ServerError::new("header key is missing", None));
        }
        let result = next.run(ctx, request).await;
        result
    }
}
