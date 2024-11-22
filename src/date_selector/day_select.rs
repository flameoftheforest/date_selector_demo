use chrono::Datelike;
use crate::{drawable::{Drawable, DrawableSetGet}, util::make_text};

macro_rules! add_selectable_label {
  ($self:expr, $ui:expr, $days:expr) => {
    for day in $days {
      if *day == 0 {
        $ui.add_sized($self.select_rect.size(), egui::Label::new(" "));
      } else {
        let day_s = match *day < 10 as usize {
          true => format!("0{day}"),
          _ => format!("{day}"),
        };
        let resp = $ui.add_sized(
          $self.select_rect.size(),
          egui::SelectableLabel::new($self.selected == *day, day_s),
        );
        if resp.rect.area() > $self.select_rect.area() {
          $self.select_rect = resp.rect;
        }
        if resp.clicked() {
          $self.selected = *day;
        }
      }
    }
  };
}

impl Drawable for DaySelect {
  fn draw(&mut self, ui: &mut egui::Ui, draw_rect: egui::Rect) {
    let bg_rect = self.bg_rect.clone();
    self.draw_bg(ui, &bg_rect);

    let (num_of_days, day_of_week) = self.num_of_days();

    let mut render_data = vec![];
    for _idx in 0..day_of_week {
      render_data.push(0);
    }
    for day in 1..=num_of_days {
      render_data.push(day);
    }

    let mut resps = vec![];
    resps.push(
      ui.horizontal(|ui| {
        for day in ["Sun", "Mon", "Tue", "Wed", "Thur", "Fri", "Sat"] {
          let r = ui.add_sized(self.select_rect.size(), egui::Label::new(make_text(day)));
          if r.rect.area() > self.select_rect.area() {
            self.select_rect = r.rect;
          }
        }
      })
      .response,
    );
    for days in render_data.chunks(7) {
      resps.push(
        ui.horizontal(|ui| {
          add_selectable_label!(self, ui, days);
        })
        .response,
      );
    }
    let top_row = resps.first().unwrap().rect;
    let left_top = top_row.min;
    let mut right_bottom = resps.last().unwrap().rect.max;
    if right_bottom.x < top_row.max.x {
      right_bottom.x = top_row.max.x;
    }
    self.bg_rect = egui::Rect::from_min_max(left_top, right_bottom);
  }
}

impl DaySelect {
  fn num_of_days(&self) -> (usize, usize) {
    let start_date =
      chrono::NaiveDate::from_ymd_opt(self.selected_year as i32, self.selected_month as u32, 1)
        .unwrap();
    let (year, month) = match self.selected_month {
      12 => (self.selected_year + 1, 1),
      _ => (self.selected_year, self.selected_month + 1),
    };
    let day_of_week = start_date.weekday().num_days_from_sunday();
    let end_date = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, 1).unwrap();
    (
      end_date.signed_duration_since(start_date).num_days() as usize,
      day_of_week as usize,
    )
  }

  pub fn set_year(&mut self, which: usize) {
    self.selected_year = which;
  }

  pub fn set_month(&mut self, which: usize) {
    self.selected_month = match which < 12 && which >= 1 {
      true => which,
      _ => self.selected_month,
    }
  }

  pub fn get_day(&self) -> usize { self.selected }

  pub fn default() -> Self {
    Self {
      selected_year: 1980,
      selected_month: 3,
      selected: 3,
      bg_rect: egui::Rect::NOTHING,
      select_rect: egui::Rect::from_center_size(egui::Pos2::new(0., 0.), egui::Vec2::new(0.1, 0.1)),
    }
  }
}

impl DrawableSetGet for DaySelect {
}

impl std::fmt::Display for DaySelect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.selected)
  }
}

#[derive(Debug, Clone)]
pub struct DaySelect {
  selected_year: usize,
  selected_month: usize,
  selected: usize,
  bg_rect: egui::Rect,
  select_rect: egui::Rect,
}
