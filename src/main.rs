use axum::Json;
use axum::{http::StatusCode, routing::post, Router};
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use std::str;
use std::time::Duration;

/* let KAFKA_TOPIC = String.from("");
const KAFKA_HOST: std str = "localhost:9092"; */

// { "joke": "Chuck Norris can binary search unsorted data." }
// https://icanhazdadjoke.com/api
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Joke {
    id: String,
    joke: String,
    status: u64,
}

//Get example (async)
#[tokio::main]
async fn main() {
    //    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // Get a joke
        .route("/joke", post(joke));

    // run our app with hyper, listening globally on port 3000
    //    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn joke() -> (StatusCode, Json<Joke>) {
    // insert your application logic here

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    let joke = get_joke().await.unwrap();
    (StatusCode::OK, Json(joke))
}

async fn get_joke() -> Result<Joke, Box<dyn std::error::Error>> {
    let res = reqwest::get("https://icanhazdadjoke.com/").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let joke: Joke = serde_json::from_str(&body)?;
    push_to_kafka(&joke).await.unwrap();
    println!("Body:\n{}", body);
    Ok(joke.clone())
}

// async fn joke(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
// ) -> (StatusCode, Json<User>) {
//     // insert your application logic here

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, ())
// }

// async fn joke() -> Result<Joke, reqwest::Error> {
//     let res = reqwest::get("https://icanhazdadjoke.com/").await?;
//     let body = res.text().await?;
//     let joke: Joke = serde_json::from_str(&body)?;
//     Ok(joke)
// }

async fn push_to_kafka(joke: &Joke) -> Result<(), KafkaError> {
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let payload = serde_json::to_string(joke).unwrap();
    println!("Pushing to Kafka topic {} payload {}", "jokes", payload);

    let delivery_status = producer
        .send(
            FutureRecord::to("jokes")
                .key(&format!("Key {}", joke.id))
                .payload(&payload),
            Duration::from_secs(0),
        )
        .await;

    delivery_status
        .map(|(_, _)| println!("Sent key: {} payload: {}", joke.id, payload))
        .map_err(|(e, _)| e)
}

// use std::fmt::Write;
// use std::time::Duration;
// use kafka::producer::{Producer, Record, RequiredAcks};

// let mut producer =
//     Producer::from_hosts(vec!("localhost:9092".to_owned()))
//         .with_ack_timeout(Duration::from_secs(1))
//         .with_required_acks(RequiredAcks::One)
//         .create()
//         .unwrap();

// let mut buf = String::with_capacity(2);
// for i in 0..10 {
//   let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
//   producer.send(&Record::from_value("my-topic", buf.as_bytes())).unwrap();
//   buf.clear();
// }
