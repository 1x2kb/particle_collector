use axum::{
    http::request,
    routing::{get, post},
    Router,
};
use particles::{establish_connection, insert_particles, query_particles};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/particle", get(get_particle_counts))
        .route("/particle", post(insert_particle_count));

    println!("Attempting to connect to: {}", "0.0.0.0:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_particle_counts() -> String {
    let mut connection = establish_connection();
    let response = match query_particles(&mut connection) {
        Ok(particles) => serde_json::to_string(&particles).unwrap(),
        Err(e) => e.to_string(),
    };

    response
}

async fn insert_particle_count(payload: String) -> String {
    let request_id = Uuid::new_v4();
    println!("Recieved new insert request, assigning id: {}", request_id);

    println!("{} deserializing payload", request_id);
    let new_particle_count = match serde_json::from_str(&payload) {
        Ok(particle_count) => particle_count,
        Err(e) => return e.to_string(),
    };
    println!("{} successfully deserialized payload", request_id);

    println!("{} attempting to establish db connection", request_id);
    let mut connection = establish_connection();
    println!("{} established connection", request_id);

    println!("{} attempting to insert particle data", request_id);
    let response = match insert_particles(&new_particle_count, &mut connection) {
        Ok(particle_count) => serde_json::to_string(&particle_count).unwrap(),
        Err(e) => return e.to_string(),
    };
    println!("{} successfully inserted particle data", request_id);

    response
}
