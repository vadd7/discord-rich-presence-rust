use eframe::egui;

pub trait DropdownTrait {
	fn dropdown_menu(&mut self, name: &str, default_index: usize, options: &Vec<DropdownButton>);
	fn get_dropdown_selected(&mut self, name: &str) -> DropdownData;
}

#[derive(Clone)]
pub struct DropdownButton {
	pub name: String,
	pub value: String
}

#[derive(Clone)]
pub struct DropdownData {
    pub selected: String,
}

impl DropdownTrait for eframe::egui::Ui {
	fn dropdown_menu(&mut self, name: &str, default_index: usize, options: &Vec<DropdownButton>) {
		let id = egui::Id::new(name);

        let mut data = self.data_mut(|d| d.get_temp::<DropdownData>(id).unwrap_or(DropdownData {selected: options[default_index].value.to_owned()}));
		self.horizontal(|h| {
			h.label(name);
			h.menu_button(format!("🔻 {d}", d = &data.selected ) , |button_content| {
				if options.len() == 0 {
					panic!("no buttons provided");
				}

				for i in options {
					if button_content.button(i.name.clone()).clicked() {
						data.selected = i.value.to_owned();
						button_content.close_menu();
					}
				}
			});
		});

		self.data_mut(|w| w.insert_temp::<DropdownData>(id, data));
	}

	fn get_dropdown_selected(&mut self, name: &str) -> DropdownData {
		let id = egui::Id::new(name);

        self.data_mut(|d| d.get_temp::<DropdownData>(id).unwrap_or(DropdownData {selected: "error".to_owned()}))
	}
}