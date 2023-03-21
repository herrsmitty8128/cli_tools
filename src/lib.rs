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

/// A module for displaying different text styles through stdout on the command line.
pub mod text {

    use std::fmt::Display;

    /// An enum of text styles that can be used with stdout.
    #[derive(Debug, Clone, Copy)]
    pub enum Style {
        Regular = 0,
        Bold = 1,
        Faint = 2,
        Italic = 3,
        Underline = 4,
        Highlight = 7,
        StrikeThrough = 9,
        DoubleUnderline = 21,
        DarkGray = 30,
        Orange = 31,
        Green = 32,
        Yellow = 33,
        Blue = 34,
        Cyan = 35,
        LightBlue = 36,
        BlackBg = 40,
        RedBg = 41,
        GreenBg = 42,
        YellowBg = 43,
        BlueBg = 44,
        CyanBg = 45,
        LightBlueBg = 46,
        WhiteBg = 47,
        Red = 91,
    }

    impl Display for Style {
        /// Formats a text::Style as a string so it can printed to stdout
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("\x1b[{}m", *self as usize))
        }
    }

    /// Prints a list of all text::Styles to stdout.
    pub fn print_samples() {
        println!("{}Style::Regular{}", Style::Regular, Style::Regular);
        println!("{}Style::Bold{}", Style::Bold, Style::Regular);
        println!("{}Style::Faint{}", Style::Faint, Style::Regular);
        println!("{}Style::Italic{}", Style::Italic, Style::Regular);
        println!("{}Style::Highlight{}", Style::Highlight, Style::Regular);
        println!("{}Style::Underline{}", Style::Underline, Style::Regular);
        println!(
            "{}Style::StrikeThrough{}",
            Style::StrikeThrough,
            Style::Regular
        );
        println!(
            "{}Style::DoubleUnderline{}",
            Style::DoubleUnderline,
            Style::Regular
        );
        println!("{}Style::DarkGray{}", Style::DarkGray, Style::Regular);
        println!("{}Style::Orange{}", Style::Orange, Style::Regular);
        println!("{}Style::Green{}", Style::Green, Style::Regular);
        println!("{}Style::Yellow{}", Style::Yellow, Style::Regular);
        println!("{}Style::Blue{}", Style::Blue, Style::Regular);
        println!("{}Style::Cyan{}", Style::Cyan, Style::Regular);
        println!("{}Style::LightBlue{}", Style::LightBlue, Style::Regular);
        println!("{}Style::BlackBg{}", Style::BlackBg, Style::Regular);
        println!("{}Style::RedBg{}", Style::RedBg, Style::Regular);
        println!("{}Style::GreenBg{}", Style::GreenBg, Style::Regular);
        println!("{}Style::YellowBg{}", Style::YellowBg, Style::Regular);
        println!("{}Style::BlueBg{}", Style::BlueBg, Style::Regular);
        println!("{}Style::CyanBg{}", Style::CyanBg, Style::Regular);
        println!("{}Style::LightBlueBg{}", Style::LightBlueBg, Style::Regular);
        println!("{}Style::WhiteBg{}", Style::WhiteBg, Style::Regular);
        println!("{}Style::Red{}", Style::Red, Style::Regular);
    }
}

/// The pbar module contains an implementation of a process bar for use in command
/// line programs. It can be used in single or multiple threads.
pub mod pbar {

    use crate::text::{self, Style};
    use std::fmt;
    use std::fmt::Display;
    use std::sync::mpsc;
    use std::{io::Write, thread, time};

    /// An enum of unicode characters commonly used in a command line progress bar.
    #[derive(Debug, Copy, Clone)]
    pub enum BarChar {
        NumberSign = 0x0023,
        EqualSign = 0x003D,
        LowLine = 0x005F,
        FullBlock = 0x2588,  //default leading char
        LightShade = 0x2591, //default trailing char
        MediumShade = 0x2592,
        DarkShade = 0x2593,
        BlackSquare = 0x25A0,
        WhiteSquare = 0x25A1,
        SquareWithHorizontalFill = 0x25A4,
        SquareWithVerticalFill = 0x25A5,
        SquareWithOrthogonalCrosshatchFill = 0x25A6,
        SquareWithUpperLeftToLowerRightFill = 0x25A7,
        SquareWithUpperRightToLowerLeftFill = 0x25A8,
        SquareWithDiagonalCrosshatchFill = 0x25A9,
        BlackSmallSquare = 0x25AA,
        WhiteSmallSquare = 0x25AB,
        BlackRectangle = 0x25AC,
        WhiteRectangle = 0x25AD,
        BlackVerticalRectangle = 0x25AE,
        WhiteVerticalRectangle = 0x25AF,
        BlackParallelogram = 0x25B0,
        WhiteParallelogram = 0x25B1,
        WhiteMediumSquare = 0x25FB,
        BlackMediumSquare = 0x25FC,
        WhiteMediumSmallSquare = 0x25FD,
        BlackMediumSmallSquare = 0x25FE,
        PlayingCardAceOfSpades = 0x1F0A1,
    }

    impl BarChar {
        /// Converts a BarChar into the equivalent char. This function should never
        /// encounter an error, but if it does then '\u{0}' will be returned.
        fn to_char(self) -> char {
            std::char::from_u32(self as u32).unwrap_or('\u{0}')
        }
    }

    impl From<char> for BarChar {
        /// Creates and returns a new BarChar object from a char. If the char cannot be
        /// converted into a BarChar object, then a BarChar::FullBlock object is returned.
        fn from(c: char) -> Self {
            match c as i32 {
                0x0023 => BarChar::NumberSign,
                0x003D => BarChar::EqualSign,
                0x005F => BarChar::LowLine,
                0x2588 => BarChar::FullBlock,  //default leading char
                0x2591 => BarChar::LightShade, //default trailing char
                0x2592 => BarChar::MediumShade,
                0x2593 => BarChar::DarkShade,
                0x25A0 => BarChar::BlackSquare,
                0x25A1 => BarChar::WhiteSquare,
                0x25A4 => BarChar::SquareWithHorizontalFill,
                0x25A5 => BarChar::SquareWithVerticalFill,
                0x25A6 => BarChar::SquareWithOrthogonalCrosshatchFill,
                0x25A7 => BarChar::SquareWithUpperLeftToLowerRightFill,
                0x25A8 => BarChar::SquareWithUpperRightToLowerLeftFill,
                0x25A9 => BarChar::SquareWithDiagonalCrosshatchFill,
                0x25AA => BarChar::BlackSmallSquare,
                0x25AB => BarChar::WhiteSmallSquare,
                0x25AC => BarChar::BlackRectangle,
                0x25AD => BarChar::WhiteRectangle,
                0x25AE => BarChar::BlackVerticalRectangle,
                0x25AF => BarChar::WhiteVerticalRectangle,
                0x25B0 => BarChar::BlackParallelogram,
                0x25B1 => BarChar::WhiteParallelogram,
                0x25FB => BarChar::WhiteMediumSquare,
                0x25FC => BarChar::BlackMediumSquare,
                0x25FD => BarChar::WhiteMediumSmallSquare,
                0x25FE => BarChar::BlackMediumSmallSquare,
                0x1F0A1 => BarChar::PlayingCardAceOfSpades,
                _ => BarChar::FullBlock,
            }
        }
    }

    impl Display for BarChar {
        /// Formats a BarChar object for printing.
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use std::fmt::Write;
            f.write_char(self.to_char())
        }
    }

    /// A message object that can be passed from a thread to a ProgressBar object
    /// through a ```mspc::channel```. The message tells the progress bar to update the
    /// corresponding member variable and then call ```self.show()```. See
    /// ```ProgressBar::listen()``` for more information.
    #[derive(Debug, Copy, Clone)]
    pub enum Message {
        Percent(f32),
        Label(&'static str),
        ShowPercentage(bool),
        ShowBrackets(bool),
        Length(u32),
        LeadingChar(BarChar),
        TrailingChar(BarChar),
        Interval(u64),
        TextStyle(text::Style),
    }

    /// An object used to display a progress bar on the command line.
    #[derive(Debug, Copy, Clone)]
    pub struct ProgressBar {
        length: u32,
        leading_char: char,
        trailing_char: char,
        show_percentage: bool,
        show_brackets: bool,
        interval: time::Duration,
        percent: f32,
        label: &'static str,
        prev_text_len: usize,
        text_style: text::Style,
    }

    impl fmt::Display for ProgressBar {
        /// Formats a progress bar for printing based on its internal configuration.
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut s: String = String::from(self.label);
            if self.show_brackets {
                s.push('[');
            }
            let count: u32 = (self.length as f32 * self.percent).round() as u32;
            for _ in 0..count {
                s.push(self.leading_char);
            }
            for _ in count..self.length {
                s.push(self.trailing_char);
            }
            if self.show_brackets {
                s.push(']');
            }
            if self.show_percentage {
                use std::fmt::Write;
                write!(s, " {:.1}%", self.percent * 100.0).unwrap();
            }
            fmt.write_str(&s)
        }
    }

    impl Default for ProgressBar {
        /// Creates a new progress bar with default settings as follows:
        /// - length of the bar, excluding the label and percentage: 50 characters
        /// - leading character (the left side of the bar that represents the percentage complete): BarChar::FullBlock
        /// - trailing character: BarChar::LightShade
        /// - show the percentage: true
        /// - show the brackets at either end of the bar: false
        /// - time interval between refreshing the toolbar on the screen: 0 milliseconds
        /// - percentage complete: 0%
        /// - text label: "Percent complete "
        /// - text style: text::Style::Regular
        fn default() -> Self {
            Self::new("Percent complete ")
        }
    }

    impl ProgressBar {
        /// Creates a new ```ProgressBar``` object with the default configuration and a label based
        /// on the ```label``` argument. See ```ProgressBar::default()``` for more information.
        pub fn new(label: &'static str) -> Self {
            Self {
                length: 50,
                leading_char: BarChar::FullBlock.to_char(),
                trailing_char: BarChar::LightShade.to_char(),
                show_percentage: true,
                show_brackets: false,
                interval: time::Duration::from_millis(0),
                percent: 0.0,
                label,
                prev_text_len: 0,
                text_style: text::Style::Regular,
            }
        }

        /// Returns the current text::Style
        pub fn style(&self) -> text::Style {
            self.text_style
        }

        /// Sets the text::Style used when printing the progress bar to stdout.
        pub fn set_style(&mut self, style: text::Style) {
            self.text_style = style;
        }

        /// Returns the length of the progress bar, excluding the label, brackets, and percentage, if any.
        /// This is the length of the actual bar itself.
        pub fn length(&self) -> u32 {
            self.length
        }

        /// Sets the length of the progress bar, excluding the label, brackets, and percentage, if any.
        /// This is the length of the actual bar itself.
        pub fn set_length(&mut self, length: u32) {
            self.length = length;
        }

        /// Returns the unicode character that ```self.show()``` uses in the body of the progress bar.
        /// The leading character comprises the left side of the progress bar that represents the
        /// portion of the task or activity that has been completed.
        pub fn leading_char(&self) -> BarChar {
            BarChar::from(self.leading_char)
        }

        /// Sets the unicode character that ```self.show()``` uses in the body of the progress bar.
        /// The leading character comprises the left side of the progress bar that represents the
        /// portion of the task or activity that has been completed.
        pub fn set_leading_char(&mut self, c: BarChar) {
            self.leading_char = c.to_char();
        }

        /// Returns the unicode character that ```self.show()``` uses in the body of the progress bar.
        /// The trailing character comprises the right side of the progress bar that represents the
        /// portion of the task or activity that has not been completed.
        pub fn trailing_char(&self) -> BarChar {
            BarChar::from(self.trailing_char)
        }

        /// Sets the unicode character that ```self.show()``` uses in the body of the progress bar.
        /// The trailing character comprises the right side of the progress bar that represents the
        /// portion of the task or activity that has not been completed.
        pub fn set_trail_char(&mut self, c: BarChar) {
            self.trailing_char = c.to_char();
        }

        /// If ```true``` is passed, then ```self.show()``` will print the percentage at the end of the progress bar.
        pub fn show_percentage(&mut self, show: bool) {
            self.show_percentage = show;
        }

        /// If ```true``` is passed, then ```self.show()``` will print the brackets at the beginning and end of the progress bar.
        pub fn show_brackets(&mut self, show: bool) {
            self.show_brackets = show;
        }

        /// Returns the number of millliseconds that ```self.show()``` will sleep after printing the progress bar to stdout.
        /// The default value is 0 milliseconds. This function is provided as a way of "slowing down" updates to the progress bar.
        /// It is used primarily for testing or simply to provide a better user experience.
        pub fn interval(&self) -> time::Duration {
            self.interval
        }

        /// Sets the number of millliseconds that ```self.show()``` will sleep before returning to its caller.
        /// The default value is 0 milliseconds. This function is provided as a way of "slowing down" updates to the progress bar.
        /// It is used primarily for testing or simply to provide a better user experience.
        pub fn set_interval(&mut self, interval: u64) {
            self.interval = time::Duration::from_millis(interval);
        }

        /// Returns the current percentage completed of the progress bar as a number 0.0 <= n <= 1.0.
        pub fn percent(&self) -> f32 {
            self.percent
        }

        /// Sets the current percentage completed of the progress bar as a number 0.0 <= n <= 1.0.
        pub fn set_percent(&mut self, percent: f32) {
            self.percent = f32::min(percent.abs(), 1.0);
        }

        /// Returns the text label to the left of the progress bar
        pub fn label(&self) -> &'static str {
            self.label
        }

        /// Sets the text label to the left of the progress bar
        pub fn set_label(&mut self, msg: &'static str) {
            self.label = msg;
        }

        /// A private function used to calculate and save the overall length of the progress bar,
        /// incuding the label, brackets, and percentage immediately after printing to stdout.
        /// This value is used to clear the row on the command line immediately prior to printing
        /// the progress bar to stdout. This improves the appearance of the progress bar and
        /// ensures that it is displayed properly in the event changes are made it its configuration
        /// in between printing to stdout.
        fn save_line_length(&mut self) {
            let mut n: usize = self.length as usize + self.label.len();
            if self.show_brackets {
                n += 2;
            }
            if self.show_percentage {
                n += if self.percent >= 1.0 {
                    7
                } else if self.percent >= 0.1 {
                    6
                } else {
                    5
                }
            };
            self.prev_text_len = n
        }

        /// A private function that clears the previous progress bar printed to stdout. This function is
        /// called by ```ProgressBar::show()``` immediately prior to printing the progress bar to stdout.
        /// See ```ProgressBar::save_line_length()``` for more information.
        fn clear_line(&self) {
            print!("\r");
            for _ in 0..self.prev_text_len {
                print!(" ");
            }
            print!("\r");
        }

        /// Listens for messages on ```rx```, executes the message and calls ```self.show()```.
        ///
        /// ```
        /// use cli_tools::pbar::{Progress, ProgressBar};
        /// use std::{sync::mpsc, thread};
        ///
        /// fn myfunc() {
        ///     let mut pbar: ProgressBar = ProgressBar::default();
        ///
        ///     // establish a channel that our thread can use to send messages
        ///     // to the progress bar in the main thread
        ///     let (tx, rx) = mpsc::channel::<Message>();
        ///
        ///     thread::spawn(move || {
        ///         // Create a thread that does some time-consuming work
        ///         for n in 1..=1000000 {
        ///             let limit = (n as f64).sqrt() as u32;
        ///             for i in 2..=limit {
        ///                 if n % i == 0 {
        ///                     break;
        ///                 }
        ///             }
        ///             if n % 1000 == 0 {
        ///                 // don't need to update the progress bar each time
        ///                 tx.send(Progress::from(n as f32 / 1000000.0_f32)).unwrap();
        ///             }
        ///         }
        ///     });
        ///
        ///     pbar.show_thread(&rx);
        ///
        ///     println!("\nDone working!");
        /// }
        /// ```
        pub fn listen(&mut self, rx: &mpsc::Receiver<Message>) {
            for msg in rx {
                match msg {
                    Message::LeadingChar(c) => self.set_leading_char(c),
                    Message::TrailingChar(c) => self.set_trail_char(c),
                    Message::Interval(i) => self.set_interval(i),
                    Message::Length(l) => self.set_length(l),
                    Message::Label(msg) => self.set_label(msg),
                    Message::Percent(p) => self.set_percent(p),
                    Message::ShowPercentage(show) => self.show_percentage(show),
                    Message::ShowBrackets(show) => self.show_brackets(show),
                    Message::TextStyle(style) => self.set_style(style),
                }
                self.show();
            }
        }

        /// Clears the current line of text on the command line, resets the cursor to the beginning
        /// of the line, and prints the progress bar. If ```self.interval``` is set to a non-zero
        /// value, then it will sleep for as many milliseconds before returning to the caller. This
        /// is intended to give the caller the ability to slow down the loop for presentation
        /// purposes if desired.
        pub fn show(&mut self) {
            self.clear_line();
            print!("{}{}{}", self.text_style, self, Style::Regular);
            self.save_line_length();
            std::io::stdout().flush().unwrap();
            thread::sleep(self.interval);
        }
    }
}
