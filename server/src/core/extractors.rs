use actix_http::{http::header, Payload};
use actix_web::dev::{JsonBody, UrlEncoded};
use actix_web::error::{JsonPayloadError, UrlencodedError};
use actix_web::web::{FormConfig, JsonConfig};
use actix_web::*;
use futures_util::future::{err, FutureExt, LocalBoxFuture};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ops;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone)]
struct PubJsonConfig {
    limit: usize,
    err_handler: Option<Arc<dyn Fn(JsonPayloadError, &HttpRequest) -> Error + Send + Sync>>,
}
#[derive(Clone)]
pub struct PubFormConfig {
    limit: usize,
    err_handler: Option<Rc<dyn Fn(UrlencodedError, &HttpRequest) -> Error>>,
}

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
        let req2 = req.clone();
        let mut content_type = req.headers().get(header::CONTENT_TYPE).take();
        let content_type = match content_type.is_some() {
            true => Ok(content_type.take().unwrap().to_str().unwrap()),
            _ => Err("No mime type"),
        };
        match content_type {
            Ok("application/json") => {
                let PubJsonConfig {
                    limit, err_handler, ..
                } = BodyConfig::get_json_config(req2.clone());
                JsonBody::new(&req2, payload, Some(Arc::from(|_| true)))
                    .limit(limit)
                    .map(move |res| match res {
                        Err(e) => {
                            log::debug!(
                                "Failed to deserialize Json from payload. \
                             Request path: {}",
                                (req2).path()
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
            Ok("application/x-www-form-urlencoded") => {
                let PubFormConfig { limit, err_handler } =
                    BodyConfig::get_form_config(req2.clone());

                UrlEncoded::new(&req2, payload)
                    .limit(limit)
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
        }
    }
}

#[derive(Clone)]
pub struct BodyConfig {
    json: PubJsonConfig,
    urlencoded: PubFormConfig,
}

impl BodyConfig {
    fn get_json_config(req: HttpRequest) -> PubJsonConfig {
        unsafe {
            match std::mem::transmute::<Option<&JsonConfig>, Option<&PubJsonConfig>>(
                req.app_data::<JsonConfig>(),
            ) {
                Some(d) => d.clone(),
                None => std::mem::transmute(BodyConfig::default().json),
            }
        }
    }

    fn get_form_config(req: HttpRequest) -> PubFormConfig {
        unsafe {
            match std::mem::transmute::<Option<&FormConfig>, Option<&PubFormConfig>>(
                req.app_data::<FormConfig>(),
            ) {
                Some(d) => d.clone(),
                None => std::mem::transmute(BodyConfig::default().urlencoded),
            }
        }
    }
}

impl Default for BodyConfig {
    fn default() -> Self {
        BodyConfig {
            json: PubJsonConfig {
                limit: 32_768, // 2^15 bytes, (~32kB)
                err_handler: None,
            },
            urlencoded: PubFormConfig {
                limit: 16_384, // 2^14 bytes (~16kB)
                err_handler: None,
            },
        }
    }
}
