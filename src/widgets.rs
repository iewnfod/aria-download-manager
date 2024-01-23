use eframe::egui::Ui;

use crate::data::CLICK_TO_COPY_PROMPT;

pub fn click_copy_label<T>(ui: &mut Ui, text: T)
where T: ToString
{
	let text = text.to_string();
	if text.is_empty() {
		ui.label("");
		return;
	}
	if ui.selectable_label(false, text.clone())
	.on_hover_text(CLICK_TO_COPY_PROMPT).clicked() {
		ui.output_mut(|p| {
			p.copied_text = text.clone();
		});
	}
}
