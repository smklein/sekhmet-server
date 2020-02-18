extern crate chrono;
extern crate google_calendar3 as calendar3;
extern crate hyper;
extern crate hyper_rustls;
extern crate time;
extern crate yup_oauth2 as oauth2;

// Authentication.
use auth;

// Time.
use self::chrono::prelude::*;

// Standard API access.
use std::fmt;

/// An opaque wrapper around a Google Calendar object.
pub struct Calendar {
    hub: calendar3::CalendarHub<
        hyper::Client,
        auth::Authorizer,
    >,
    id: String,
}

// Yeah yeah, I know I'm storing duplicate state here, but it's convenient to
// keep around the original event if I need to update it.
//
// It's also convenient to not unwrap a billion options if I just
// want to look basic fields.
/// A representation of Calendar Events.
///
/// Sekhmet-server only acknowledges events which have all the
/// following fields; the rest are ignored.
#[derive(Debug)]
pub struct Event {
    id: String,
    pub start: chrono::DateTime<Utc>,
    pub end: chrono::DateTime<Utc>,

    /// A free-form text summary of the event.
    pub summary: String,

    /// An optional, free-form description of the Event location.
    pub location: String,

    original: calendar3::Event,
}

/// Enumerated Color options for calendar events.
pub enum Color {
    LightPurple,
    LightGreen,
    Purple,
    Salmon,
    Yellow,
    Orange,
    LightBlue,
    Gray,
    Blue,
    Green,
    Red,
}

fn color_id(c: Color) -> &'static str {
    match c {
        Color::LightPurple => "1",
        Color::LightGreen => "2",
        Color::Purple => "3",
        Color::Salmon => "4",
        Color::Yellow => "5",
        Color::Orange => "6",
        Color::LightBlue => "7",
        Color::Gray => "8",
        Color::Blue => "9",
        Color::Green => "10",
        Color::Red => "11",
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = if self.location == "" {
            "".to_string()
        } else {
            format!(" @ {}", self.location)
        };

        let color = match self.original.color_id.clone() {
            Some(id) => format!("[color: {}] ", id),
            None => "".to_string(),
        };

        write!(f, "{} - {}: {}{}{}",
               self.start,
               self.end,
               color,
               self.summary,
               location)
    }
}

fn parse_time(evt: &Option<calendar3::EventDateTime>) -> Option<chrono::DateTime<Utc>> {
    evt.as_ref()?
        .date_time
        .as_ref()?
        .parse::<chrono::DateTime<Utc>>()
        .ok()
}

fn parse_event(e: calendar3::Event) -> Option<Event> {
    // Required fields
    let id = e.id.as_ref()?.to_string();
    let start = parse_time(&e.start)?;
    let end = parse_time(&e.end)?;
    let summary = e.summary.as_ref()?.to_string();

    // Optional fields
    let location = e.location.as_ref().unwrap_or(&"".to_string()).to_string();
    let original = e;

    Some(Event {
        id,
        start,
        end,
        summary,
        location,
        original,
    })
}

#[derive(Debug)]
pub enum CalendarError {
    Auth(auth::AuthError),
    CalendarAPI(calendar3::Error),
    Other(String),
}

impl From<auth::AuthError> for CalendarError {
    fn from(err: auth::AuthError) -> CalendarError {
        CalendarError::Auth(err)
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
        let auth = auth::new_authenticator()?;
        let hub = calendar3::CalendarHub::new(
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

    // TODO(smklein): Should this just be part of the initialization process?
    // (Can be manually changed later...)
    pub fn set_primary(&mut self) -> Result<(), CalendarError> {
        let (_, list) = self.hub.calendar_list().list().doit()?;
        let items = list.items.ok_or("No calendars listed".to_string())?;
        let primary = items.iter().find(|&entry| entry.primary.is_some()).ok_or(
            "No primary".to_string(),
        )?;
        self.id = primary.id.clone().ok_or("Primary missing ID".to_string())?;
        Ok(())
    }

    /// Update the color of a calendar event.
    pub fn set_color(&self, event: &mut Event, color: Color) -> Result<(), CalendarError> {
        event.original.color_id = Some(color_id(color).to_string());

        self.hub
            .events()
            .update(event.original.clone(), &self.id, &event.id)
            .doit()?;

        Ok(())
    }

    /// Acquire all events within the following ranges of time, ordered
    /// by start time.
    pub fn get_events(
        &self,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
    ) -> Result<Vec<Event>, CalendarError> {
        let min = start.to_rfc3339();
        let max = end.to_rfc3339();

        let (_, events) =
            self.hub
                .events()
                .list(&self.id)
                .add_scope(calendar3::Scope::Readonly)
                .time_min(&min)
                .time_max(&max)
                .single_events(true)
                .order_by("startTime")
                .doit()?;

        let items = events.items.ok_or("No items".to_string())?;

        let mut result = Vec::new();

        for item in items.iter() {
            match parse_event(item.clone()) {
                Some(e) => result.push(e),
                None => (),
            }
        }

        Ok(result)
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
