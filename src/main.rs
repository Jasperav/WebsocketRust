use ws::{listen, connect};
use std::thread::{spawn, sleep};
use std::time::Duration;

fn main() {
    run_ws();

    connect("ws://127.0.0.1:3042", |c| {
        c.send(""); // Don't know why something needs to be send to open up the connection
        sleep(Duration::from_secs(1));
        move |m| {
            c.shutdown().unwrap();
            Ok(())
        }
    }).unwrap();

    run_ws();
}

fn run_ws() {
    let _ = spawn(|| {
        listen("127.0.0.1:3042", |c| {
            move |msg| {
                c.send(""); // Send something back
                // Not sure why I can not shutdown the websocket on the instance of the websocket itself.
                Ok(())
            }
        }).unwrap();
    });

    sleep(Duration::from_secs(1)); // Let server startup
}
