use actix_web::web;

#[warn(unused_macros)]

macro_rules! json_route {
    ($method:ident) => {
        web::$method().guard(guard::Header("Content-Type", "application/json"))
    };
}

macro_rules! urlencoded_route {
    ($method:ident) => {
        web::$method().guard(guard::Header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        ))
    };
}

macro_rules! json_post {
    () => {
        json_route!(post)
    };
}

macro_rules! json_get {
    () => {
        json_route!(get)
    };
}

macro_rules! json_patch {
    () => {
        json_route!(patch)
    };
}

macro_rules! json_put {
    () => {
        json_route!(put)
    };
}

macro_rules! urlencoded_post {
    () => {
        urlencoded_route!(post)
    };
}

macro_rules! urlencoded_get {
    () => {
        urlencoded_route!(get)
    };
}

macro_rules! urlencoded_patch {
    () => {
        urlencoded_route!(patch)
    };
}

macro_rules! urlencoded_put {
    () => {
        urlencoded_route!(put)
    };
}
