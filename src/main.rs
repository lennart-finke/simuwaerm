// author   : fi-le (she/her)
// website  : fi-le.net
// e-mail   : info @t fi-le.net
// license  : MIT

use std::io::{stdout, Write};
use std::time::Duration;
use std::env;
use crossterm::{
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
    QueueableCommand,
    event::Event,
    cursor,
    terminal,
    event::{read, poll}
};

#[allow(unused_parens)]

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();
    stdout.execute(terminal::SetTitle("SIMUWAERM v0.02")).ok();     // Inexplicably doesn't work on UNIX
    stdout.execute(terminal::SetSize(65, 40)).ok();                 // Inexplicably doesn't work on UNIX

    const WIDTH : usize = 25;
    const HEIGHT : usize = 50;

    let mut screen_dimension : (u16, u16) = terminal::size().unwrap();

    let mut language : &str = "en";
    let mut lambda : f32 = 0.1;
    const SCALE : f32= 1.001;

    let mut cursor = [0, 0];
    let mut pause : bool = false;

    let mut plate: Vec<f32> = vec![125.; WIDTH * HEIGHT];
    let mut plate_p : Vec<f32>;

    let mut counter : u8 = 0; // We refresh once in a while as UNIX terminals like to leave fragments. Windows terminals don't handle clearing the screen well, so we only do it when needed. Conditional compilation would be the better option.

    loop {
        if (screen_dimension != terminal::size().unwrap() || counter == 150 || env::consts::OS == "linux") {
            stdout.queue(terminal::Clear(terminal::ClearType::All)).ok();
            screen_dimension = terminal::size().unwrap();
            counter = 0; // Deliberate integer overflow would have been fun, but it's of course not very Rust-y.
        }

        else {
            counter += 1;
        }

        stdout.queue(cursor::MoveTo(0,3)).ok();

        if (language == "de") {
            stdout.queue(Print("     SIMUWAERM v0.02\n")).ok();
            stdout.queue(Print("     --------- -----\n")).ok();
            stdout.queue(Print("     Cursor kann mit Pfeiltasten bewegt werden.             \n")).ok();
            stdout.queue(Print("     h - erwärmen | + - Lambda erhöhen    | Esc - Beenden   \n")).ok();
            stdout.queue(Print("     k - abkühlen | - - Lambda verringern | p   - Pausieren \n\n")).ok();
            stdout.queue(Print(format!("     \tLambda = {:.2}                               \n", lambda))).ok();
        }

        else if (language == "en") {
            stdout.queue(Print("     SIMUWAERM v0.02\n")).ok();
            stdout.queue(Print("     --------- -----\n")).ok();
            stdout.queue(Print("     Cursor is moved with arrow keys.                       \n")).ok();
            stdout.queue(Print("     h - heat | + - increase Lambda    | Esc - Exit         \n")).ok();
            stdout.queue(Print("     c - cool | - - decrease Lambda    | p   - Pause        \n\n")).ok();
            stdout.queue(Print(format!("     \tLambda = {:.2}                               \n", lambda))).ok();
        }

        for i in 0..WIDTH {
            stdout.queue(Print("\n     ")).ok();

            for j in 0..HEIGHT {
                if [j, i] == cursor {
                    if (env::consts::OS == "macos") {
                        stdout.queue(SetForegroundColor(Color::AnsiValue(83))).ok();
                    }
                    else {
                        stdout.queue(SetForegroundColor(Color::Rgb{r:100, g: 250, b: 100})).ok();
                    }
                }
                else {
                    if (env::consts::OS == "macos") {
                        stdout.queue(SetForegroundColor(Color::AnsiValue(22 + (36 * ((plate[i + WIDTH * j] as u8) / 51)) + ((255. - plate[i + WIDTH * j]) as u8) / 51))).ok();
                    }
                    else {
                        stdout.queue(SetForegroundColor(Color::Rgb{r:(plate[i + WIDTH * j] as u8), g: 100, b: (255. - plate[i + WIDTH * j]) as u8})).ok();
                    }
                }

                stdout.queue(Print('█')).ok();
            }
        }

        if plate[cursor[1] +  WIDTH * cursor[0]] > 0. {
            if (language == "de") {
                println!("\n     Temperatur am Cursor: {:>5} °K                                     ", plate[cursor[1] +  WIDTH * cursor[0]] + 158.5863008);
            }

            else if (language == "en") {
                println!("\n     Temperature at Cursor: {:>5} °K                                    ", plate[cursor[1] +  WIDTH * cursor[0]] + 158.5863008);
            }
        }
        else {
            if (language == "de") {
                println!("\n     Temperatur am Cursor: {:>5} °K                                     ", 158.5863008 * SCALE.powf(plate[cursor[1] +  WIDTH * cursor[0]]));
            }

            else if (language == "en") {
                println!("\n     Temperature at Cursor: {:>5} °K                                    ", 158.5863008 * SCALE.powf(plate[cursor[1] +  WIDTH * cursor[0]]));
            }
        }

        plate_p = plate.to_vec();

        for i in 1..(HEIGHT - 1) {
            if pause {continue;}
            for j in 1..(WIDTH - 1) {
                let neighbor : f32 = ((plate[1 + j + WIDTH * i] + plate[1 + j + WIDTH * (i - 1)] + plate[1 + j + WIDTH * (i + 1)] + plate[j - 1 + WIDTH * i] + plate[j + WIDTH * (i + 1)] + plate[j - 1 + WIDTH * (i + 1)] + plate[j - 1 + WIDTH * (i - 1)] + plate[j + WIDTH * (i - 1)] ) / 8.) ;
                plate_p[j + WIDTH * i] -= (lambda * ((plate[j + WIDTH * i] - neighbor)));
            }
        }
        plate = plate_p.to_vec();

        crossterm::terminal::enable_raw_mode().ok(); // For UNIX terminals

        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();
            if let Event::Resize(_, _) = event {
                crossterm::terminal::disable_raw_mode().ok();
                continue;
            }

            let code = format!("{:?}", event).split_at(21).1.split(',').next().unwrap().to_string();

            match code.as_str() {
                "Left" => {
                    if cursor[0] > 0    {
                        cursor[0] -= 1;
                    }
                },

                "Right" => {
                    if cursor[0] < HEIGHT - 1 {
                        cursor[0] += 1;
                    }
                },

                "Up" => {
                    if cursor[1] > 0 {
                        cursor[1] -= 1;
                    }
                },

                "Down" => {
                    if cursor[1] < WIDTH - 1 {
                        cursor[1] += 1;
                    }
                }

                "Char('h')"                => plate[cursor[1] +  WIDTH * cursor[0]] += 50.,

                "Char('k')" | "Char('c')"  => plate[cursor[1] +  WIDTH * cursor[0]] -= 50.,

                "Char('+')" => lambda += 0.01,

                "Char('-')" => lambda -= 0.01,

                "Char('p')" => pause = !pause,

                "Char('l')" => {
                    if (language == "de") {
                        language = "en";
                    }

                    else if (language == "en") {
                        language = "de";
                    }
                },

                "Esc" => break,

                _ => {}
            }
        }
        crossterm::terminal::disable_raw_mode().ok();
        stdout.flush().ok();
    }
    crossterm::terminal::disable_raw_mode().ok();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();
}
