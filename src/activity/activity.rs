#![windows_subsystem = "windows"]

use discord_rich_presence::activity::Party;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;

use settings_lib::{self, ButtonTrait, SettingsJson, SettingsJsonTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {// 
    let config: SettingsJson = SettingsJson::get_json()?;

    let timestamp = generate_timestamp(&config)?;
    
    let mut client = DiscordIpcClient::new(&config.app_settings.appid)?;

    let party: Party = if let Some(x) = &config.activity_details.party {
        Party::new().size([x.size_min, x.size_max])
    } else {
        Party::new()
    };

	let buttons = config.activity_details.to_discord_button_vec();

	// worst code ever:
	let mut activity = if config.activity_details.assets.large_key.trim() != "" {
		let mut thingimajig = activity::Assets::new()
			.large_image(&config.activity_details.assets.large_key);

		if config.activity_details.assets.large_text_enabled {
			thingimajig = thingimajig.clone().large_text(&config.activity_details.assets.large_text)
		}
		
		thingimajig
	} else {
		activity::Assets::new()
	};

	activity = if config.activity_details.assets.small_key.trim() != "" {
		println!("asd");
		let mut thingimajig = activity.clone()
			.small_image(&config.activity_details.assets.small_key);
			
		if config.activity_details.assets.small_text_enabled {
			thingimajig = thingimajig.clone().small_text(&config.activity_details.assets.small_text)
		}
	
		thingimajig
	} else {
		activity.clone()
	};

    client.connect().unwrap();
    let activity = activity::Activity::new()
        .state(&config.activity_details.state)
        .details(&config.activity_details.details)
        .timestamps(timestamp)
        .party(party)
        .buttons(buttons)
        .assets(activity);


	client.set_activity(activity.clone())?;

	// to be replaced.
	sleep(86400.0);

    client.close()?;

    return Ok(());
}

fn generate_timestamp(config: &SettingsJson) -> Result<activity::Timestamps, Box<dyn std::error::Error>> { // 
    let timezone_offset: i64 = -((Local::now().naive_local() - Local::now().naive_utc()).num_seconds());

    println!("{}", timezone_offset);

    let current_time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("msg")
        .as_secs()
        .try_into()?;
    
    let start_timestamp: activity::Timestamps = match config.activity_details.start_time {
        settings_lib::StartTimestampTypes::None => activity::Timestamps::new(),
        settings_lib::StartTimestampTypes::LaunchTime => activity::Timestamps::new().start(current_time),
        settings_lib::StartTimestampTypes::LocalTime => activity::Timestamps::new().start(timezone_offset + current_time - (current_time % 86400)) ,
        settings_lib::StartTimestampTypes::Number(n ) => {
            let ni64: i64 = n.try_into()?;
            activity::Timestamps::new().start(ni64 + timezone_offset)
        },
    };

    let timestamp: activity::Timestamps = match config.activity_details.end_time {
        settings_lib::EndTimestampTypes::None => start_timestamp ,
        settings_lib::EndTimestampTypes::DayEnd => start_timestamp.end(current_time + timezone_offset - (current_time % 86400) + 86400),
        settings_lib::EndTimestampTypes::Number(n) => {
            let ni64: i64 = n.try_into()?;
            start_timestamp.end(ni64 + timezone_offset)
        },
    };

    Ok(timestamp)    
}

fn sleep(time: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(time))
}