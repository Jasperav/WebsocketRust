use ws::{listen, connect, Sender, Handler, Message, Handshake};
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::Arc;

struct Client {
    c: Sender,
    b: Arc<Sender>
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        self.c.send("") // Send a message to open the connection, weird
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        self.b.shutdown().unwrap();
        self.c.shutdown().unwrap();

        Ok(())
    }
}

fn main() {
    let s = run_ws();
    let arc = Arc::new(s);

    connect("ws://127.0.0.1:3042", |c| {
        Client {
            c,
            b: Arc::clone(&arc)
        }
    }).unwrap();

    run_ws();
}

fn run_ws() -> Sender {
    let socket = ws::Builder::new().build(|c: ws::Sender| {
        move |msg| {
            c.send(""); // Send something back
            // Not sure why I can not shutdown the websocket on the instance of the websocket itself.
            Ok(())
        }
    }).unwrap();

    let broadcast = socket.broadcaster();

    let _ = spawn(move || {
        socket.listen("127.0.0.1:3042").unwrap();
    });

    sleep(Duration::from_secs(1)); // Let server startup

    broadcast
}
