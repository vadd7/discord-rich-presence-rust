pub mod req;
pub mod launch;

use std::io::BufWriter;
use std::fs::File;
use std::io::{BufReader, Write};

use serde::{Deserialize, Serialize};
use serde_json::{self};
use discord_rich_presence::activity;

pub static SETTINGS_PATH: &str = "data/settings.json";

pub fn test() -> String {
    "hi?".to_string()
}

/// definitions for SettingsJson struct.
pub trait SettingsJsonTrait {
    /// write the current settings into settings.json.
    fn write_json(&self) -> std::io::Result<()>;
    /// create an empty instance.
    fn new_empty() -> std::io::Result<SettingsJson>;
    /// create a default instance 
    fn new_default() -> std::io::Result<SettingsJson>;
    /// write a default instance to settings.json and return the default instance
    fn write_default() -> std::io::Result<SettingsJson>;
    /// get current stored data in settings.json
    fn get_json() -> std::io::Result<SettingsJson>;
}

#[derive(Serialize, Deserialize)]
pub struct SettingsJson {
    pub app_settings: AppSettings,
    pub activity_details: ActivityDetails,
}

impl SettingsJsonTrait for SettingsJson {
    fn get_json() -> std::io::Result<SettingsJson> {
        let file = File::open(SETTINGS_PATH)?;
        let reader = BufReader::new(file);

        let u = serde_json::from_reader(reader)?;

        return Ok(u);
    }

    fn write_json(&self) -> std::io::Result<()> {
        let file = File::create(SETTINGS_PATH)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &self)?;
        writer.flush()?;

        Ok(())
    }

    fn new_empty() -> std::io::Result<SettingsJson> {
        let ret = SettingsJson {
            app_settings: AppSettings {
                appid: "".to_string(), 
                auto_update: AutoUpdateStates::No
            },
            activity_details: ActivityDetails {
                state: "".to_string(),
                details: "".to_string(),
                start_time: StartTimestampTypes::None,
                end_time: EndTimestampTypes::None,
                assets: Assets {
                    large_id: None,
                    large_key: "".to_string(),
                    large_text: "".to_string(),
                    small_id: None,
                    small_key: "".to_string(),
                    small_text: "".to_string(),
                    large_text_enabled: false,
                    small_text_enabled: false,
                },
                party: None,
                buttons: None
            }    
        };
        Ok(ret)
    }

    fn new_default() -> std::io::Result<SettingsJson> {
        let ret = SettingsJson {
            app_settings: AppSettings {
                appid: "1221020414004822057".to_string(), 
                auto_update: AutoUpdateStates::No
            },
            activity_details: ActivityDetails {
                state: "Doing Epic Things".to_string(),
                details: "some other epic thing.".to_string(),
                start_time: StartTimestampTypes::LocalTime,
                end_time: EndTimestampTypes::None,
                assets: Assets {
                    large_id: None,
                    large_key: "".to_string(),
                    large_text: "".to_string(),
                    small_id: None,
                    small_key: "".to_string(),
                    small_text: "".to_string(),
					large_text_enabled: false,
                    small_text_enabled: false,
                },
                party: Some(Party {
                    size_min: 2,
                    size_max: 5
                }),
                buttons: Some([
                    Button {
                        label: "epic button".to_string(),
                        url: "about:blank".to_string()
                    }
                ].to_vec())
            }    
        };
		
        Ok(ret)
    }

    fn write_default() -> std::io::Result<SettingsJson> {
        let ret = SettingsJson::new_default()?;
        ret.write_json()?;

        Ok(ret)
    }
}


#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub appid: String,
    pub auto_update: AutoUpdateStates
}

#[derive(Serialize, Deserialize)]
pub enum AutoUpdateStates {
    No,
    Auto,
    Ask
}

#[derive(Serialize, Deserialize)]
pub struct ActivityDetails {
    pub state: String,
    pub details: String,
    pub start_time: StartTimestampTypes,
    pub end_time: EndTimestampTypes,
    pub assets: Assets,
    pub party: Option<Party>,
    pub buttons: Option<Vec<Button>>
}

pub trait ButtonTrait {
    fn to_discord_button_vec(&self) -> Vec<activity::Button>;
}

impl ButtonTrait for ActivityDetails {
    fn to_discord_button_vec(&self) -> Vec<activity::Button> {
        if let Some(x) = &self.buttons {
            let mut final_vec: Vec<activity::Button> = Vec::new();

            for i in x {
				println!("{}, {}", &i.label, &i.url);
                final_vec.push(activity::Button::new(&i.label, &i.url));
            }

            return final_vec;
        } else {
            return Vec::new();
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Button {
    pub label: String,
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct Assets {
    pub large_id: Option<String>,
    pub large_key: String,
    pub small_key: String,
    pub small_id: Option<String>,
    pub large_text: String,
    pub small_text: String,
	pub large_text_enabled: bool,
    pub small_text_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Party {
    pub size_min: i32,
    pub size_max: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StartTimestampTypes {
    None,
    LaunchTime,
    LocalTime,
    Number(u64)
}

#[derive(Serialize, Deserialize, Clone)]
pub enum EndTimestampTypes {
    None,
    DayEnd,
    Number(u64)
}