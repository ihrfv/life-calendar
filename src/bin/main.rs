use argh::FromArgs;
use life_calendar::CliPrinter;
use life_calendar::LifeCalendar;

#[derive(FromArgs)]
/// LifeCalendar input parsing utility
struct Cli {
    #[argh(option)]
    /// year
    year: i32,

    #[argh(option)]
    /// month
    month: u8,

    #[argh(option)]
    /// day
    day: u8,
}

fn main() {
    let cli: Cli = argh::from_env();
    match LifeCalendar::new(cli.year, cli.month, cli.day) {
        Ok(lc) => {
            let cli_printer = CliPrinter::new(&lc);
            cli_printer.print_life_in_weeks();
        }
        Err(e) => eprintln!("{e}"),
    }
}
