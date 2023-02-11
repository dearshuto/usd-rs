use eframe::egui;
use egui::{
    plot::{Legend, Line, LineStyle, Plot, PlotPoints},
    Color32,
};
use usd_rs::{AsciiReader, LoadState, Path, StreamReader};

fn main() -> Result<(), eframe::Error> {
    let data = include_str!("cube_animated.usda");
    let mut stream_reader = StreamReader::new(data.as_bytes());
    let mut reader = usd_rs::AsciiReader::new(&mut stream_reader);
    reader.read(LoadState::TopLevel);
    reader.reconstruct_stage();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new(reader))),
    )
}

struct MyApp<'a> {
    name: String,
    age: u32,
    reader: AsciiReader<'a>,
}

impl<'a> MyApp<'a> {
    fn new(reader: AsciiReader<'a>) -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            reader,
        }
    }

    fn circle(&self) -> Line {
        let Some(stage) =  self.reader.try_get_stage() else {
                println!("failed to get stage");
                todo!()
        };
        let path = Path::new("/Cube", "");
        let Some(prim) = stage.find_prim_at_path(&path) else {
            todo!()
        };
        let Some(xform) =  prim.as_x_form() else {
            todo!()
        };

        let op = xform.get_op(0);
        let time_samples = op.get_time_samples().unwrap();
        let circle_points: PlotPoints = (0..time_samples.size())
            .map(|i| {
                let t = time_samples.get_time(i);
                let v = time_samples.get_value(i).as_float();
                [t, v as f64]
            })
            .collect();
        Line::new(circle_points)
            .color(Color32::from_rgb(100, 200, 100))
            .style(LineStyle::Solid)
            .name("circle")
    }
}

impl<'a> eframe::App for MyApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            let plot = Plot::new("lines_demo").legend(Legend::default());
            let _ = plot.show(ui, |plot_ui| {
                plot_ui.line(self.circle());
            });
        });
    }
}
