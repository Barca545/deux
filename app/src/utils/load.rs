use super::champ_data::ChampionData;
use eyre::{eyre, Result};
use std::fs;

/// Load a a Champion's data from the assets folder.
pub fn load_champion(name:&str,) -> Result<ChampionData,> {
  let path = format!("assets/champions/{name}.json");

  let champion_string = match fs::read_to_string(&path,) {
    Ok(str,) => str,
    Err(err,) => {
      return Err(eyre!(
        "Champion data {} does not exist at {path}.\n System Error: {err:?}",
        name.to_string()
      ),);
    }
  };
  let champion = serde_json::from_str::<ChampionData,>(&champion_string,)?;
  Ok(champion,)
}
