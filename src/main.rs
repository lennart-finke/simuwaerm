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

fn main() {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All));


    const width : usize = 25;
    const height : usize = 50;

    let mut lambda : f32 = 0.1;
    const scale : f32= 1.001;

    let mut cursor = [0, 0];
    let mut pause : bool = false;

    let mut plate: Vec<f32> = vec![125.; width * height];
    let mut plate_p : Vec<f32>;
    plate[12] = 500.;
    plate[11 + width] = 500.;

    loop {

        stdout.queue(terminal::Clear(terminal::ClearType::All));

        stdout.queue(cursor::MoveTo(0,6));
        stdout.queue(Print("     SIMUWAERM v0.01\n"));
        stdout.queue(Print("     --------- -----\n"));
        stdout.queue(Print("     Cursor kann mit Pfeiltasten bewegt werden. \n"));
        stdout.queue(Print("     h - erwärmen | + - Lambda erhöhen    | Esc - Beenden\n"));
        stdout.queue(Print("     k - abkühlen | - - Lambda verringern | p   - Pausieren\n\n"));
        stdout.queue(Print(format!("     \tLambda = {:.2}\n", lambda)));
        for i in 0..width {

            stdout.queue(Print("\n     "));

            for j in 0..height {
                if [j, i] == cursor {
                    stdout.queue(SetForegroundColor(Color::Rgb{r:100, g: 250, b: 100}));
                }
                else {
                    stdout.queue(SetForegroundColor(Color::Rgb{r:(plate[i + width * j] as u8), g: 100, b: (255. - plate[i + width * j]) as u8}));
                }

                stdout.queue(Print('█'));
            }
        }

        if plate[cursor[1] +  width * cursor[0]] > 0. {
            println!(" Temperatur am Cursor: {:.10} °K", plate[cursor[1] +  width * cursor[0]] + 158.5863008);
        }
        else {
            println!(" Temperatur am Cursor: {:.10} °K", 158.5863008 * scale.powf(plate[cursor[1] +  width * cursor[0]]));
        }


        plate_p = plate.to_vec();

        for i in 1..(height - 1) {
            if pause {continue;}
            for j in 1..(width - 1) {
                let neighbor : f32 = ((plate[1 + j + width * i] + plate[1 + j + width * (i - 1)] + plate[1 + j + width * (i + 1)] + plate[j - 1 + width * i] + plate[j + width * (i + 1)] + plate[j - 1 + width * (i + 1)] + plate[j - 1 + width * (i - 1)] + plate[j + width * (i - 1)] ) / 8.) ;
                plate_p[j + width * i] -= (lambda * ((plate[j + width * i] - neighbor)));
            }
        }
        stdout.flush();

        plate = plate_p.to_vec();

        crossterm::terminal::enable_raw_mode();
        if poll(Duration::from_millis(100)).unwrap() {
            let event = read().unwrap();

            if event == Event::Key(KeyCode::Left.into()) {
                if cursor[0] > 0    {
                    cursor[0] -= 1;
                }
            }

            else if event == Event::Key(KeyCode::Right.into()) {
                if cursor[0] < height - 1 {
                    cursor[0] += 1;
                }
            }

            else if event == Event::Key(KeyCode::Up.into()) {
                if cursor[1] > 0 {
                    cursor[1] -= 1;
                }
            }

            else if event == Event::Key(KeyCode::Down.into()) {
                if cursor[1] < width - 1 {
                    cursor[1] += 1;
                }
            }

            else if event == Event::Key(KeyCode::Char('h').into()) {
                plate[cursor[1] +  width * cursor[0]] += 50.
            }

            else if (event == Event::Key(KeyCode::Char('c').into()) || event == Event::Key(KeyCode::Char('k').into())) {
                plate[cursor[1] +  width * cursor[0]] -= 50.
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

            else if (event == Event::Key(KeyCode::Esc.into())) {
                break;
            }

        } else {
            // Timeout expired, no `Event` is available
        }

        crossterm::terminal::disable_raw_mode();
    }

    crossterm::terminal::disable_raw_mode();
    stdout.execute(terminal::Clear(terminal::ClearType::All));
}
