use crate::WEEKS_COUNT;
use crate::life_calendar::LifeCalendar;

pub struct CliPrinter<'lc> {
    lc: &'lc LifeCalendar,
}

impl<'lc> CliPrinter<'lc> {
    pub fn new(lc: &'lc LifeCalendar) -> Self {
        CliPrinter { lc }
    }

    pub fn print_life_in_weeks(&self) {
        let (years, weeks) = self.lc.passed();
        println!("Life Calendar in weeks.");
        Self::print_separator();
        Self::print_weeks_layout();
        Self::print_separator();

        for i in 1..years {
            Self::print_year(i, WEEKS_COUNT);
            if i != 0 && i % 10 == 0 {
                println!();
            }
        }
        Self::print_year(years, weeks);
        // print the rest of the decade
        if years % 10 != 0 {
            for i in years..(years / 10 + 1) * 10 {
                Self::print_year(i + 1, 0);
            }
        }

        Self::print_separator();
        Self::print_weeks_layout();
        Self::print_separator();
    }

    fn print_separator() {
        println!("{:-<1$}", "", WEEKS_COUNT as usize * 3 + 3);
    }

    fn print_weeks_layout() {
        print!("  |");
        for i in 1..WEEKS_COUNT + 1 {
            if i % 4 == 0 {
                print!("{i:02}|");
            } else if i % 4 == 1 {
                print!(" {i:02}");
            } else {
                print!("   ");
            }
        }
        println!();
    }

    fn print_year(year: u64, weeks: u8) {
        print!("{:02}|", year);
        for i in 1..weeks + 1 {
            print!(" x");

            if i % 4 == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        for i in weeks + 1..WEEKS_COUNT + 1 {
            print!(" .");
            if i % 4 == 0 {
                print!("|");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
