#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use dropdown::{DropdownButton, DropdownTrait};
use eframe::egui::{self, Color32, RichText};

use egui_extras::{Size, StripBuilder};
use settings_lib::{self, EndTimestampTypes, SettingsJson, SettingsJsonTrait, StartTimestampTypes};
mod dropdown;


pub fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 480.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Discord Rich Presence Config",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
			
            Ok(Box::<RpcApp>::default())
        }),
    )
}

struct RpcApp {
    page: ActivePage,
    activity_settings: settings_lib::SettingsJson,
    bottom_size: f32,
	contains_errors: bool,
	bottomerr: Option<RichText>,
	party_on: bool,
	saved_assets: Vec<settings_lib::req::DiscordAsset>
}

pub enum ActivePage {
    RPCSettings, 
    Settings
}

impl Default for RpcApp {
    fn default() -> Self {
        let mut settings = SettingsJson::get_json().unwrap();

		let discord_assets = settings_lib::req::get_assets(&settings.app_settings.appid).unwrap();

        Self {
            page: ActivePage::RPCSettings,
            bottom_size: 18.0,
			contains_errors: false,
            bottomerr: None,
			party_on: if let Some(_) = &settings.activity_details.party {
				true
			} else {
				settings.activity_details.party = Some(settings_lib::Party { size_min: 0, size_max: 0 });
				false
			},
			saved_assets: discord_assets,
			activity_settings: settings
        }
    }
}

impl eframe::App for RpcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {   
			self.contains_errors = false;

            ui.horizontal(|ui|{
                if ui.button("rich presence").clicked() {
                    self.page = ActivePage::RPCSettings;
                }
                if ui.button("settings").clicked() {
                    self.page = ActivePage::Settings;
                }
            });    
            
            ui.separator();

            match self.page {
                ActivePage::RPCSettings => {
                    StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::exact(self.bottom_size.clone()))
                    .vertical(|mut strip| {
                        strip.strip(|strip_builder| {
                            strip_builder.size(Size::relative(0.25).at_least(250.0))
                            .size(Size::relative(0.25).at_least(250.0))
                            .size(Size::remainder().at_most(300.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    // buttons strip 1.
									ui.heading("Text: ");

                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("State: ");
                                        ui.text_edit_singleline(&mut self.activity_settings.activity_details.state);
                                    });

                                    if self.activity_settings.activity_details.state.trim() == "" {
										self.contains_errors = true;
                                        ui.label(RichText::new("Must Provide a State for Rich Presence.")
                                            .color(Color32::RED)
                                        );
                                    }
									
									ui.label("");

                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Details: ");
                                        ui.text_edit_singleline(&mut self.activity_settings.activity_details.details);
                                    });

                                    if &self.activity_settings.activity_details.details.trim() == &"" {
										self.contains_errors = true;
                                        ui.label(RichText::new("Must Provide Details text for Rich Presence.")
                                            .color(Color32::RED)
                                        );
                                    }

									ui.separator();
									ui.heading("Party: ");

									ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Party enabled: ");
										ui.checkbox(&mut self.party_on, "");
                                    });

									ui.label("");

									let party_clone = self.activity_settings.activity_details.party.clone();

									let mut temp_min = party_clone.unwrap().size_min.to_string();
									let mut temp_max = party_clone.unwrap().size_max.to_string();

									let mut party_res = settings_lib::Party { 
										size_max: party_clone.unwrap().size_max,
										size_min: party_clone.unwrap().size_min
									};
									
									ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Party min: ");
                                        ui.text_edit_singleline(&mut temp_min);
                                    });

									if !is_numeric(&temp_min) && self.party_on {
										self.contains_errors = true;
                                        //ui.label(RichText::new("The Party number contains non numeric characterrs.").color(Color32::RED));
                                    } else {
										party_res.size_min = temp_min.parse::<i32>().unwrap_or(0);
									}

									ui.label("");

									ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Party max: ");
                                        ui.text_edit_singleline(&mut temp_max);
                                    });

									if !is_numeric(&temp_max) && self.party_on {
										self.contains_errors = true;
                                    } else {
										party_res.size_max = temp_max.parse::<i32>().unwrap_or(0);

										if party_res.size_max < party_res.size_min {
											self.contains_errors = true;
											ui.label(RichText::new("The maximum size must be bigger than the minimum!").color(Color32::RED));
										}
									}
								
									self.activity_settings.activity_details.party = Some(party_res)
                                });
                                strip.cell(|ui| {
									ui.heading("Image Text:");

                                    // buttons strip 2
									ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Image text: ");
										ui.checkbox(&mut self.activity_settings.activity_details.assets.large_text_enabled, "");
                                        ui.text_edit_singleline(&mut self.activity_settings.activity_details.assets.large_text);
                                    });

									if self.activity_settings.activity_details.assets.large_text.trim().len() < 3 &&
									   self.activity_settings.activity_details.assets.large_text_enabled == true 
									{
										self.contains_errors = true;
                                        ui.label(RichText::new("The Text Length must be longer than 3 characters")
                                            .color(Color32::RED)
                                        );
                                    }

									ui.label("");

									ui.horizontal(|ui: &mut egui::Ui| {
                                        ui.label("Small image text: ");
										ui.checkbox(&mut self.activity_settings.activity_details.assets.small_text_enabled, "");
                                        ui.text_edit_singleline(&mut self.activity_settings.activity_details.assets.small_text);
                                    });

									if self.activity_settings.activity_details.assets.small_text.trim().len() < 3 &&
									   self.activity_settings.activity_details.assets.small_text_enabled == true 
									{
										self.contains_errors = true;
                                        ui.label(RichText::new("The Text Length must be longer than 3 characters")
                                            .color(Color32::RED)
                                        );
                                    }

									ui.separator();

									ui.heading("Images: ");

									let mut images_vec: Vec<DropdownButton<String>> = [DropdownButton {
										name: "None".to_owned(),
										value: "none".to_owned()
									}].to_vec();
									for i in &self.saved_assets {
										images_vec.push(DropdownButton {
											name: i.name.clone(),
											value: i.name.clone()
										});
									}

									// get index for currently selected large image
									let mut large_selected_index: usize = 0;
									if self.activity_settings.activity_details.assets.large_key.trim() != "" {
										for i in &images_vec {
											if i.name == self.activity_settings.activity_details.assets.large_key {
												break;
											}
											large_selected_index += 1;
										}
									}

									let mut small_selected_index: usize = 0;
									if self.activity_settings.activity_details.assets.small_key.trim() != "" {
										for i in &images_vec {
											if i.value == self.activity_settings.activity_details.assets.small_key {
												break;
											}
											small_selected_index += 1;
										}
									}
									// get index for currently selected small image
									
									
									ui.dropdown_menu("large image: ", large_selected_index, &images_vec);

									let large_dropdown_state: Option<DropdownButton<String>> = ui.get_dropdown_selected("large image: ").selected;
									self.activity_settings.activity_details.assets.large_key = if let Some(j) = large_dropdown_state {
										j.value
									} else {
										"".to_owned()
									};

									ui.label("");

									ui.dropdown_menu("small image: ", small_selected_index, &images_vec);

									let small_dropdown_state: Option<DropdownButton<String>> = ui.get_dropdown_selected("small image: ").selected;
									self.activity_settings.activity_details.assets.small_key = if let Some(j) = small_dropdown_state {
										if j.value == "none" {
											"".to_owned()
										} else {
											j.value
										}
									} else {
										"".to_owned()
									};

									ui.separator();
									ui.heading("Timestamps: ");
									
									let start_time_index = match self.activity_settings.activity_details.start_time {
										settings_lib::StartTimestampTypes::None => 0,
										settings_lib::StartTimestampTypes::LaunchTime => 1,
										settings_lib::StartTimestampTypes::LocalTime => 2,
										settings_lib::StartTimestampTypes::Number(_) => 3,
									};

									ui.dropdown_menu::<StartTimestampTypes>("Start Timestamp", start_time_index, &[
										DropdownButton {name: "None".to_string(), value: StartTimestampTypes::None},
										DropdownButton {name: "launch time".to_string(), value: StartTimestampTypes::LaunchTime},
										DropdownButton {name: "local time".to_string(), value: StartTimestampTypes::LocalTime},
										DropdownButton {name: "timestamp number".to_string(), value: StartTimestampTypes::Number(0)},
									].to_vec());

									let mut start_time = match ui.get_dropdown_selected::<StartTimestampTypes>("Start Timestamp").selected {
										Some(selected_button) => selected_button,
										None => DropdownButton { name: "None".to_string(), value: StartTimestampTypes::None },
									}.value;

									let start_time_number: u64 = if let settings_lib::StartTimestampTypes::Number(n) = &self.activity_settings.activity_details.start_time {
										n.clone()
									} else { 0 };

									let mut start_time_num_string = start_time_number.to_string();

									if let StartTimestampTypes::Number(_) = start_time {
										ui.horizontal(|ui| {
											ui.label("Number: ");
											ui.text_edit_singleline(&mut start_time_num_string);
										});

										start_time = StartTimestampTypes::Number(start_time_num_string.parse::<u64>().unwrap_or(0));
									} 

									self.activity_settings.activity_details.start_time = start_time;

									ui.label("");

									let end_timestamp_index = match self.activity_settings.activity_details.end_time {
										settings_lib::EndTimestampTypes::None => 0,
										settings_lib::EndTimestampTypes::DayEnd => 1,
										settings_lib::EndTimestampTypes::Number(_) => 2,
									};
									
									ui.dropdown_menu::<EndTimestampTypes>("End Timestamp", end_timestamp_index, &[
										DropdownButton {name: "None".to_string(), value: EndTimestampTypes::None},
										DropdownButton {name: "day end".to_string(), value: EndTimestampTypes::DayEnd},
										DropdownButton {name: "timestamp number".to_string(), value: EndTimestampTypes::Number(0)},
									].to_vec());

									let mut end_time = match ui.get_dropdown_selected::<EndTimestampTypes>("End Timestamp").selected {
										Some(selected_button) => selected_button,
										None => DropdownButton { name: "None".to_string(), value: EndTimestampTypes::None },
									}.value;

									let end_time_number: u64 = if let settings_lib::EndTimestampTypes::Number(n) = &self.activity_settings.activity_details.end_time {
										n.clone()
									} else { 0 };

									let mut end_time_num_string = end_time_number.to_string();

									if let EndTimestampTypes::Number(_) = end_time {
										ui.horizontal(|ui| {
											ui.label("Number: ");
											ui.text_edit_singleline(&mut end_time_num_string);
										});

										end_time = EndTimestampTypes::Number(end_time_num_string.parse::<u64>().unwrap_or(0))
									} 

									self.activity_settings.activity_details.end_time = end_time;
                                });
                                strip.cell(|ui| {
									ui.heading("Preview: ");
									ui.label("i am lazy to do this rn will be done later maybe possibly or no idfk.");
									ui.label("until then here is an image sometimes.");

                                    // preview strip.
                                    let preview_id = if let Some(s) = &self.activity_settings.activity_details.assets.large_id {
                                        &s
                                    } else {
                                        let asd = get_image_id_by_name(&self.activity_settings.activity_details.assets.large_key, &self.saved_assets);

                                        self.activity_settings.activity_details.assets.large_id = Some(asd.clone());
                                        println!("{}", asd);
                                        &asd.clone()
                                    };

                                    let web_path = format!("https://cdn.discordapp.com/app-assets/{appid}/{asset_id}.{ext}",
                                        appid = &self.activity_settings.app_settings.appid, 
                                        asset_id = &preview_id,
                                        ext = settings_lib::req::FILE_EXT);

                                    ui.image(&web_path);
                                });
                            });
                        });
                        strip.cell(|ui| {
                            // bottom buttons cell
							ui.horizontal(|ui| {
								if ui.button("save").clicked() {
									if self.contains_errors {
										self.bottomerr = Some(RichText::new("Some fields contained invalid values, so it couldn't be saved to disk.").color(Color32::LIGHT_RED));
									} else {
										let temp_party_values = self.activity_settings.activity_details.party.clone().unwrap();
										if !self.party_on { 
											self.activity_settings.activity_details.party = None 
										}

										match self.activity_settings.write_json() {
											Ok(_) => {
												self.bottomerr = Some(RichText::new("The configuration has been saved to disk."));
											},
											Err(e) => {
												self.bottomerr = Some(RichText::new(format!("Something went wrong while saving. {}", e)).color(Color32::RED));
											},
										};

										if !self.party_on { self.activity_settings.activity_details.party = Some(temp_party_values) }
									}
								};
								
								if ui.button("launch").clicked() {
									match settings_lib::launch::launch_presence() {
                                        Ok(_) => {
											self.bottomerr = Some(RichText::new("Successfully launched activity!"));
										},
                                        Err(e) => {
											self.bottomerr = Some(RichText::new(format!("Couldnt launch the activity process: {}", e)).color(Color32::RED));
										}
                                    };
								};
							
								if ui.button("stop").clicked() {
									match settings_lib::launch::exit_presence() {
										Err(ee) => {
											self.bottomerr = Some(RichText::new(format!("Couldnt stop the activity process: {err}", err = ee)).color(Color32::RED));
										},
										_ => {self.bottomerr =  Some(RichText::new("Stopped activity process."));}
									}
								};

								if let Some(s) = &self.bottomerr {
									ui.separator();
									ui.label(s.clone());
								}
							});
                        });
                    });
                },
                ActivePage::Settings => {
					ui.horizontal( |ui| {
						ui.label("application id: ");
						ui.text_edit_singleline(&mut self.activity_settings.app_settings.appid);
					});
				}
            }
        });
    }
}

fn is_numeric(text: &str) -> bool {
	let mut is_numeric = true;
	for i in text.chars() {
		if !i.is_numeric() {
			is_numeric = false;
			break;
		}
	}

	return is_numeric;
}

fn get_image_id_by_name(target: &str, find_in: &Vec<settings_lib::req::DiscordAsset> ) -> String {
	let mut asd: String = "-1".to_owned();
	for i in find_in {
	    println!("i.name = {}, key = {}", &i.name, target);
	    if i.name == target {
	        println!("match: {}", i.id);
	        asd = i.id.clone();
	        break;
	    }
	}

	return asd;
}