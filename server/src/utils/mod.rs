macro_rules! json_route {
    ($method:ident) => {
        actix_web::web::$method()
            .guard(actix_web::guard::Header("Content-Type", "application/json"))
    };
}

macro_rules! urlencoded_route {
    ($method:ident) => {
        actix_web::web::$method().guard(actix_web::guard::Header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        ))
    };
}

#[allow(unused_macros)]
macro_rules! json_post {
    () => {
        json_route!(post)
    };
}

#[allow(unused_macros)]
macro_rules! json_patch {
    () => {
        json_route!(patch)
    };
}

#[allow(unused_macros)]
macro_rules! json_put {
    () => {
        json_route!(put)
    };
}
#[allow(unused_macros)]
macro_rules! json_delete {
    () => {
        json_route!(put)
    };
}

#[allow(unused_macros)]
macro_rules! urlencoded_post {
    () => {
        urlencoded_route!(post)
    };
}

#[allow(unused_macros)]
macro_rules! urlencoded_patch {
    () => {
        urlencoded_route!(patch)
    };
}
#[allow(unused_macros)]
macro_rules! urlencoded_put {
    () => {
        urlencoded_route!(put)
    };
}

#[allow(unused_macros)]
macro_rules! urlencoded_delete {
    () => {
        urlencoded_route!(put)
    };
}
