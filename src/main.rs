// mod cornucopia;
mod linfa;

#[tokio::main]
async fn main() {
    dotenvy::from_filename("env/.env").expect("Cannot load env variables");

    // cornucopia::run::run().await;
    linfa::run::run();
}
