use nom::combinator::all_consuming;
use nom::Finish;
use parse::{Direction, Instruction};
use eframe::egui;

mod parse;

fn main() {
    
    println!("Hello, world!");
    
    let instructions = include_str!("sample.txt")
        .lines()
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
        .collect::<Vec<_>>();
    
    // for ins in instructions {
    //     println!("{ins:?}");
    // }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 — Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp { instructions })),
    );

}

struct MyApp {
    instructions: Vec<Instruction>,
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instructions:");
            for ins in &self.instructions {
                let arrow = match ins.dir {
                    Direction::Up => "⬆",
                    Direction::Down => "⬇",
                    Direction::Right => "➡",
                    Direction::Left => "⬅",
                };
                ui.label(arrow.repeat(ins.dist as _));
            }
        });
    }
}
