use iron::prelude::*;
use iron::status;

use std::error::Error;
use std::fmt::{self, Debug};
use std::convert::{Into, From};

#[derive(Debug)]
pub struct NotImplemented {
    route: String,
}

impl NotImplemented {
    pub fn new(req: &Request) -> NotImplemented {
        NotImplemented {
            route: req.url.clone().into_generic_url().serialize()
        }
    }
}

impl fmt::Display for NotImplemented {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for NotImplemented {
    fn description(&self) -> &str { &self.route }
}


#[derive(Debug)]
pub struct TemplateError {
    cause: Option<Box<Error + Send>>,
}

impl TemplateError {
    pub fn new() -> TemplateError {
        TemplateError {
            cause: None,
        }
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for TemplateError {
    fn description(&self) -> &str { "Template could not be parsed." }
}

impl From<fmt::Error> for TemplateError {
    fn from(other: fmt::Error) -> Self {
        TemplateError {
            cause: Some(Box::new(other)),
        }
    }
}

impl From<TemplateError> for IronError {
    fn from(e: TemplateError) -> IronError {
        IronError::new(Box::new(e), status::InternalServerError)
    }
}
