use anyhow::Result;
mod database;
mod track;
mod import;
mod utils;

fn main() -> Result<()> {
    database::initialize_database("tyrstunes.db")?;
    let directory = import::select_directory()?;
    import::import_tracks_from_directory("tyrstunes.db", &directory)?;
    Ok(())
}