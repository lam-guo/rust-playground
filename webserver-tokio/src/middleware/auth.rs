use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::prelude::future::LocalBoxFuture;
use std::future::{ready, Ready};

#[derive(Clone)]
pub struct AuthService;

impl<S, B> Transform<S, ServiceRequest> for AuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticatedMiddleware {
            service: std::rc::Rc::new(service),
        }))
    }
}

pub struct AuthenticatedMiddleware<S> {
    service: std::rc::Rc<S>,
}

async fn authenticate() -> bool {
    //TODO 你的逻辑
    return true;
}

impl<S, B> Service<ServiceRequest> for AuthenticatedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            if let false = authenticate().await {
                return Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .body("invalid token")
                        .map_into_right_body(),
                ));
            }

            Ok(service.call(req).await?.map_into_left_body())
        })
    }
}
