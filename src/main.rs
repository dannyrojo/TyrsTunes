use anyhow::Result;
mod database;
mod track;
mod import;
mod utils;

fn main() -> Result<()> {
    import::import_tracks_button("tyrstunes.db")?;
    Ok(())
}