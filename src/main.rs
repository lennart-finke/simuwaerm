use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::{
    style::{Color, Print, SetForegroundColor},
    ExecutableCommand,
    QueueableCommand,
    event::Event,
    event::KeyCode,
    cursor,
    terminal,
    event::{read, poll}
};

#[allow(unused_parens)]

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();

    // change for different plate dimensions
    const WIDTH : usize = 25;
    const HEIGHT : usize = 50;

    let mut language : &str = "en";
    let mut lambda : f32 = 0.1;
    const SCALE : f32= 1.001;

    let mut cursor = [0, 0];
    let mut pause : bool = false;

    let mut plate: Vec<f32> = vec![125.; WIDTH * HEIGHT];
    let mut plate_p : Vec<f32>;

    loop {
        stdout.queue(terminal::Clear(terminal::ClearType::All)).ok();

        stdout.queue(cursor::MoveTo(0,6)).ok();

        if (language == "de") {
            stdout.queue(Print("     SIMUWAERM v0.01\n")).ok();
            stdout.queue(Print("     --------- -----\n")).ok();
            stdout.queue(Print("     Cursor kann mit Pfeiltasten bewegt werden. \n")).ok();
            stdout.queue(Print("     h - erwärmen | + - Lambda erhöhen    | Esc - Beenden\n")).ok();
            stdout.queue(Print("     k - abkühlen | - - Lambda verringern | p   - Pausieren\n\n")).ok();
            stdout.queue(Print(format!("     \tLambda = {:.2}\n", lambda))).ok();
        }

        else if (language == "en") {
            stdout.queue(Print("     SIMUWAERM v0.01\n")).ok();
            stdout.queue(Print("     --------- -----\n")).ok();
            stdout.queue(Print("     Cursor is moved with arrow keys. \n")).ok();
            stdout.queue(Print("     h - heat | + - increase Lambda    | Esc - Exit\n")).ok();
            stdout.queue(Print("     k - cool | - - decrease Lambda    | p   - Pause\n\n")).ok();
            stdout.queue(Print(format!("     \tLambda = {:.2}\n", lambda))).ok();
        }

        for i in 0..WIDTH {

            stdout.queue(Print("\n     ")).ok();

            for j in 0..HEIGHT {
                if [j, i] == cursor {
                    stdout.queue(SetForegroundColor(Color::Rgb{r:100, g: 250, b: 100})).ok();
                }
                else {
                    stdout.queue(SetForegroundColor(Color::Rgb{r:(plate[i + WIDTH * j] as u8), g: 100, b: (255. - plate[i + WIDTH * j]) as u8})).ok();
                }

                stdout.queue(Print('█')).ok();
            }
        }

        if plate[cursor[1] +  WIDTH * cursor[0]] > 0. {
            if (language == "de") {
                println!("\n     Temperatur am Cursor: {:.5} °K", plate[cursor[1] +  WIDTH * cursor[0]] + 158.5863008);
            }

            else if (language == "en") {
                println!("\n     Temperature at Cursor: {:.5} °K", plate[cursor[1] +  WIDTH * cursor[0]] + 158.5863008);
            }
        }
        else {
            if (language == "de") {
                println!("\n     Temperatur am Cursor: {:.5} °K", 158.5863008 * SCALE.powf(plate[cursor[1] +  WIDTH * cursor[0]]));
            }

            else if (language == "en") {
                println!("\n     Temperature at Cursor: {:.5} °K", 158.5863008 * SCALE.powf(plate[cursor[1] +  WIDTH * cursor[0]]));
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
        stdout.queue(cursor::MoveTo(15,4)).ok(); // The cursor is still displayed, so we move it onto somewhere pretty
        stdout.flush().ok();

        plate = plate_p.to_vec();

        crossterm::terminal::enable_raw_mode().ok();

        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();

            if (event == Event::Key(KeyCode::Left.into())) {
                if cursor[0] > 0    {
                    cursor[0] -= 1;
                }
            }

            else if (event == Event::Key(KeyCode::Right.into())) {
                if cursor[0] < HEIGHT - 1 {
                    cursor[0] += 1;
                }
            }

            else if (event == Event::Key(KeyCode::Up.into())) {
                if cursor[1] > 0 {
                    cursor[1] -= 1;
                }
            }

            else if (event == Event::Key(KeyCode::Down.into())) {
                if cursor[1] < WIDTH - 1 {
                    cursor[1] += 1;
                }
            }

            else if (event == Event::Key(KeyCode::Char('h').into())) {
                plate[cursor[1] +  WIDTH * cursor[0]] += 50.
            }

            else if (event == Event::Key(KeyCode::Char('c').into()) || event == Event::Key(KeyCode::Char('k').into())) {
                plate[cursor[1] +  WIDTH * cursor[0]] -= 50.
            }

            else if (event == Event::Key(KeyCode::Char('+').into())) {
                lambda += 0.01;
            }

            else if (event == Event::Key(KeyCode::Char('-').into())) {
                lambda -= 0.01;
            }

            else if (event == Event::Key(KeyCode::Char('p').into())) {
                pause = !pause;
            }

            else if (event == Event::Key(KeyCode::Char('l').into())) {
                if (language == "de") {
                    language = "en";
                }

                else if (language == "en") {
                    language = "de";
                }
            }

            else if (event == Event::Key(KeyCode::Esc.into())) {
                break;
            }

        } else {
            // No Input was given
        }

        crossterm::terminal::disable_raw_mode().ok();
    }

    crossterm::terminal::disable_raw_mode().ok();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();
}
