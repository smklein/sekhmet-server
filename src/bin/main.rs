extern crate sekhmet_server as sekhmet;
extern crate chrono;

use sekhmet::calendar::{Calendar, CalendarError, Color, Event};
use sekhmet::fit::Go;
use sekhmet::gpio::{get_hardware, Hardware};
use sekhmet::thread_pool::ThreadPool;

use self::chrono::prelude::*;
use self::chrono::Duration as CDuration;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn sekhmet_events(c: &Calendar) -> Result<Vec<Event>, CalendarError> {
    let start = Utc::now();
    let end = start + CDuration::days(1);
    let events = try!(c.get_events(start, end));

    println!("--- All events:");
    for e in events.iter() {
        println!("{}", e);
    }

    Ok(
        events
            .into_iter()
            .filter(|e| e.summary.starts_with("#sek "))
            .collect(),
    )
}

fn main() {
    println!("sekhmet server... STARTING");

    Go();
    let mut c = Calendar::new().unwrap();

    println!("sekhmet server... created calendar");

    c.set_primary().unwrap();

    let events = match sekhmet_events(&c) {
        Ok(events) => events,
        Err(err) => {
            panic!("Sekhmet server cannot acquire events: {:?}", err);
        }
    };

    println!("--- Sekhmet events:");
    for mut event in events {
        println!("{}", event);
        if let Err(err) = c.set_color(&mut event, Color::Purple) {
            panic!("Sekhmet server cannot set color: {:?}", err)
        }
    }

    let mut hardware = get_hardware();

    hardware.led_toggle(Duration::from_millis(1000));

    // TODO(smklein): Listen on domain (or static ip?)
    // TODO(smklein): Authenticate to access
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(10) {
        let stream = stream.unwrap();

        pool.execute(|| { handle_connection(stream); });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
