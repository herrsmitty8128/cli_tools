/// MIT License
///
/// Copyright (c) 2023 herrsmitty8128
///
/// Permission is hereby granted, free of charge, to any person obtaining a copy
/// of this software and associated documentation files (the "Software"), to deal
/// in the Software without restriction, including without limitation the rights
/// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
/// copies of the Software, and to permit persons to whom the Software is
/// furnished to do so, subject to the following conditions:
///
/// The above copyright notice and this permission notice shall be included in all
/// copies or substantial portions of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
/// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
/// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
/// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
/// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
/// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
/// SOFTWARE.
use cli_tools::pbar::{BarChar, Message, ProgressBar};
use cli_tools::text::{print_samples, Style};
use std::{sync::mpsc, thread};

fn main() {
    println!("Here is a list of all the text styles:");
    print_samples();

    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || {
        for n in 1..=1000000 {
            let limit = (n as f64).sqrt() as u32;
            for i in 2..=limit {
                if n % i == 0 {
                    break;
                }
            }
            if n % 1000 == 0 {
                // don't need to update the progess bar every time
                tx.send(Message::Percent(n as f32 / 1000000.0_f32)).unwrap();
            }

            if n == 250000 {
                tx.send(Message::TextStyle(Style::Blue)).unwrap();
            }

            if n == 500000 {
                tx.send(Message::TextStyle(Style::Green)).unwrap();
                tx.send(Message::ShowBrackets(true)).unwrap();
                tx.send(Message::TrailingChar(BarChar::LowLine)).unwrap();
            }

            if n == 750000 {
                // change the message half way through
                tx.send(Message::TextStyle(Style::Red)).unwrap();
                tx.send(Message::Label("Update ")).unwrap();
            }
        }
    });

    println!("Calculating prime numbers...");

    let mut pbar: ProgressBar = ProgressBar::new("My Progress Bar ");
    pbar.set_interval(3);
    pbar.set_style(Style::Italic);
    pbar.listen(&rx);

    println!("\nDone working!");
}
