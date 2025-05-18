mod cornucopia;

use cornucopia::cornucopia::run;

fn main() {
    dotenvy::from_filename("env/.env").expect("Cannot load env variables");

    run();
}
