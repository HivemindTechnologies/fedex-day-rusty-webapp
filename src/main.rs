use axum::Json;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use log::{info, LevelFilter};
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
    env_logger::builder().filter_level(LevelFilter::Info).init();
    //    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/health", get(health))
        // Get a joke
        .route("/joke", post(joke));

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health() -> (StatusCode, ()) {
    (StatusCode::OK, ())
}

async fn joke() -> (StatusCode, Json<Joke>) {
    // insert your application logic here

    // this will be converted into a JSON response
    // with a status code of `200 OK`
    let joke = get_joke().await.unwrap();
    (StatusCode::OK, Json(joke))
}

async fn get_joke() -> Result<Joke, Box<dyn std::error::Error>> {
    let res = reqwest::Client::new()
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .send()
        .await?;
    //    let res = reqwest::get("https://icanhazdadjoke.com/").await?;
    info!("Status: {}", res.status());
    info!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let joke: Joke = serde_json::from_str(&body)?;
    push_to_kafka(&joke).await.unwrap();
    info!("Body:\n{}", body);
    Ok(joke.clone())
}

async fn push_to_kafka(joke: &Joke) -> Result<(), KafkaError> {
    let bootstrap_servers = std::env::var("BOOTSTRAP_SERVERS")
        .or_else(|_| -> Result<String, ()> { Ok("localhost:9092".to_string()) })
        .unwrap();
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let payload = serde_json::to_string(joke).unwrap();
    info!("Pushing to Kafka topic {} payload {}", "jokes", payload);

    let delivery_status = producer
        .send(
            FutureRecord::to("jokes")
                .key(&format!("Key {}", joke.id))
                .payload(&payload),
            Duration::from_secs(0),
        )
        .await;

    delivery_status
        .map(|(_, _)| info!("Sent key: {} payload: {}", joke.id, payload))
        .map_err(|(e, _)| e)
}
