use std::io::{Write};
use std::time::Duration;
//use std::ascii::AsciiExt;

use kafka::client::RequiredAcks;

use kafka::producer::{Producer, Record};
use tracing_subscriber;
mod messaging;
use messaging::process_by_client;
/// This is a very simple command line application reading from a
/// specific kafka topic and dumping the messages to standard output.
fn main() {
    tracing_subscriber::fmt::init();

    // produce_messages();
    match process_by_client(){
        Ok(_) => println!("ok"),
        Err(e) => println!("error: {:?}", e)
    };
}

fn produce_messages() {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut buf = Vec::with_capacity(1024);
    for i in 0..10 {
        let _ = write!(buf, "{}", i); // some computation of the message data to be sent
        producer
            .send(&Record::from_key_value("first-course", "key", "value"))
            .unwrap();
        buf.clear();
    }
}

