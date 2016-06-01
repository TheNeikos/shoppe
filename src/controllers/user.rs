
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;
use params::Params;
use diesel::{self, ExecuteDsl};

use error::NotImplemented;
use views;
use models;
use database;

pub fn index(req: &mut Request) -> IronResult<Response> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    let user_list = users.limit(10).load::<models::user::User>(&*database::connection().get().unwrap())
        .expect("Error loading users");

    let mut resp = Response::with((status::Ok, views::user::index(&user_list).unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, views::user::new(None).unwrap()));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use models::schema::users;

    let map = req.get_ref::<Params>().unwrap();

    let username = match map.get("user_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let email = match map.get("user_email") {
        Some(&Value::String(ref email)) => Some(&email[..]),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(&pass[..]),
        _ => None
    };

    let new_user = match models::user::NewUser::new(username, email, password) {
        Ok(new_user) => new_user,
        Err(err) => {
            let mut resp = Response::with((status::Ok, views::user::new(Some(err)).unwrap()));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    diesel::insert(&new_user).into(users::table)
        .execute(&*database::connection().get().unwrap()).expect("Error saving new user");

    // TODO: Add config for url?
    return Ok(Response::with((status::SeeOther, Redirect(Url::parse("http://localhost:3000/users/").unwrap()))))
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


