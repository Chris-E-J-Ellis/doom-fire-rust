use super::fire_engine::FireBuffer;
use super::fire_palette;
use crossterm::{
    cursor,
    style::{Color, ResetColor, SetBackgroundColor},
    terminal, QueueableCommand,
};
use std::io::{stdout, Write};

pub struct ConsoleFireRenderer;

impl super::fire_engine::FireRenderer for ConsoleFireRenderer {
    fn render(&mut self, buffer: &FireBuffer) {
        ConsoleFireRenderer::render(buffer)
    }
}

impl Drop for ConsoleFireRenderer {
    fn drop(&mut self) {
        ConsoleFireRenderer::reset();
    }
}

impl ConsoleFireRenderer {
    pub fn new() -> ConsoleFireRenderer { ConsoleFireRenderer {} }
    pub fn render(buffer: &FireBuffer) {
        print!("\n{}", SetBackgroundColor(Color::Reset));
        let mut stdout = stdout();
        //stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap(); // Alternate clear
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        for x in 0..buffer.buffer.len() as usize {
            let pixel = buffer.buffer[x];
            let r = fire_palette::DOOM_FIRE_PALETTE[(pixel * 3) as usize];
            let g = fire_palette::DOOM_FIRE_PALETTE[(pixel * 3 + 1) as usize];
            let b = fire_palette::DOOM_FIRE_PALETTE[(pixel * 3 + 2) as usize];
            stdout
                .queue(SetBackgroundColor(Color::Rgb { r, g, b }))
                .unwrap();
            stdout.queue(crossterm::Output(" ")).unwrap();

            if x % buffer.width == (buffer.width - 1) {
                stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
                stdout.queue(crossterm::Output("\n")).unwrap();
            }
        }
        stdout.flush().unwrap();
    }

    pub fn reset() {
        let mut stdout = stdout();
        stdout.queue(ResetColor).unwrap();
        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        stdout.flush().unwrap();
    }
}
