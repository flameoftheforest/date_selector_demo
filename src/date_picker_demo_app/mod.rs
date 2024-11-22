use crate::DateSelector;
mod states;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DatePickerDemoApp {
  #[serde(skip)]
  state: fn(&mut Self, &egui::Context, &mut eframe::Frame),

  #[serde(skip)]
  date_selector: DateSelector,
}

impl DatePickerDemoApp {
  pub(crate) fn set_draw_zone(&mut self, ui: &mut egui::Ui, draw_zone: egui::Rect) {
    // draw_zone is in pixels
    // tell egui that i am going to draw into this area
    // this rect will be the max_rect of the current widget
    // this rect will be the min_rect of the next widget
    // not using the response
    
    ui.allocate_rect(draw_zone, egui::Sense::click());
  }
}

impl Default for DatePickerDemoApp {
  fn default() -> Self {
    Self {
      state: Self::setup,
      date_selector: DateSelector::default(),
    }
  }
}

impl DatePickerDemoApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    // Load previous app state (if any).
    // Note that you must enable the `persistence` feature for this to work.
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }

    Default::default()
  }
}

impl eframe::App for DatePickerDemoApp {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
    // For inspiration and more examples, go to https://emilk.github.io/egui
    (self.state)(self, ctx, _frame);
  }
}
