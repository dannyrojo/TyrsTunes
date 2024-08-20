use anyhow::{Result, Context};
mod database;
mod track;

fn main() -> Result<()> {
    database::initialize_database("tyrstunes.db")?;

    Ok(())
}