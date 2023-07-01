use std::env;

use diesel::{result::Error, *};
use dotenvy::dotenv;
use models::{schema::particles::dsl::particles, NewParticleCount, ParticleCount};

pub fn insert_particles(
    new_particles: &NewParticleCount,
    db_connection: &mut PgConnection,
) -> Result<ParticleCount, Error> {
    diesel::insert_into(models::schema::particles::table)
        .values(new_particles)
        .returning(ParticleCount::as_returning())
        .get_result(db_connection)
}

pub fn query_particles(db_connection: &mut PgConnection) -> Result<Vec<ParticleCount>, Error> {
    particles
        .select(ParticleCount::as_select())
        .load(db_connection)
}

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
}
