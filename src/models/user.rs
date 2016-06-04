use bcrypt::{hash, DEFAULT_COST};

use models::schema::users;
use database;
use models;
use error;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub name: String,
}


#[insertable_into(users)]
pub struct NewUser<'a> {
    email: &'a str,
    password_hash: String,
    name: &'a str,
}

#[derive(Debug)]
pub struct UserError {
    pub email: Vec<&'static str>,
    pub password: Vec<&'static str>,
    pub name: Vec<&'static str>,
}

impl UserError {
    fn has_any_errors(&self) -> bool {
        !(self.email.is_empty() && self.name.is_empty() && self.password.is_empty())
    }
}

impl<'a> NewUser<'a> {
    pub fn new<'b>(name: Option<&'b str>, email: Option<&'b str>, password: Option<&'b str>)
        -> Result<NewUser<'b>, UserError>
    {
        let mut ue = UserError { email: vec![], password: vec![], name: vec![] };
        if let Some(name) = name {
            if name.is_empty() {
                ue.name.push("Username cannot be empty.");
            }

            // FIXME: This should check the graphemes instead of length...
            if name.chars().count() > 20 {
                ue.name.push("Username should be less than 20 characters")
            }
        } else {
            ue.name.push("Username cannot be empty.");
        }

        if let Some(email) = email {
            if email.is_empty() {
                ue.email.push("Email cannot be empty");
            }
            if email.find('@').is_none() {
                ue.email.push("A valid Email contains an @");
            }
        } else {
            ue.email.push("Email cannot be empty.");
        }

        if let Some(password) = password {
            if password.is_empty() {
                ue.password.push("Password cannot be empty");
            }
        } else {
            ue.password.push("Password cannot be empty.");
        }

        if ue.has_any_errors() {
            return Err(ue);
        }

        let password_hash = hash(password.unwrap(), DEFAULT_COST).expect("Could not hash password!");

        Ok(NewUser {
            email: email.unwrap(),
            name: name.unwrap(),
            password_hash: password_hash,
        })
    }
}

pub fn find_all() -> Result<Vec<User>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    users.get_results::<models::user::User>(&*database::connection().get().unwrap()).map_err(|e| e.into())
}

pub fn find(uid: i32) -> Result<Option<User>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::users::dsl::*;

    users.limit(1).filter(id.eq(uid))
         .get_result::<models::user::User>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}


