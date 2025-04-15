use super::champ_data::ChampionData;
use eyre::{eyre, Result};
use std::{fs, path::PathBuf};

/// Load a a Champion's data from the assets folder.
pub fn load_champion(name: &str,) -> Result<ChampionData,> {
  let path =
    PathBuf::from(r#"C:\Users\jamar\Documents\Hobbies\Coding\deux\assets\champions\warrior.json"#,);

  dbg!(path.canonicalize().unwrap());

  let champion_string = match fs::read_to_string(&path,) {
    Ok(str,) => str,
    Err(err,) => {
      return Err(eyre!(
        "Champion data {} does not exist at {}.\n System Error: {err:?}",
        name.to_string(),
        path.to_str().unwrap()
      ),);
    }
  };
  let champion = serde_json::from_str::<ChampionData,>(&champion_string,)?;
  Ok(champion,)
}
