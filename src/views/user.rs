
use views;
use std::borrow::Cow;

pub fn new(errors: Option<(Option<&String>, Option<&String>)>) -> Result<String, ::std::fmt::Error> {
    let mut buffer = String::new();
    let mut partial = String::new();
    try!(html!(partial,
        h1 { "Create new User" }
        form method="post" action="./" {
            div {
                label for="user_email" "Email:"
                input type="text" id="user_email" name="user_email" ""
                @if let Some(errors) = errors {
                    @if errors.0.is_none() {
                        p class="error" "Email cannot be empty"
                    }
                }
            }
            div {
                label for="password" "Password:"
                input type="password" id="password" name="user_password" ""
                @if let Some(errors) = errors {
                    @if errors.1.is_none() {
                        p class="error" "Password cannot be empty"
                    }
                }
            }

            input type="submit" /
        }
    ));

    try!(views::layout::application(&mut buffer, Cow::Borrowed("Register"), Cow::Borrowed(&partial[..])));

    Ok(buffer)
}

