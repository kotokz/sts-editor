mod game_type;
mod xor;

use anyhow::Result;
use game_type::GameObj;
use std::fs;
use std::str;
use xor::Xor;

fn main() -> Result<()> {
    let mut game_map = decrypt_save("WATCHER.autosave")?;

    Ok(())
}

fn basic_edit(game_map: &mut GameObj) -> &GameObj {
    game_map.red += 1;
    game_map.relics.push("Membership Card".to_string());
    game_map.gold += 1000;
    for card in game_map.cards.iter_mut() {
        card.upgrades = 1;
    }
    game_map
}

fn decrypt_save(file_name: &str) -> Result<GameObj> {
    let file = fs::read_to_string(file_name)?;
    let raw_vec = base64::decode(&file).map(|raw| raw.xor(b"key"))?;
    // println!("{:?}", str::from_utf8(&raw_vec));
    let map: GameObj = serde_json::from_slice(&raw_vec)?;
    Ok(map)
}

fn encrypt_save(file: &str, json: &GameObj) -> Result<()> {
    let encrypted = serde_json::to_vec(json)
        .and_then(|raw| Ok(raw.xor(b"key")))
        .and_then(|ref raw| Ok(base64::encode(raw)))?;

    fs::write(file, encrypted)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_decrypt() {
        let game_map = decrypt_save("WATCHER.autosave").unwrap();

        assert_eq!(game_map.cards[0].id, "Defend_P");
        assert_eq!(game_map.relics[1], "Membership Card");
    }

    #[test]
    fn test_encrypt() {
        let game_map = decrypt_save("WATCHER.autosave").unwrap();
        encrypt_save("test2", &game_map).unwrap();

        let test_json = decrypt_save("test2").unwrap();
        assert_eq!(test_json.cards[0].id, "Defend_P");
        assert_eq!(test_json.relics[1], "Membership Card");
    }
}
