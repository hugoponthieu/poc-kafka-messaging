use std::{
    io::{self, Write},
    time::Duration,
};

use kafka::{
    client::{FetchOffset, GroupOffsetStorage, KafkaClient},
    consumer::Consumer,
    Error,
};

pub fn process_by_client() -> Result<(), Error> {
    let client = KafkaClient::new(vec!["localhost:9092".to_owned()]);
    println!("client");
    let mut c = {
        let mut cb = Consumer::from_client(client)
            .with_group("kafka-rust-console-consumer".to_owned())
            .with_client_id("kafka-rust-console-consumer".to_owned())
            // .with_fetch_max_wait_time(Duration::from_secs(1))
            // .with_fetch_min_bytes(1_000)
            // .with_fetch_max_bytes_per_partition(100_000)
            // .with_retry_max_bytes_limit(1_000_000)
            .with_group("group".to_owned())
            .with_fallback_offset(FetchOffset::Latest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka));
        println!(
            "after config
            ",
        );
        cb = cb.with_topic("first-course".to_owned()); 
        cb.create()?
    };

    println!("connext");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = Vec::with_capacity(1024);

    loop {
        let message_sets: kafka::consumer::MessageSets = match c.poll() {
            Ok(ms) => ms,
            Err(e) => {
                println!("here");
                return Err(e);
            }
        };

        for ms in message_sets.iter() {
            for m in ms.messages() {
                let _ = println!("{}:{}@{}:", ms.topic(), ms.partition(), m.offset);
                buf.extend_from_slice(m.value);
                buf.push(b'\n');
                stdout.write_all(&buf)?;
                buf.clear();
            }
            let _ = c.consume_messageset(ms);
        }
    }
}
