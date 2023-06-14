// use crate::api::v1::user::user_model::User;
// use crate::config::constants;
// use crate::exceptions::error::AppError;
// use super::state::AppState;
// use crate::auth::jwt::{validate_token};
// use actix_web::HttpMessage;
// use actix_web::{
//     body::EitherBody,
//     dev::{Service, ServiceRequest, ServiceResponse, Transform},
//     http::Method,
//     web::Data,
//     Error, HttpRequest, HttpResponse,
// };
// use diesel::pg::PgConnection;
// use futures::future::{ok, Ready};
// use futures::Future;
// use serde_json::json;
// use std::pin::Pin;
// use uuid::Uuid;

// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct Authentication;

// // Middleware factory is `Transform` trait from actix-service crate
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S, ServiceRequest> for Authentication
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = AuthenticationMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(AuthenticationMiddleware { service })
//     }
// }

// pub struct AuthenticationMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = Error;

//     #[allow(clippy::type_complexity)] // TODO: want to remove allowness to skip and refactor somehow
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     actix_web::dev::forward_ready!(service);

//     fn call(&self, mut req: ServiceRequest) -> Self::Future {
//         let is_verified = if should_skip_auth(&req) {
//             true
//         } else {
//             set_auth_user(&mut req)
//         };
//         if is_verified {
//             let fut = self.service.call(req);
//             Box::pin(async move {
//                 let res = fut.await?.map_into_left_body();
//                 Ok(res)
//             })
//         } else {
//             Box::pin(async move {
//                 let (req, _res) = req.into_parts();
//                 let res = HttpResponse::Unauthorized().finish().map_into_right_body();
//                 let srv = ServiceResponse::new(req, res);
//                 Ok(srv)
//             })
//         }
//     }
// }

// fn should_skip_auth(req: &ServiceRequest) -> bool {
//     let method = req.method();
//     if Method::OPTIONS == *method {
//         return true;
//     }

//     SKIP_AUTH_ROUTES
//         .iter()
//         .any(|route| route.matches_path_and_method(req.path(), req.method()))
// }

// const TOKEN_IDENTIFIER: &str = "Token";

// fn set_auth_user(req: &mut ServiceRequest) -> bool {
//     match fetch_user(req) {
//         Ok(user) => {
//             req.extensions_mut().insert(user);
//             true
//         }
//         Err(err_msg) => {
//             info!("Cannot fetch user {}", err_msg);
//             false
//         }
//     }
// }

// fn fetch_user(req: &ServiceRequest) -> Result<User, &str> {
//     let user_id = get_user_id_from_header(req)?;

//     let conn = &mut req
//         .app_data::<Data<AppState>>()
//         .ok_or("Cannot get state.")
//         .and_then(|state| state.get_conn().map_err(|_err| "Cannot get db connection."))?;

//     find_auth_user(conn, user_id).map_err(|_err| "Cannot find auth user")
// }

// fn get_user_id_from_header(req: &ServiceRequest) -> Result<Uuid, &str> {
//     req.headers()
//         .get(constants::AUTHORIZATION)
//         .ok_or("Cannot find authorization key-value in req header")
//         .and_then(|auth_header| auth_header.to_str().map_err(|_err| "Cannot stringify"))
//         .and_then(|auth_str| {
//             if auth_str.starts_with(TOKEN_IDENTIFIER) {
//                 Ok(auth_str)
//             } else {
//                 Err("Invalid token convention")
//             }
//         })
//         .map(|auth_str| auth_str[6..auth_str.len()].trim())
//         .and_then(|token| validate_token(token).map_err(|_err| "Cannot decode token."))
//         .map(|token| token.claims.user_id)
// }

// pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
//     req.extensions()
//         .get::<User>()
//         .map(|user| user.to_owned()) // TODO: avoid copy
//         .ok_or_else(|| {
//             AppError::Unauthorized(json!({"error": "Unauthrized user. Need auth token on header."}))
//         })
// }

// struct SkipAuthRoute {
//     path: &'static str,
//     method: Method,
// }

// impl SkipAuthRoute {
//     fn matches_path_and_method(&self, path: &str, method: &Method) -> bool {
//         self.matches_path(path) && self.matches_method(method)
//     }

//     fn matches_path(&self, path: &str) -> bool {
//         let expect_path = self.path.split('/').collect::<Vec<_>>();
//         let this_path = path.split('/').collect::<Vec<_>>();
//         if expect_path.len() != this_path.len() {
//             return false;
//         };
//         let path_set = expect_path.iter().zip(this_path.iter());
//         for (expect_path, this_path) in path_set {
//             if SkipAuthRoute::is_slug_path(expect_path) {
//                 continue;
//             }
//             if expect_path != this_path {
//                 return false;
//             }
//         }
//         true
//     }

//     fn matches_method(&self, method: &Method) -> bool {
//         self.method == method
//     }

//     fn is_slug_path(text: &str) -> bool {
//         let first = text.chars().next().unwrap_or(' ');
//         let last = text.chars().last().unwrap_or(' ');
//         first == '{' && last == '}'
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::http::Method;
//     #[test]
//     fn is_match_path_and_method_test() {
//         let route = SkipAuthRoute {
//             path: "/api/healthcheck",
//             method: Method::GET,
//         };
//         assert!(route.matches_path_and_method("/api/healthcheck", &Method::GET));

//         let route = SkipAuthRoute {
//             path: "/api/{this-is-slug}/healthcheck",
//             method: Method::POST,
//         };
//         assert!(route.matches_path_and_method("/api/1234/healthcheck", &Method::POST));
//     }
// }

// const SKIP_AUTH_ROUTES: [SkipAuthRoute; 6] = [
//     SkipAuthRoute {
//         path: "/api/healthcheck",
//         method: Method::GET,
//     },
//     SkipAuthRoute {
//         path: "/api/tags",
//         method: Method::GET,
//     },
//     SkipAuthRoute {
//         path: "/api/users",
//         method: Method::POST,
//     },
//     SkipAuthRoute {
//         path: "/api/users/login",
//         method: Method::POST,
//     },
//     SkipAuthRoute {
//         path: "/api/articles",
//         method: Method::GET,
//     },
//     SkipAuthRoute {
//         path: "/api/articles/{article_title_slug}/comments",
//         method: Method::GET,
//     },
// ];

// // ================
// // TODO: should inject this func
// fn find_auth_user(conn: &mut PgConnection, user_id: Uuid) -> Result<User, AppError> {
//     let user = User::find(conn, user_id)?;
//     Ok(user)
// }
