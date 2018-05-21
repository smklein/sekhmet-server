extern crate hyper;
extern crate hyper_rustls;
extern crate time;
extern crate yup_oauth2 as oauth2;

// Standard API access.
use std::io;
use std::path::Path;

// Authentication with Google APIs.
use self::oauth2::{Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage};

#[derive(Debug)]
pub enum AuthError {
    Io(io::Error),
    Other(String),
}

pub type Authorizer = Authenticator<
            DefaultAuthenticatorDelegate,
            DiskTokenStorage,
            hyper::Client>;

impl From<io::Error> for AuthError {
    fn from(err: io::Error) -> AuthError {
        AuthError::Io(err)
    }
}

impl From<String> for AuthError {
    fn from(err: String) -> AuthError {
        AuthError::Other(err)
    }
}

pub fn new_authenticator() -> Result<Authorizer, AuthError> {
    // Get an ApplicationSecret instance by some means. It contains the
    // `client_id` and `client_secret`, among other things.
    let secret_path = Path::new("secrets/secret.json");
    let secret = try!(oauth2::read_application_secret(secret_path));
    let token_storage = try!(DiskTokenStorage::new(&"secrets/token".to_string()));

    Ok(Authenticator::new(
        &secret,
        DefaultAuthenticatorDelegate,
        hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_rustls::TlsClient::new(),
        )),
        token_storage,
        None,
    ))
}

