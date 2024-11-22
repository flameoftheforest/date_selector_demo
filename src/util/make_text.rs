pub fn make_text(text: &str) -> egui::RichText {
  egui::RichText::new(text).text_style(egui::TextStyle::Body)
}