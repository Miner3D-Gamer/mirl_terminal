//! A library for a better time interacting with the console/terminal
//!
//! This lib is a big WIP. A lot will be ported over from the python lib TGE.
//!
//! This file is terrible, more TGE lib functions need to be ported over

use std::io::{
    //stdout,
    BufRead,
    Write,
};

use mirl_graphics::u32_color_casting::ColorManipulationWithoutInput;

// use crossterm::ExecutableCommand;
// use crate::graphics::{ColorManipulation, Pixel};

// /// Clears the currently visible console
// ///
// /// # Errors
// /// Idk, this functions needs to be rewritten
// pub fn clear_console() -> std::io::Result<()> {
//     stdout()
//         .execute(crossterm::terminal::Clear(
//             crossterm::terminal::ClearType::All,
//         ))?
//         .execute(crossterm::cursor::MoveTo(0, 0))?;
//     Ok(())
// }
// /// Moves the cursor to the top
// /// # Errors
// /// Idk, this functions needs to be rewritten
// pub fn move_to_top() -> std::io::Result<()> {
//     stdout().execute(crossterm::cursor::MoveTo(0, 0))?;
//     Ok(())
// }
#[must_use]
/// Color the given text (requires the console to support the full color range)
pub fn color_text(msg: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m{msg}\x1b[0m")
}
#[must_use]
/// Color the background of the given text (requires the console to support the full color range)
pub fn color_background<T: core::fmt::Display, F: core::fmt::Display>(
    msg: &T,
    r: F,
    g: F,
    b: F,
) -> String {
    format!("\x1b[48;2;{r};{g};{b}m{msg}\x1b[0m")
}
#[must_use]
/// Color the text and color of the given string (requires the console to support the full color range)
pub fn color<T: core::fmt::Display>(msg: &str, r1: T, g1: T, b1: T, r2: T, g2: T, b2: T) -> String {
    format!("\x1b[38;2;{r1};{g1};{b1}m\x1b[48;2;{r2};{g2};{b2}m{msg}\x1b[0m")
}
#[must_use]
/// Return the 'clear' all effects marker
pub fn reset_color() -> String {
    "\x1b[0m".to_string()
}
/// Clear X lines
///
/// # Errors
/// When it cannot write to the console it errors
pub fn clear_lines(n: usize) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();

    for _ in 0..n {
        // Move cursor up one line
        write!(stdout, "\x1B[1A")?;
        // Clear the entire line
        write!(stdout, "\x1B[2K")?;
    }

    // Ensure the commands are flushed to the terminal
    stdout.flush()?;
    Ok(())
}

/// A python like input function
///
/// # Errors
/// When it cannot read the console it will error
pub fn input(msg: &str) -> std::io::Result<String> {
    let mut input = String::new();
    println!("{msg}");
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.len() - 1);
    Ok(input)
}
#[must_use]
/// Get the (full) content of the console
pub fn get_console_content(max_lines: usize) -> Vec<String> {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();

    let mut recent_lines = Vec::new();

    for line in lines.map_while(Result::ok) {
        recent_lines.push(line);
        if recent_lines.len() > max_lines {
            recent_lines.remove(0);
        }
    }

    recent_lines
}

// /// Print the pixel struct as color
// pub fn print_color(buffer: &[Pixel]) {
//     for i in buffer {
//         print!("{}", color_text("#", i.r, i.g, i.b));
//     }
// }
// // "▄"
// /// Print an image in console version using a list of Pixel structs
// pub fn print_color_v(buffer: &[Pixel], width: usize) {
//     for i in 0..buffer.len() / 2 {
//         print!(
//             "{}",
//             color(
//                 "▄",
//                 buffer[i].r,
//                 buffer[i].g,
//                 buffer[i].b,
//                 buffer[i + width].r,
//                 buffer[i + width].g,
//                 buffer[i + width].b
//             )
//         );
//         if i % width == width - 1 {
//             println!();
//         }
//     }
// }
#[must_use]
/// Convert an array of u32 to a list of console formatted color data
pub fn color_data_to_console(pixels: &[u32], width: usize, height: usize) -> Vec<String> {
    let mut output = Vec::new();
    for y in 0..height / 2 {
        let mut local = String::new();
        for x in 0..width {
            let pixel_below = pixels[x + y * width];
            let pixel = pixels[x + (y + 1) * width];
            local.push_str(&color(
                "▄",
                pixel.red(),
                pixel.green(),
                pixel.blue(),
                pixel_below.red(),
                pixel_below.green(),
                pixel_below.blue(),
            ));
        }
        output.push(local);
    }
    if height % 2 == 1 {
        let mut local = String::new();
        for x in 0..width {
            let pixel = pixels[x + (height - 1) * width];
            local.push_str(&color(
                "▄",
                pixel.red(),
                pixel.green(),
                pixel.blue(),
                0,
                0,
                0,
            ));
        }
        output.push(local);
    }

    output
}
