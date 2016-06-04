
use std::borrow::Cow;

use views;
use models::user::{UserError, User};

pub fn new(errors: Option<UserError>) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Create new User" }
        form method="post" action="./" {
            div {
                label for="user_name" "Name:"
                input type="text" id="user_name" name="user_name" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.name {
                        p class="error" ^err
                    }
                }
            }
            div {
                label for="user_email" "Email:"
                input type="text" id="user_email" name="user_email" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.email {
                        p class="error" ^err
                    }
                }
            }
            div {
                label for="password" "Password:"
                input type="password" id="password" name="user_password" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.password {
                        p class="error" ^err
                    }
                }
            }

            input type="submit" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Borrowed(&partial[..])));

    Ok(buffer)
}

pub fn index(users: &[User]) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Users" }

        p a href=^(url!("/users/new")) "New User"

        @for user in users {
            div class="user" {
                a.user-link href=^(url!(format!("/users/{}", user.id))) ^user.name
            }
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Users"), Cow::Borrowed(&partial[..])));

    Ok(buffer)
}

pub fn show(user: &User) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { ^user.name }

        div.email {
            "Email: "
            ^user.email
        }

        a href=^(url!(format!("/users/{}/edit", user.id))) "Edit"

        form method="post" action=^(format!("/users/{}", user.id)) {
            input type="hidden" value="DELETE" name="_method" /
            input type="submit" value="Delete" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Owned(format!("User: {}", user.name)), Cow::Borrowed(&partial[..])));

    Ok(buffer)
}

pub fn edit(user: &User, errors: Option<UserError>) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Edit User " ^(user.name) }
        form method="post" action=^(format!("/users/{}", user.id)) {
            div {
                label for="user_name" "Name:"
                input type="text" id="user_name" name="user_name" value=^user.name ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.name {
                        p class="error" ^err
                    }
                }
            }
            div {
                label for="user_email" "Email:"
                input type="text" id="user_email" disabled="disabled" value=^user.email ""
            }
            div {
                label for="password" "Password:"
                input type="password" id="password" name="user_password" ""
                @if let &Some(ref errors) = &errors {
                    @for err in &errors.password {
                        p class="error" ^err
                    }
                }
            }

            input type="submit" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Borrowed(&partial[..])));

    Ok(buffer)
}
