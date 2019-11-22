mod xor;

use anyhow::Result;
use serde_json::Value;
use std::fs;
use xor::Xor;

fn main() -> Result<()> {
    let json = decrypt_save("WATCHER.autosave")?;
    println!("cards: {}", json["cards"]);

    encrypt_save("test", &json)?;

    let test_json = decrypt_save("test")?;
    println!("relics: {}", test_json["relics"]);

    Ok(())
}

fn decrypt_save(file_name: &str) -> Result<Value> {
    let file = fs::read_to_string(file_name)?;
    let raw_vec = base64::decode(&file).map(|raw| raw.xor(b"key"))?;
    let map: Value = serde_json::from_slice(&raw_vec)?;
    Ok(map)
}

fn encrypt_save(file: &str, json: &Value) -> Result<()> {
    let encrypted = serde_json::to_vec(json)
        .and_then(|raw| Ok(raw.xor(b"key")))
        .and_then(|ref raw| Ok(base64::encode(raw)))?;

    fs::write(file, encrypted)?;
    Ok(())
}
