use std::{io::{self, Write}, time::Duration};

use kafka::{
    client::{FetchOffset, GroupOffsetStorage, KafkaClient, RequiredAcks}, consumer::Consumer, producer::{Producer, Record}, Error
};

pub fn process_by_client() -> Result<(), Error> {
    println!("client");
    let mut c = {
        let mut client = KafkaClient::new(vec!["localhost:9092".to_owned()]);
        client.load_metadata_all().unwrap();

        let builder = Consumer::from_client(client);
        let mut cb = builder
            .with_group("group".to_owned())
            .with_fallback_offset(FetchOffset::Latest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka));
        cb = cb.with_topic("first-course".to_owned());
        cb.create()?
    };
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


pub fn produce_messages() {
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