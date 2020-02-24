extern crate google_fitness1 as fitness;
extern crate hyper;
extern crate hyper_rustls;
extern crate time;
extern crate yup_oauth2 as oauth2;

// Authentication.
use auth;

/// An opaque wrapper around a Google Fitness object.
pub struct Fitness {
    hub: fitness::Fitness<hyper::Client, auth::Authorizer>,
}

pub fn go() {
    let f = Fitness::new().unwrap();
    f.hello();
}

#[derive(Debug)]
pub enum FitnessError {
    Auth(auth::AuthError),
    FitnessAPI(fitness::Error),
    Other(String),
}

impl From<auth::AuthError> for FitnessError {
    fn from(err: auth::AuthError) -> FitnessError {
        FitnessError::Auth(err)
    }
}

impl From<fitness::Error> for FitnessError {
    fn from(err: fitness::Error) -> FitnessError {
        FitnessError::FitnessAPI(err)
    }
}

impl From<String> for FitnessError {
    fn from(err: String) -> FitnessError {
        FitnessError::Other(err)
    }
}

impl Fitness {
    pub fn new() -> Result<Fitness, FitnessError> {
        let auth = auth::new_authenticator()?;
        let hub = fitness::Fitness::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(
                hyper_rustls::TlsClient::new(),
            )),
            auth,
        );
        let f = Fitness { hub };

        Ok(f)
    }

    pub fn hello(&self) {
        // XXX XXX figure out what to do with fitness api...
        //
        // merge with calendar? That would be cool..
        let (_, list) = self.hub.users().data_sources_list("me").doit().unwrap();
        println!("Acquired data sources...");
        let srcs = list.data_source.unwrap();
        println!("Unwrapped sources..");
        for (mut i, src) in srcs.iter().enumerate() {
            // XXX yikes, is this listing *everything* ???
            // XXX contains ~90 sources?
            println!("Fitness listing data sources: {:#?}", src);

            i += 1;
            if i == 100 {
                break;
            }
        }

        println!("Number of sources: {}", srcs.len());
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
