use eframe::egui;

pub trait DropdownTrait {
	fn dropdown_menu<T: std::clone::Clone + std::marker::Send + std::marker::Sync + 'static>(&mut self, name: &str, default_index: usize, options: &Vec<DropdownButton<T>>);
	fn get_dropdown_selected<T: std::clone::Clone + 'static >(&mut self, name: &str) -> DropdownData<T>;
}

#[derive(Clone)]
pub struct DropdownButton<T: std::clone::Clone> {
	pub name: String,
	pub value: T
}

#[derive(Clone)]
pub struct DropdownData<T: std::clone::Clone> {
    pub selected: Option<DropdownButton<T>>,
}

impl DropdownTrait for eframe::egui::Ui {
	fn dropdown_menu<TT: std::clone::Clone + Send + Sync + 'static>(&mut self, name: &str, default_index: usize, options: &Vec<DropdownButton<TT>>) {
		let id = egui::Id::new(name);

        let mut data: DropdownData<TT> = self.data_mut(|d| d.get_temp::<DropdownData<TT>>(id).unwrap_or(DropdownData {selected: Some(options[default_index].clone())}));
		self.horizontal(|h| {
			h.label(name);
			let title = if let Some(s) = &data.selected {
				&s.name
			} else {
				"none"
			};

			h.menu_button(format!("🔻 {d}", d = title ) , |button_content| {
				if options.len() == 0 {
					panic!("no buttons provided");
				}

				for i in options {
					if button_content.button(i.name.clone()).clicked() {
						data.selected = Some(i.clone());
						button_content.close_menu();
					}
				}
			});
		});

		self.data_mut(|w| w.insert_temp::<DropdownData<TT>>(id, data));
	}

	fn get_dropdown_selected<T>(&mut self, name: &str) -> DropdownData<T> where T: std::clone::Clone + 'static {
		let id = egui::Id::new(name);

        self.data_mut(|d| d.get_temp::<DropdownData<T>>(id).unwrap_or(DropdownData::<T> {selected: None}))
	}
}