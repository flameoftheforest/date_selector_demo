use crate::{consts::SLIDER_LENGTH, drawable::Drawable};

use super::DatePickerDemoApp;

impl DatePickerDemoApp {
  pub fn setup(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let preference = egui::ThemePreference::Dark;
    ctx.set_theme(preference);
    ctx.style_mut(|style| {
      style.spacing.slider_width = SLIDER_LENGTH;
    });

    self.state = Self::draw;
  }

  pub fn draw(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      // The top panel is often a good place for a menu bar:

      egui::menu::bar(ui, |ui| {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
          ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
              ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
          });
          ui.add_space(16.0);
        }

        egui::widgets::global_theme_preference_buttons(ui);
      });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      // The central panel the region left after adding TopPanel's and SidePanel's
      let center = ui.max_rect().center();

      // date_selector is only interested in the center info for rect
      // it will overflow this rect if the required size is > then the rect
      let rect = egui::Rect::from_center_size(center, egui::Vec2::new(2., 2.));
      
      self.set_draw_zone(ui, rect);
      self.date_selector.draw(ui, rect);
      self.date_selector.draw_debug(ui, &rect);
      self.date_selector.draw_print(ui, format!("\ndrawn rect: {:?}", self.date_selector.get_drawn_rect()));

      ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        powered_by_egui_and_eframe(ui);
        egui::warn_if_debug_build(ui);
      });
    });
  }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
  ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Powered by ");
    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
    ui.label(" and ");
    ui.hyperlink_to(
      "eframe",
      "https://github.com/emilk/egui/tree/master/crates/eframe",
    );
    ui.label(".");
  });
}
