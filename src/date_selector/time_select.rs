use crate::drawable::{Drawable, DrawableSetGet};

#[derive(Debug, Clone)]
pub struct HourMinSelect {
  selected: i32,
  bg_rect: egui::Rect,
}

impl Drawable for HourMinSelect {
  fn draw(&mut self, ui: &mut egui::Ui, _rect: egui::Rect) {
    let bg_rect = self.bg_rect.clone();
    self.draw_bg(ui, &bg_rect);

    let resp = ui
      .horizontal(|ui| {
        ui.add_space(8.);
        ui.add(
          egui::Slider::new(&mut self.selected, 0..=((60 * 24) - 1))
            .custom_formatter(|n, _| {
              let n = n as i32;
              to_time_string(n)
            })
            .custom_parser(|s| {
              let parts: Vec<&str> = s.split(':').collect();
              if parts.len() == 2 {
                parts[0]
                  .parse::<i32>()
                  .and_then(|h| parts[1].parse::<i32>().map(|m| (h * 60 + m) as f64))
                  .ok()
              } else {
                None
              }
            }),
        )
      })
      .response;

    self.bg_rect = resp.rect;
  }
}

impl DrawableSetGet for HourMinSelect {
  fn get_data(&self) -> String {
    self.to_string()
  }
}

impl HourMinSelect {
  pub fn default() -> Self {
    Self {
      selected: 13 * 60,
      bg_rect: egui::Rect::NOTHING,
    }
  }
  pub fn get_hour(&self) -> usize {
    (self.selected / 60) as usize
  }
  pub fn get_minute(&self) -> usize {
    (self.selected % 60) as usize
  }
}

impl std::fmt::Display for HourMinSelect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", to_time_string(self.selected))
  }
}

fn to_time_string(v: i32) -> String {
  let hours = v / 60;
  let mins = v % 60;
  format!("{hours:02}:{mins:02}")
}
