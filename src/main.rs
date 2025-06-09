use actix_web::{web, App, HttpServer, Responder};
use rand::Rng;

async fn latin_phrase() -> impl Responder {
    const LATIN_PHRASES: [&str; 10] = [
        "Absit iniuria verbis",
        "Ab uno disce omnis",
        "Ab urbe condita",
        "Abusus non tollit usum",
        "Acta est fabula",

        "Age quod agis",
        "Alea iacta est",
        "Amicus Plato, sed magis amica veritas",
        "Animus meminisse horret",
        "Ars longa, vita brevis",
    ];
    let mut rng = rand::rng();
    let random_index = rng.random_range(0..LATIN_PHRASES.len());
    LATIN_PHRASES[random_index]
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(latin_phrase))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
