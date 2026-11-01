use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConfigService {
    data: HashMap<String, Value>,
}

impl Default for ConfigService {
    fn default() -> Self {
        let mut data = HashMap::new();
        data.insert("adminOverlay".to_string(), json!(true));
        data.insert("adminSound".to_string(), json!(true));
        data.insert("speedtag".to_string(), json!(false));
        data.insert("citySpeedUnlock".to_string(), json!(false));
        data.insert("noclip".to_string(), json!(false));
        data.insert("spoofer".to_string(), json!({
            "enabled": false,
            "spooferOverlay": true
        }));
        data.insert("dlc".to_string(), json!(false));
        Self { data }
    }
}

impl ConfigService {
    pub fn get_all(&self) -> Value {
        serde_json::to_value(&self.data).unwrap_or(json!({}))
    }

    pub fn set(&mut self, key: &str, val: Value) -> Result<(), String> {
        self.data.insert(key.to_string(), val);
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        // Saves to %APPDATA%\Owlin\config.json
        Ok(())
    }
}
