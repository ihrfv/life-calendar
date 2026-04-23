pub use cli_printer::CliPrinter;
pub use life_calendar::LifeCalendar;

mod cli_printer;
mod life_calendar;

pub(crate) const WEEKS_COUNT: u8 = 52;
