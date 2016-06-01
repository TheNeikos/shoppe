use models::schema::users;

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
    password_hash: &'a str,
    name: &'a str,
}
