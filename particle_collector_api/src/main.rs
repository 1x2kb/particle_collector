use axum::{
    routing::{get, post},
    Json, Router,
};
use models::ParticleCount;
use particles::{establish_connection, insert_particles, query_particles};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/particle", get(get_particle_counts))
        .route("/particle", post(insert_particle_count));

    let connection_string = "0.0.0.0:3000".to_string();
    println!("Attempting to connect to: {}", &connection_string);
    axum::Server::bind(&connection_string.parse().unwrap())
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
    println!("Recieved new insert request, assigning id: {}", request_id);

    let new_particle_count = serde_json::from_str(&payload)
        .map(|result| {
            println!("{} Successfully deserialized payload", &request_id);
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
            Json(result)
        })
        .map_err(|e| {
            println!("{} Failed to insert particle data", &request_id);
            e.to_string()
        })
}
