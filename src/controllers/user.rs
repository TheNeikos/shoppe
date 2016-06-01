
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;
use params::Params;

use error::NotImplemented;
use views;

pub fn index(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, views::user::new(None).unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    let username = match map.get("user_email") {
        Some(&Value::String(ref name)) => Some(name),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(pass),
        _ => None
    };

    println!("{:#?}", map);

    if username.is_none() || password.is_none() {
        let mut resp = Response::with((status::Ok, views::user::new(Some((username, password))).unwrap()));
        resp.headers.set(ContentType::html());
        return Ok(resp);
    }

    // TODO: Add ccnfig for url?
    return Ok(Response::with((Redirect(Url::parse("http://localhost:3000/").unwrap()),)))
}

pub fn show(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    Err(IronError::new(NotImplemented::new(req), status::NotImplemented))
}


