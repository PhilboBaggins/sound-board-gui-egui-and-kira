use eframe::egui;
use eframe::egui::CentralPanel;
use egui_extras::{TableBuilder, Column};

use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::StaticSoundData,
};

const APP_NAME: &str = "sound-board-gui";

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    manager: AudioManager::<DefaultBackend>,
    sound_data: StaticSoundData,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        // TODO: Lots of them
        let sample_file = "sample.wav";
        let sound_data = StaticSoundData::from_file(sample_file).unwrap();

        Self {
            manager: manager,
            sound_data: sound_data,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            rounding: 10.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            ..Default::default()
        };

        let num_cols = 4; // TODO: Use this to make the columns below
        let col_width = 100.0 / num_cols as f32;

        let num_rows = 10;
        let row_height = 100.0 / num_rows as f32;

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::exact(col_width)) //.column(Column::auto())
                .column(Column::exact(col_width)) //.column(Column::auto())
                .column(Column::exact(col_width)) //.column(Column::auto())
                .column(Column::exact(col_width)) //.column(Column::auto())
                .body(|mut body| {
                    for _ in 0..num_rows {
                        body.row(row_height, |mut row| {
                            for _ in 0..num_cols {
                                row.col(|ui| {
                                    if ui.button("AAA").clicked() {
                                        self.manager.play(self.sound_data.clone()).unwrap();
                                    }
                                });
                            }
                        });
                    }
                });
            });
    }
}
