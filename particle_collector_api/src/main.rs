use axum::{
    http::request,
    routing::{get, post},
    Json, Router,
};
use models::ParticleCount;
use particles::{establish_connection, insert_particles, query_particles};
use uuid::Uuid;

const DEFAULT_HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/particle", get(get_particle_counts))
        .route("/particle", post(insert_particle_count));

    let host = get_api_host();

    println!("Attempting to connect to: {}", &host);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_particle_counts() -> Result<Json<Vec<ParticleCount>>, String> {
    let request_id = Uuid::new_v4();
    println!(
        "Recieved new get particles request, assigning id: {}",
        request_id
    );

    let mut connection = establish_connection()
        .map(|connection| {
            println!("{} Successfully established a connection", &request_id);
            connection
        })
        .map_err(|e| {
            println!("{} Failed to establish connection", &request_id);
            e.to_string()
        })?;

    query_particles(&mut connection)
        .map(|particles| {
            println!("{}", &request_id);
            Json(particles)
        })
        .map_err(|e| {
            println!("{}", &request_id);
            e.to_string()
        })
}

async fn insert_particle_count(payload: String) -> Result<Json<ParticleCount>, String> {
    let request_id = Uuid::new_v4();
    println!(
        "{} Recieved new insert request, assigning id: {}",
        &request_id, &request_id
    );

    let new_particle_count = serde_json::from_str(&payload)
        .map(|result| {
            println!("{} Successfully deserialized payload", &request_id);
            println!("{} Payload: {:#?}", &request_id, &result);
            result
        })
        .map_err(|e| {
            println!("{} Failed to desearialize payload", &request_id);
            e.to_string()
        })?;

    let mut connection = establish_connection()
        .map(|connection| {
            println!("{} Successfully established connection", &request_id);
            connection
        })
        .map_err(|e| {
            println!("{} Failed to establish a connection", &request_id);
            println!("{}\n{:#?}", &request_id, e);
            e.to_string()
        })?;

    insert_particles(&new_particle_count, &mut connection)
        .map(|result| {
            println!("{} Successfully inserted particle data", &request_id);
            println!("{} Write Result: {:#?}", &request_id, &result);
            Json(result)
        })
        .map_err(|e| {
            println!("{} Failed to insert particle data", &request_id);
            e.to_string()
        })
}

fn get_api_host() -> String {
    std::env::var("API_HOST").unwrap_or(DEFAULT_HOST.to_string())
}

#[cfg(test)]
mod get_api_host_should {
    use std::sync::Mutex;

    use crate::get_api_host;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn read_return_custom_value() {
        let _lock = ENV_LOCK.lock().unwrap();

        let value = "127.211.42:3000";

        std::env::set_var("API_HOST", &value);
        let api_host = get_api_host();

        assert_eq!(api_host, value.to_string());
    }

    #[test]
    fn return_default_value() {
        let _lock = ENV_LOCK.lock().unwrap();

        std::env::remove_var("API_HOST");
        let api_host = get_api_host();

        assert_eq!(api_host, "0.0.0.0:3000");
    }
}
