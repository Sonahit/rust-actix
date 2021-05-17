use actix_http::{http::header, Payload};
use actix_web::dev::{JsonBody, UrlEncoded};
use actix_web::web::{FormConfig, JsonConfig};
use actix_web::*;
use futures_util::future::{err, FutureExt, LocalBoxFuture};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Serialize, Deserialize)]
pub struct BodyExtractor<T>(pub T);

impl<T> BodyExtractor<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> ops::Deref for BodyExtractor<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for BodyExtractor<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromRequest for BodyExtractor<T>
where
    T: DeserializeOwned + 'static,
{
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Error>>;
    type Config = BodyConfig;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req1 = req.clone();
        let content_type = req1
            .headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap_or("default");
        let returns = match content_type {
            "application/json" => {
                let req2 = req.clone();
                let config = BodyConfig::from_req_json(req);
                let JsonConfig {
                    limit,
                    content_type,
                    err_handler,
                } = config;
                JsonBody::new(req, payload, content_type.clone())
                    .limit(limit.clone())
                    .map(move |res| match res {
                        Err(e) => {
                            log::debug!(
                                "Failed to deserialize Json from payload. \
                             Request path: {}",
                                req2.path()
                            );
                            if let Some(err) = err_handler {
                                Err((*err)(e, &req2))
                            } else {
                                Err(e.into())
                            }
                        }
                        Ok(data) => Ok(BodyExtractor(data)),
                    })
                    .boxed_local()
            }
            "application/x-www-form-urlencoded" => {
                let req2 = &req.clone();
                let config = BodyConfig::from_req_urlencoded(&req2);
                let FormConfig { limit, err_handler } = config;
                UrlEncoded::new(req, payload)
                    .limit(limit.clone())
                    .map(move |res| match res {
                        Err(e) => {
                            if let Some(err) = err_handler {
                                Err((*err)(e, &req2))
                            } else {
                                Err(e.into())
                            }
                        }
                        Ok(item) => Ok(BodyExtractor(item)),
                    })
                    .boxed_local()
            }
            _ => Box::pin(err(error::ErrorNotImplemented("Wrong content type"))),
        };
        returns
    }
}

#[derive(Clone)]
pub struct BodyConfig {
    json: JsonConfig,
    urlencoded: FormConfig,
}

impl BodyConfig {
    fn from_req_json(req: &HttpRequest) -> &JsonConfig {
        req.app_data::<JsonConfig>()
            .or_else(|| req.app_data::<web::Data<JsonConfig>>().map(|d| d.as_ref()))
            .unwrap_or(&JsonConfig::default())
    }

    fn from_req_urlencoded(req: &HttpRequest) -> &FormConfig {
        req.app_data::<FormConfig>()
            .or_else(|| req.app_data::<web::Data<FormConfig>>().map(|d| d.as_ref()))
            .unwrap_or(&FormConfig::default())
    }
}

impl Default for BodyConfig {
    fn default() -> Self {
        BodyConfig {
            json: JsonConfig::default(),
            urlencoded: FormConfig::default(),
        }
    }
}
