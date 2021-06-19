use actix_http::{http::header, Payload};
use actix_web::dev::{JsonBody, UrlEncoded};
use actix_web::web::{FormConfig, JsonConfig};
use actix_web::*;
use futures_util::future::{err, FutureExt, LocalBoxFuture};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ops;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct BodyExtractor<T>(pub T);

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
        let limit = 4086;
        let mut content_type = req.headers().get(header::CONTENT_TYPE).take();
        let content_type = match content_type.is_some() {
            true => Ok(content_type.take().unwrap().to_str().unwrap()),
            _ => Err("No mime type"),
        };
        match content_type {
            Ok("application/json") => {
                let req2 = req.clone().to_owned();
                JsonBody::new(&req2.clone(), payload, Some(Arc::from(|_| true)))
                    .limit(limit)
                    .map(move |res| match res {
                        Err(e) => {
                            log::debug!(
                                "Failed to deserialize Json from payload. \
                             Request path: {}",
                                (req2).path()
                            );
                            Err(e.into())
                        }
                        Ok(data) => Ok(BodyExtractor(data)),
                    })
                    .boxed_local()
            }
            Ok("application/x-www-form-urlencoded") => {
                let req2 = req.clone().to_owned();

                UrlEncoded::new(&req2.clone(), payload)
                    .limit(limit)
                    .map(move |res| match res {
                        Err(e) => Err(e.into()),
                        Ok(item) => Ok(BodyExtractor(item)),
                    })
                    .boxed_local()
            }
            _ => Box::pin(err(error::ErrorNotImplemented("Wrong content type"))),
        }
    }
}

#[derive(Clone)]
pub struct BodyConfig {
    json: JsonConfig,
    urlencoded: FormConfig,
}

impl Default for BodyConfig {
    fn default() -> Self {
        BodyConfig {
            json: JsonConfig::default(),
            urlencoded: FormConfig::default(),
        }
    }
}
