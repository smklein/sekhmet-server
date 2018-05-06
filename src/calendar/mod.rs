extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;

// Ineract with Google Calendar.
use self::calendar3::Channel;
use self::calendar3::{Result, Error};
use self::calendar3::CalendarHub;

// Standard API access.
use std::default::Default;
use std::path::Path;

// Authentication with Google APIs.
use self::oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};

pub struct Calendar {
    id: u32,
}

impl Calendar {
    pub fn new() -> Calendar {
        let id = 0;

        // Get an ApplicationSecret instance by some means. It contains the
        // `client_id` and `client_secret`, among other things.
        let secret_path = Path::new("secrets/secret.json");
        let secret = match oauth2::read_application_secret(secret_path) {
            Ok(secret) => secret,
            Err(err) => {
                panic!("Cannot open secrets/secret.json: {}", err);
            }
        };

        // Instantiate the authenticator. It will choose a suitable
        // authentication flow for you, unless you replace  `None` with the
        // desired Flow.  Provide your own `AuthenticatorDelegate` to adjust the
        // way it operates and get feedback about what's going on. You probably
        // want to bring in your own `TokenStorage` to persist tokens and
        // retrieve them from storage.
        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            <MemoryStorage as Default>::default(),
            None,
        );
        let hub = CalendarHub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );
        // As the method needs a request, you would usually fill it with the
        // desired information into the respective structure. Some of the parts
        // shown here might not be applicable !  Values shown here are possibly
        // random and not representative !
        let req = Channel::default();

        // You can configure optional parameters by calling the respective
        // setters at will, and execute the final call using `doit()`.  Values
        // shown here are possibly random and not representative !
        let result = hub.calendar_list().list().doit();

        match result {
            Err(e) => {
                match e {
                    // The Error enum provides details about what exactly happened.
                    // You can also just use its `Debug`, `Display` or `Error` traits
                    Error::HttpError(_) |
                    Error::MissingAPIKey |
                    Error::MissingToken(_) |
                    Error::Cancelled |
                    Error::UploadSizeLimitExceeded(_, _) |
                    Error::Failure(_) |
                    Error::BadRequest(_) |
                    Error::FieldClash(_) |
                    Error::JsonDecodeError(_, _) => println!("Failed request: {}", e),
                }
            }
            Ok(res) => println!("Success: {:?}", res),
        }

        Calendar { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(2 + 2, 4);
    }
}
