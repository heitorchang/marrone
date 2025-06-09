use actix_web::{get, web::ServiceConfig, Responder};
use rand::Rng;
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
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

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(latin_phrase);
    };

    Ok(config.into())
}
