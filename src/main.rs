use anyhow::Result;
mod database;
mod track;
mod import;
mod utils;
mod stage;

fn main() -> Result<()> {
    import::import_tracks_button("tyrstunes.db")?;
    Ok(())
}