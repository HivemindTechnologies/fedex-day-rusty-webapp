use axum::{
    routing::get,
    Router,
};

// { "joke": "Chuck Norris can binary search unsorted data." }
// https://icanhazdadjoke.com/api
struct Joke {
    id: String,
    joke: String,
    status: u64
}

//Get example RUST
fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("https://icanhazdadjoke.com/")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

// def get: F[Jokes.Joke] =
// C.expect[Joke](GET(uri"https://icanhazdadjoke.com/"))
//   .adaptError{ case t => JokeError(t)} // Prevent Client Json Decoding Failure Leaking
// 
// def jokeRoutes[F[_]: Sync](J: Jokes[F]): HttpRoutes[F] =
// val dsl = new Http4sDsl[F]{}
// import dsl._
// HttpRoutes.of[F] {
//   case GET -> Root / "joke" =>
//     for {
//       joke <- J.get
//       resp <- Ok(joke)
//     } yield resp
// }

fn runClient()
