use serde::{Deserialize, Serialize};
use serde_json;
use reqwest;

// appid: 1221020414004822057

/// add flile format as a freature to image package in cargo.toml if change
pub static FILE_EXT: &str = "png";

/// returns a vec of settings_lib::req::DiscordAsset
/// this contains the image id, name, and a type number (?) 
pub fn get_assets(appid: &str) -> std::io::Result<Vec<DiscordAsset>> {
    let res = reqwest::blocking::get(format!("https://discord.com/api/v9/oauth2/applications/{appid}/assets")).unwrap().text().unwrap();
    let res_parsed: Vec<DiscordAsset> = serde_json::from_str(&res)?;

    return Ok(res_parsed);
}

/// download an image from discord, and write to disk.
/// 
/// i dont actually use this and i spend like 2 hours on this lmao
pub fn get_img_by_id_and_save(appid: &str, asset_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://cdn.discordapp.com/app-assets/{appid}/{asset_id}.{FILE_EXT}");
    let res = reqwest::blocking::get(url)?;

    println!("{:?}", res.headers());
    // checks to see if the folders are there to save the file.
    // first check for app id sub dir, then the parent dir.
    // and then create them
    // check if appid sub directory exists.
    match std::fs::read_dir(format!("data/_img/{appid}", appid = appid)) {
        Ok(_) => {},
        Err(e) => {
            if format!("{:?}", e) == "Os { code: 3, kind: NotFound, message: \"The system cannot find the path specified.\" }" { 
                // check if the image directory exists:
                match std::fs::read_dir(format!("data/_img")) {
                    Ok(_) => {},
                    Err(f) => {
                        if format!("{:?}", f) == "Os { code: 3, kind: NotFound, message: \"The system cannot find the path specified.\" }" {
                            std::fs::create_dir("data/_img")?;
                        } else {
                            panic!("{}", f)
                        }
                    }
                }

                std::fs::create_dir(format!("data/_img/{appid}"))?;
            } else {
                panic!("{}", e)
            }
        }
    }

    std::fs::write(format!("data/_img/{appid}/{asset_id}.{FILE_EXT}"), res.bytes()?)
	    .expect("balls");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordAsset {
    pub id: String,
    pub r#type: u8,
    pub name: String
}