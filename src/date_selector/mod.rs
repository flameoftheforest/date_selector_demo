mod day_select;
mod month_select;
mod time_select;
use day_select::DaySelect;
use month_select::MonthSelect;
use time_select::HourMinSelect;

use super::{
  drawable::{make_get, make_get_set, make_set, Drawable, DrawableSetGet},
  util::make_text,
};

const DEFAULT_YEAR: usize = 1980;
const DEFAULT_MONTH: usize = 8;
const DEFAULT_DAY: usize = 8;
const DEFAULT_HOUR: usize = 18;
const DEFAULT_MIN: usize = 28;

impl Drawable for DateSelector {
  fn draw(&mut self, ui: &mut egui::Ui, draw_rect: egui::Rect) {
    let rect = match self.drawn_rect != egui::Rect::NOTHING {
      true => {
        self.drawn_rect.set_center(draw_rect.center());
        self.drawn_rect
      }
      _ => {
        let mut rect = draw_rect;
        rect.set_width(rect.width() * 0.5);
        rect
      }
    };
    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(rect), |ui| {
      let height = ui.spacing().interact_size.y * 1.1;
      let label_width = ui.spacing().interact_size.x * 1.25;
      let field_width = label_width;

      let strip = egui_extras::StripBuilder::new(ui)
        .sizes(
          egui_extras::Size::Absolute {
            initial: height,
            range: egui::Rangef { min: 0., max: 0.1 },
          },
          5,
        )
        .vertical(|mut strip| {
          // row 1
          strip.strip(|builder| {
            builder
              .size(egui_extras::Size::Absolute {
                initial: label_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .size(egui_extras::Size::Absolute {
                initial: field_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .horizontal(|mut strip| {
                strip.cell(|ui| {
                  ui.label(make_text("Year:"));
                });
                strip.cell(|ui| {
                  let mut year_str = self.year.to_string();
                  if ui
                    .add(egui::TextEdit::singleline(&mut year_str).font(egui::TextStyle::Body))
                    .lost_focus()
                  {
                    let year = year_str.parse::<usize>().unwrap();
                    if year > 999 && year <= 9999 {
                      self.year = year;
                    }
                  }
                });
              });
          });

          // row 2
          strip.strip(|builder| {
            builder
              .size(egui_extras::Size::Absolute {
                initial: label_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .size(egui_extras::Size::Absolute {
                initial: field_width * 4.,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .horizontal(|mut strip| {
                strip.cell(|ui| {
                  ui.label(make_text("Month:"));
                });
                strip.cell(|ui| {
                  self.month_selector.draw(ui, egui::Rect::NOTHING);
                  self.month = self.month_selector.get_selected().to_value();
                });
              });
          });

          // row 3
          strip.strip(|builder| {
            builder
              .size(egui_extras::Size::Absolute {
                initial: label_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .size(egui_extras::Size::Absolute {
                initial: field_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .horizontal(|mut strip| {
                strip.cell(|ui| {
                  ui.label(make_text("Day:"));
                });
                strip.cell(|ui| {
                  self
                    .day_selector
                    .set_year(self.year);
                  self
                    .day_selector
                    .set_month(self.month);
                  self.day_selector.draw(ui, egui::Rect::NOTHING);
                  self.day = self.day_selector.get_day();
                });
              });
          });

          // row 4
          strip.strip(|builder| {
            builder
              .size(egui_extras::Size::Absolute {
                initial: label_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .size(egui_extras::Size::Absolute {
                initial: field_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .horizontal(|mut strip| {
                //
                strip.cell(|ui| {
                  ui.label(make_text("Time:"));
                });
                strip.cell(|ui| {
                  self.time_selector.draw(ui, egui::Rect::NOTHING);
                  self.hour = self.time_selector.get_hour();
                  self.min = self.time_selector.get_minute();
                });
              });
          });

          // row 5
          strip.strip(|builder| {
            builder
              .size(egui_extras::Size::Absolute {
                initial: label_width + field_width,
                range: egui::Rangef { min: 0., max: 0.1 },
              })
              .horizontal(|mut strip| {
                strip.cell(|ui| {
                  if self.drawn_button_rect != egui::Rect::NOTHING {
                    self
                      .drawn_button_rect
                      .set_left(rect.center().x - self.drawn_button_rect.width() / 2.);
                    self.drawn_button_rect.set_top(ui.max_rect().top());
                    ui.allocate_new_ui(
                      egui::UiBuilder::new().max_rect(self.drawn_button_rect),
                      |ui| {
                        let resp = ui.add_sized(
                          [label_width + field_width, height],
                          egui::Button::new(make_text("Start")),
                        );
                        self.drawn_button_rect = resp.rect;
                        if resp.clicked() {
                          self.triggered = true;
                        }
                      },
                    );
                    ui.advance_cursor_after_rect(self.drawn_button_rect);
                  } else {
                    // init
                    self.drawn_button_rect = ui
                      .add_sized(
                        [label_width + field_width, height],
                        egui::Button::new(make_text("Start")),
                      )
                      .rect;
                  }
                });
              });
          });
        });

      self.drawn_rect = strip.rect;
      // self.draw_debug(&strip.rect, ui);
    });
    ui.advance_cursor_after_rect(self.drawn_rect);
  }
}

impl DateSelector {
  make_get!(pub, triggered, bool);
  pub fn get_drawn_rect(&self) -> egui::Rect { self.drawn_rect }
}

impl DrawableSetGet for DateSelector {
  make_get_set!(size, f32);
  make_get!(data, String);
  make_get!(name, String);
  make_set!(name, &str);
}

impl std::fmt::Display for DateSelector {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}, {}, {}{}{}T{}:{}",
      self.name, self.size, self.year, self.month, self.day, self.hour, self.min,
    )
  }
}

impl DateSelector {
  pub fn default() -> Self {
    Self {
      size: 1.,
      data: String::new(),
      name: String::new(),
      year: DEFAULT_YEAR,
      month: DEFAULT_MONTH,
      day: DEFAULT_DAY,
      hour: DEFAULT_HOUR,
      min: DEFAULT_MIN,
      triggered: false,
      drawn_rect: egui::Rect::NOTHING,
      drawn_button_rect: egui::Rect::NOTHING,
      month_selector: MonthSelect::default(),
      day_selector: DaySelect::default(),
      time_selector: HourMinSelect::default(),
    }
  }
}

pub struct DateSelector {
  size: f32, // (0..=1) to parent rect
  data: String,
  name: String,
  year: usize,
  month: usize,
  day: usize,
  hour: usize,
  min: usize,
  triggered: bool,
  drawn_rect: egui::Rect,
  drawn_button_rect: egui::Rect,
  month_selector: MonthSelect,
  day_selector: DaySelect,
  time_selector: HourMinSelect,
}
