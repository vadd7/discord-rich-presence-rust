use std::process;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

const TARGET_NAME: &str = "discord_presence_activity";

pub fn launch_presence() -> Result<(), Box<dyn std::error::Error>> {
	//check if process exists
	let mut system = System::new_with_specifics(
		RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
   	);

	system.refresh_processes();
	
	for (_, proc) in system.processes() {
		if proc.name() == TARGET_NAME || proc.name() == format!("{t}.exe", t= TARGET_NAME) {
			proc.kill();
			break;
		}
	}

	let target = if cfg!(debug_assertions) {
		format!("./target/debug/{TARGET_NAME}.exe")
	} else {
		format!("{TARGET_NAME}.exe")
	};

	process::Command::new(target)
		.spawn()?;

    Ok(())
}

/// kill the discord rich presence process.
pub fn exit_presence() -> Result<(), Box<dyn std::error::Error>> {
	let mut system = System::new_with_specifics(
		RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
   	);

	system.refresh_processes();

	let mut broken = false;
	for (_, proc) in system.processes() {
		if proc.name() == TARGET_NAME || proc.name() == format!("{t}.exe", t= TARGET_NAME) {
			proc.kill();
			broken = true;
			break;
		}
	}

	if !broken {return Err("No Process Found.".into());}
	Ok(())
}