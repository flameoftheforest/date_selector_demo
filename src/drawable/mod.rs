macro_rules! make_null_set {
  ($prop:tt, $type:expr) => {
    paste::item! {
      fn [<set_ $prop>](&mut self, [<in_ $prop>]: $type) { () }
    }
  };
}
pub(crate) use make_null_set;

macro_rules! make_set {
  ($prop:tt, $type:expr) => {
    paste::item! {
      fn [<set_ $prop>](&mut self, [<in_ $prop>]: $type) { self.$prop = [<in_ $prop>].into() }
    }
  };
}
pub(crate) use make_set;

macro_rules! make_null_get {
  ($prop:tt, $type:expr, $default_val:expr) => {
    paste::item! {
      fn [<get_ $prop>](&self) -> $type { $default_val }
    }
  };
}
pub(crate) use make_null_get;

macro_rules! make_get {
  ($prop:tt, $type:expr) => {
    paste::item! {
      fn [<get_ $prop>](&self) -> $type { self.$prop.clone() }
    }
  };

  ($prefix:tt, $prop:tt, $type:expr) => {
    paste::item! {
      $prefix fn [<get_ $prop>](&self) -> $type { self.$prop.clone() }
    }
  };
}
pub(crate) use make_get;

macro_rules! make_get_set {
  ($prop:tt, $type:tt) => {
    paste::item! {
      fn [<set_ $prop>](&mut self, [<in_ $prop>]: $type) { self.$prop = [<in_ $prop>] }
      fn [<get_ $prop>](&self) -> $type { self.$prop.clone() }
    }
  };
}
pub(crate) use make_get_set;

pub trait DrawableSetGet {
  fn get_size(&self) -> f32 { 0. }
  fn get_loc(&self) -> egui::Pos2 { egui::Pos2::new(0., 0.) }
  fn get_data(&self) -> String { "".to_owned() }
  fn get_selected(&self) -> bool { false }
  fn get_name(&self) -> String { "".to_owned() }

  fn set_size(&mut self, _size: f32) { () }
  fn set_loc(&mut self, _loc: egui::Pos2) { () }
  fn set_data(&mut self, _v: &str) { () }
  fn set_selected(&mut self, _state: bool) { () }
  fn set_name(&mut self, _v: &str) { () }
}

pub trait Drawable: std::fmt::Display + DrawableSetGet {
  fn get_size_pixel(&self, draw_rect: &egui::Rect) -> f32 { draw_rect.width() * self.get_size().clone() }
  fn get_loc_pixel(&self, draw_rect: &egui::Rect) -> egui::Pos2 {
    let size = self.get_size_pixel(draw_rect);
    draw_rect.min
      + egui::Vec2 {
        x: size * 0.5,
        y: size * 0.5,
      }
      + egui::Vec2 {
        x: (draw_rect.width() - size) * self.get_loc().x,
        y: (draw_rect.height() - size) * self.get_loc().y,
      }
  }

  fn draw(&mut self, ui: &mut egui::Ui, draw_rect: egui::Rect) {
    let size = self.get_size_pixel(&draw_rect);
    let loc = self.get_loc_pixel(&draw_rect);

    let painter = ui.painter();

    painter.text(
      loc,
      egui::Align2::CENTER_CENTER,
      self.get_data().to_string(),
      egui::FontId::monospace(10.),
      egui::Color32::DARK_RED,
    );

    painter.rect_stroke(
      egui::Rect::from_center_size(loc, egui::Vec2 { x: size, y: size }).shrink(1.0),
      egui::Rounding::ZERO,
      egui::Stroke::new(1.0, egui::Color32::GREEN),
    );
    painter.circle(loc, 1.0, egui::Color32::GREEN, egui::Stroke::NONE);
  }

  fn draw_bg(&mut self, ui: &mut egui::Ui, bg_rect: &egui::Rect) {
    let painter = ui.painter();
    painter.rect_filled(
      bg_rect.clone(),
      egui::Rounding::from(2.5),
      egui::Color32::DARK_GRAY,
    );
  }

  fn draw_debug(&mut self, ui: &mut egui::Ui, rect: &egui::Rect) {
    let painter = ui.painter();
    painter.rect_stroke(
      egui::Rect::from_min_size(rect.min, egui::Vec2::new(rect.width(), rect.height())).shrink(1.0),
      egui::Rounding::ZERO,
      egui::Stroke::new(1.0, egui::Color32::YELLOW),
    );
    painter.circle(rect.center(), 1.0, egui::Color32::GREEN, egui::Stroke::NONE);
  }

  fn draw_print(&self, ui: &mut egui::Ui, s: String) {
    ui.horizontal(|ui| {ui.label(format!("{s}"));});
  }
}
