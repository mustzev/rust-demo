mod cornucopia;

use cornucopia::run::run;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("env/.env").expect("Cannot load env variables");

    run().await;
}
