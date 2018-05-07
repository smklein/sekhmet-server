extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;

// Ineract with Google Calendar.
use self::calendar3::CalendarHub;

// Standard API access.
use std::io;
use std::path::Path;

// Authentication with Google APIs.
use self::oauth2::{Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage};

pub struct Calendar {
    hub: CalendarHub<
        hyper::Client,
        Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>,
    >,
    id: String,
}

#[derive(Debug)]
pub enum CalendarError {
    Io(io::Error),
    CalendarAPI(calendar3::Error),
    Other(String),
}

impl From<io::Error> for CalendarError {
    fn from(err: io::Error) -> CalendarError {
        CalendarError::Io(err)
    }
}

impl From<calendar3::Error> for CalendarError {
    fn from(err: calendar3::Error) -> CalendarError {
        CalendarError::CalendarAPI(err)
    }
}

impl From<String> for CalendarError {
    fn from(err: String) -> CalendarError {
        CalendarError::Other(err)
    }
}

impl Calendar {
    pub fn new() -> Result<Calendar, CalendarError> {
        // Get an ApplicationSecret instance by some means. It contains the
        // `client_id` and `client_secret`, among other things.
        let secret_path = Path::new("secrets/secret.json");
        let secret = try!(oauth2::read_application_secret(secret_path));
        let token_storage = try!(DiskTokenStorage::new(&"secrets/token".to_string()));

        let auth = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            token_storage,
            None,
        );
        let hub = CalendarHub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );
        let cal = Calendar {
            hub,
            id: "None".to_string(),
        };

        Ok(cal)
    }

    pub fn set_primary(&mut self) -> Result<(), CalendarError> {
        let (_, list) = try!(self.hub.calendar_list().list().doit());
        let items = try!(list.items.ok_or("No calendars listed".to_string()));
        let primary = try!(items.iter().find(|&entry| entry.primary.is_some()).ok_or(
            "No primary".to_string(),
        ));

        self.id = try!(primary.id.clone().ok_or("Primary missing ID".to_string()));
        Ok(())
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
