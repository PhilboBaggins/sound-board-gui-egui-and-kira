use eframe::egui;
use eframe::egui::CentralPanel;

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
    sound_data: Vec<StaticSoundData>,
    grid_width: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        // TODO: Generate this list from a directory or config file
        let sound_data = vec![
            // 0 to 3
            StaticSoundData::from_file("sample_0.wav").unwrap(),
            StaticSoundData::from_file("sample_1.wav").unwrap(),
            StaticSoundData::from_file("sample_2.wav").unwrap(),
            StaticSoundData::from_file("sample_3.wav").unwrap(),

            // 4 to 7
            StaticSoundData::from_file("sample_0.wav").unwrap(),
            StaticSoundData::from_file("sample_1.wav").unwrap(),
            StaticSoundData::from_file("sample_2.wav").unwrap(),
            StaticSoundData::from_file("sample_3.wav").unwrap(),
        ];

        // TODO: Make this dynamic
        let grid_width = 4;

        Self {
            manager: manager,
            sound_data: sound_data,
            grid_width: grid_width,
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

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {

            ctx.style_mut(|style| {
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(32.0, egui::FontFamily::Proportional),
                );
            });

            egui::Grid::new("grid").show(ui, |ui| {
                for i in 0..self.sound_data.len() {
                    let sound_name = format!("Sound {}", i); // TODO: Use sample name
                    if ui.button(sound_name).clicked() {
                        self.manager.play(self.sound_data[i].clone()).unwrap();
                    }
                    if i % self.grid_width == self.grid_width - 1 {
                        ui.end_row();
                    }
                }
            });
        });
    }
}
