use thiserror::Error;
use std::io::Cursor;
use std::fs::File;
use rocket::response::Response;
use rocket::http::{Status, ContentType};
use simplelog::{ WriteLogger, LevelFilter, Config};
use log::error;

pub fn initialize_logger() {
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), File::create("server.log").unwrap());
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("SQL Error: {source:?}")]
    Sql {
        #[from] source: sqlx::Error,
    },
    #[error("JSON Error: {source:?}")]
    Json {
        #[from] source: rocket::error::Error
    },
    #[error("API Error: {msg:?}")]
    Api {
        msg: String
    }
}

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        {
            error!("{}", self);
            let body = "{\"status\": \"failed\"}";
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .sized_body(body.len(), Cursor::new(body))
                .ok()
        }
    }
}