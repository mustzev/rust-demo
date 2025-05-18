use cornucopia::{generate_live, CodegenSettings};
use postgres::{Client, Config, Error, NoTls};
use std::env;

fn create_client() -> Result<Client, Error> {
    let (host, port, username, password, database) = (
        env::var("postgres_host").expect("Env postgres_host not found"),
        env::var("postgres_port").expect("Env postgres_port not found"),
        env::var("postgres_username").expect("Env postgres_username not found"),
        env::var("postgres_password").expect("Env postgres_password not found"),
        env::var("postgres_database").expect("Env postgres_database not found"),
    );
    Config::new()
        .host(&host)
        .port(port.parse().unwrap())
        .user(&username)
        .password(password)
        .dbname(&database)
        .connect(NoTls)
}

fn main() -> Result<(), Error> {
    dotenvy::from_filename("env/.env").expect("Cannot load env variables");

    let queries_path = "src/cornucopia/queries";
    let destination = "src/cornucopia/queries.rs";
    let settings = CodegenSettings {
        is_async: true,
        derive_ser: true,
    };
    println!("cargo:rerun-if-changed={queries_path}");
    let mut client = create_client().unwrap();
    let _ = generate_live(&mut client, queries_path, Some(destination), settings);
    Ok(())
}
