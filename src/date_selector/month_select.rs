use crate::{drawable::{Drawable, DrawableSetGet}, util::make_text};


macro_rules! add_selectable_label {
  ($self:expr, $ui:expr, $($month:tt),+) => {
    $(
      if $ui
        .add(egui::SelectableLabel::new(
          $self.selected == MonthEnum::$month,
          MonthEnum::$month.to_text(),
        ))
        .clicked() {
        $self.selected = MonthEnum::$month;
      }
    )+
  };
}

#[derive(Debug, Clone)]
pub struct MonthSelect {
  selected: MonthEnum,
  bg_rect: egui::Rect,
}

impl Drawable for MonthSelect {
  fn draw(&mut self, ui: &mut egui::Ui, _rect: egui::Rect) {
    let bg_rect = self.bg_rect.clone();
    self.draw_bg(ui, &bg_rect);

    let resp_0 = ui
      .horizontal(|ui| {
        add_selectable_label!(self, ui, jan, feb, mar, apr, may, jun);
      })
      .response;
    let resp_1 = ui
      .horizontal(|ui| {
        add_selectable_label!(self, ui, jul, aug, sep, oct, nov, dec);
      })
      .response;
    self.bg_rect = egui::Rect::from_min_max(resp_0.rect.min, resp_1.rect.max);
    // self.draw_debug(&rect, ui);
  }
}

impl DrawableSetGet for MonthSelect {
  fn get_data(&self) -> String {
    self.selected.to_str().to_string()
  }
}

impl MonthSelect {
  pub fn get_selected(&self) -> MonthEnum {
    self.selected.clone()
  }

  pub fn default() -> Self {
    Self {
      selected: MonthEnum::mar,
      bg_rect: egui::Rect::NOTHING,
    }
  }
}

impl std::fmt::Display for MonthSelect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.selected.to_str())
  }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MonthEnum {
  jan,
  feb,
  mar,
  apr,
  may,
  jun,
  jul,
  aug,
  sep,
  oct,
  nov,
  dec,
}

impl MonthEnum {
  pub fn to_value(&self) -> usize {
    match *self {
      Self::jan => 1,
      Self::feb => 2,
      Self::mar => 3,
      Self::apr => 4,
      Self::may => 5,
      Self::jun => 6,
      Self::jul => 7,
      Self::aug => 8,
      Self::sep => 9,
      Self::oct => 10,
      Self::nov => 11,
      Self::dec => 12,
    }
  }

  pub fn to_text(&self) -> egui::RichText {
    make_text(self.to_str())
  }

  fn to_str(&self) -> &str {
    [
      "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ][self.to_value() - 1]
  }

  pub fn try_from(v: usize) -> Option<Self> {
    match v {
      1 => Some(Self::jan),
      2 => Some(Self::feb),
      3 => Some(Self::mar),
      4 => Some(Self::apr),
      5 => Some(Self::may),
      6 => Some(Self::jun),
      7 => Some(Self::jul),
      8 => Some(Self::aug),
      9 => Some(Self::sep),
      10 => Some(Self::oct),
      11 => Some(Self::nov),
      12 => Some(Self::dec),
      _ => None
    }
  }

  pub fn try_from_str(v: &str) -> Option<Self> {
    [
      "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
    .iter()
    .enumerate()
    .find(|vv| *vv.1 == v)
    .map(|(v, _)| match v + 1 {
      1 => Self::jan,
      2 => Self::feb,
      3 => Self::mar,
      4 => Self::apr,
      5 => Self::may,
      6 => Self::jun,
      7 => Self::jul,
      8 => Self::aug,
      9 => Self::sep,
      10 => Self::oct,
      11 => Self::nov,
      12 => Self::dec,
      _ => panic!(""),
    })
  }
}
