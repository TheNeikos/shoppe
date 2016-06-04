

#[macro_export]
macro_rules! resource {
    ($name:ident) => {{
        use $crate::router::Router;
        let mut router = Router::new();
        router.get("/",         $name::index);
        router.get("/new",      $name::new);
        router.post("/",        $name::create);
        router.get("/:id",      $name::show);
        router.get("/:id/edit", $name::edit);
        router.put("/:id",      $name::update);
        router.post("/:id",      $name::update);
        router.delete("/:id",   $name::delete);
        router
    }}
}

#[macro_export]
macro_rules! template {
    ($fun:expr) => {{
        use error;
        try!(match $fun {
            ::std::result::Result::Ok(val) => ::std::result::Result::Ok(val),
            ::std::result::Result::Err(err) => {
                let e : error::TemplateError = ::std::convert::From::from(err);
                ::std::result::Result::Err(e)
            }
        })
    }}
}

#[macro_export]
macro_rules! database_try {
    ($fun:expr) => {{
        use error;
        match $fun {
            ::std::result::Result::Ok(val) => ::std::result::Result::Ok(val),
            ::std::result::Result::Err(err) => {
                let e : error::DatabaseError = ::std::convert::From::from(err);
                ::std::result::Result::Err(e)
            }
        }
    }}
}

#[macro_export]
macro_rules! temp_redirect {
    ($url:expr) => {
        (status::SeeOther, Redirect(url!($url)))
    }
}

#[macro_export]
macro_rules! url {
    ($url:expr) => {{use iron::Url; Url::parse(&(format!("http://localhost:3000{}", $url)[..])).unwrap() }}
}
