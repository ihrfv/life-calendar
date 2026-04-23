use time::{Date, Month, OffsetDateTime};

pub struct LifeCalendar {
    bday: Date,
}

impl LifeCalendar {
    pub fn new(year: i32, month: u8, day: u8) -> Result<Self, Box<dyn std::error::Error>> {
        Self::new_with_now(year, month, day, OffsetDateTime::now_utc().date())
    }

    fn new_with_now(
        year: i32,
        month: u8,
        day: u8,
        now: Date,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let bday = Date::from_calendar_date(year, Month::try_from(month).unwrap(), day)?;
        if (bday - now).is_positive() {
            return Err("The provided bday input cannot be in the future".into());
        }
        Ok(LifeCalendar { bday })
    }

    /// This method returns a tuple of (years, weeks since last anniversary) tuple
    pub fn passed(&self) -> (u64, u8) {
        self.passed_since(OffsetDateTime::now_utc().date())
    }

    fn passed_since(&self, now: Date) -> (u64, u8) {
        if (self.bday - now).is_positive() {
            panic!("Where is your validation mister?");
        }
        let years = now.year() - self.bday.year();
        let last_anniversary = self.bday.replace_year(now.year()).unwrap();
        let weeks = (now - last_anniversary).whole_weeks();
        (years as u64, weeks as u8)
    }

    /// This method return number of days that passed
    pub fn passed_days(&self) -> i64 {
        let now = OffsetDateTime::now_utc().date();
        let passed_duration = now - self.bday;
        passed_duration.whole_days()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn date_in_past() {
        let lc = LifeCalendar::new_with_now(
            2000,
            1,
            22,
            Date::from_calendar_date(2026, Month::April, 23).unwrap(),
        )
        .unwrap();
        let (years, weeks) = lc.passed();
        assert_eq!(years, 26);
        assert_eq!(weeks, 13);
    }

    #[test]
    fn date_in_future() {
        let lc = LifeCalendar::new_with_now(
            2026,
            4,
            24,
            Date::from_calendar_date(2026, Month::April, 23).unwrap(),
        );
        assert!(lc.is_err());
    }

    #[test]
    fn same_date() {
        let lc = LifeCalendar::new_with_now(
            2026,
            4,
            23,
            Date::from_calendar_date(2026, Month::April, 23).unwrap(),
        )
        .unwrap();
        let (years, weeks) = lc.passed();
        assert_eq!(years, 0);
        assert_eq!(weeks, 0);
    }
}
