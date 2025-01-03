# Simple messaging microservice with Kafka and Rust

To launch the stack, run:

```bash
docker-compose up -d 
```

Then create a topic:

```bash
docker exec -it first-course-kafka-1 /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --topic first-course
```

Then run the producer:

```bash
docker exec --workdir /opt/kafka/bin/ -it first-course-kafka-1 sh ./kafka-console-producer.sh --bootstrap-server localhost:9092 --topic first-course
```

Then cargo run in another terminal to run the service:

```bash
cargo run
```


## Description

```
└── src
    ├── config        # Configure the Http server, access to services...
    ├── main.rs
    ├── recorder      # Record that arrive from the Kafka topic in the database
    ├── messaging     # Logic handle http call from clients
    ├── repositories  # High level implementation of the business logic
    ├── routes        # Define the routes of the application
    ├── server        # Start the HTTP server
    └── services      # Instantiation and acknoledgement of the services (Mongo, Kafka)
```