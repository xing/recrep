/// The Printing trait describes things which can print things
pub trait Printing {
    /// Printing things can print text
    fn print(&self, text: String);
}

/// A StdOutPrinter can print to the console
pub struct StdOutPrinter {}

impl Printing for StdOutPrinter {
    /// Prints text to standard out
    fn print(&self, text: String) {
        println!("{}", text);
    }
}
