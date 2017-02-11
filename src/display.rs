use term_painter::ToStyle;
use term_painter::Color::*;


pub fn print_log_message(msg: &str) {
    println!("  ↘️️    {}", Yellow.paint(msg));
}

pub fn print_debug_message(msg: &str) {
    println!("  ↗️️️️    {}", BrightMagenta.paint(msg));
}
