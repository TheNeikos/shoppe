
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;
use iron::Url;
use diesel::{self, ExecuteDsl};

use error::{self};
use views;
use models;
use database;

pub fn index(_req: &mut Request) -> IronResult<Response> {
    let user_list = try!(models::user::find_all());

    let mut resp = Response::with((status::Ok, template!(views::user::index(&user_list))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn new(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::with((status::Ok, template!(views::user::new(None))));
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
            let mut resp = Response::with((status::Ok, template!(views::user::new(Some(err)))));
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
    use router::Router;

    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let mut resp = Response::with((status::Ok, template!(views::user::show(&user))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn edit(req: &mut Request) -> IronResult<Response> {
    use router::Router;
    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let mut resp = Response::with((status::Ok, template!(views::user::edit(&user, None))));
    resp.headers.set(ContentType::html());
    Ok(resp)
}

pub fn update(req: &mut Request) -> IronResult<Response> {
    use params::{Params, Value};
    use router::Router;


    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    let map = req.get_ref::<Params>().unwrap();
    let username = match map.get("user_name") {
        Some(&Value::String(ref name)) => Some(&name[..]),
        _ => None
    };

    let password = match map.get("user_password") {
        Some(&Value::String(ref pass)) => Some(&pass[..]),
        _ => None
    };

    let update_user = match models::user::UpdateUser::new(username, password) {
        Ok(update_user) => update_user,
        Err(err) => {
            let mut resp = Response::with((status::Ok, template!(views::user::edit(&user, Some(err)))));
            resp.headers.set(ContentType::html());
            return Ok(resp);
        }
    };

    try!(user.update(&update_user));

    return Ok(Response::with((status::SeeOther, Redirect(url!(format!("/users/{}", user.id))))))
}

pub fn delete(req: &mut Request) -> IronResult<Response> {
    use router::Router;

    let id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(t) => {
            match t.parse::<_>() {
                Ok(t) => t,
                Err(_) => return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")))
            }
        }
        None => {
            return Err(IronError::new(error::BadFormattingError::new(), temp_redirect!("/users/")));
        }
    };

    let user = match try!(models::user::find(id)) {
        Some(u) => u,
        None => {
            let mut resp = Response::with(status::NotFound);
            resp.headers.set(ContentType::html());
            return Ok(resp)
        }
    };

    try!(user.delete());

    return Ok(Response::with((status::SeeOther, Redirect(url!("/users")))))
}

