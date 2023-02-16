use rayon::prelude::*;

#[derive(Default)]
pub struct RayonTestApp {
    numbers: Vec<u64>,

    loop_time: Vec<instant::Duration>,
    iter_time: Vec<instant::Duration>,
    par_iter_time: Vec<instant::Duration>,
}

impl RayonTestApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn measure(&mut self, count: u64) {
        if self.numbers.len() as u64 != count {
            self.numbers = (0..count).collect();
        }
        let numbers = &self.numbers;

        let start = instant::Instant::now();
        let mut loop_sum = 0;
        for &n in numbers {
            loop_sum += n;
        }
        let loop_time = start.elapsed();

        let start = instant::Instant::now();
        let iter_sum: u64 = numbers.iter().sum();
        let iter_time = start.elapsed();

        let start = instant::Instant::now();
        let par_iter_sum: u64 = numbers.par_iter().sum();
        let par_iter_time = start.elapsed();

        assert_eq!(loop_sum, iter_sum);
        assert_eq!(loop_sum, par_iter_sum);

        self.loop_time.push(loop_time);
        self.iter_time.push(iter_time);
        self.par_iter_time.push(par_iter_time);
    }

    fn results_ui(&self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        ui.style_mut().wrap = Some(false);

        ui.strong("elements / ns - higher is better:");
        ui.add_space(8.0);

        let format_result = |duration: &instant::Duration| {
            let num_per_ns = self.numbers.len() as f32 / (duration.as_nanos() as f32);
            format!("{num_per_ns:.2}")
        };

        TableBuilder::new(ui)
            .columns(Column::auto().at_least(100.0), 3)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("for-loop");
                });
                header.col(|ui| {
                    ui.strong("iter().sum()");
                });
                header.col(|ui| {
                    ui.strong("par_iter().sum()");
                });
            })
            .body(|mut body| {
                for (loop_time, iter_time, par_iter_time) in
                    itertools::izip!(&self.loop_time, &self.iter_time, &self.par_iter_time)
                {
                    body.row(14.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format_result(loop_time));
                        });
                        row.col(|ui| {
                            ui.label(format_result(iter_time));
                        });
                        row.col(|ui| {
                            ui.label(format_result(par_iter_time));
                        });
                    });
                }
            });
    }
}

impl eframe::App for RayonTestApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let n = 200_000_000;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("rayon test");

            ui.label(format!(
                "Measures the time it takes to sum {}M numbers",
                n / 1_000_000
            ));

            ui.horizontal(|ui| {
                if ui.button("Measure").clicked() {
                    self.measure(n);
                }
                if ui.button("Clear").clicked() {
                    self.loop_time.clear();
                    self.iter_time.clear();
                    self.par_iter_time.clear();
                }
            });

            ui.separator();

            self.results_ui(ui);
        });
    }
}
