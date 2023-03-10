use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest},
    Request, ServerError, ServerResult,
};
use axum::{async_trait, http::HeaderMap};

pub struct HeaderMiddleware;

pub struct AuthMiddleware;

impl ExtensionFactory for HeaderMiddleware {
    fn create(&self) -> std::sync::Arc<dyn Extension> {
        Arc::new(HeaderMiddleware)
    }
}

impl ExtensionFactory for AuthMiddleware {
    fn create(&self) -> std::sync::Arc<dyn Extension> {
        Arc::new(AuthMiddleware)
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
        let headers = ctx.data::<HeaderMap>().unwrap();

        if let Some(header_key) = headers.get("X-APP-REVDESINA-RS") {
            if let Ok(val) = header_key.to_str() {
                if val != "some-rust-graphql-api" {
                    return Err(ServerError::new("header key is invalid", None));
                }
            }
        } else {
            return Err(ServerError::new("header key is missing", None));
        }

        next.run(ctx, request).await
    }
}

#[async_trait]
impl Extension for AuthMiddleware {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let headers = ctx.data::<HeaderMap>().unwrap();

        if headers.get("authToken").is_none() {
            return Err(ServerError::new("authToken is required", None));
        }

        next.run(ctx, request).await
    }
}
