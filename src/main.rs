use std::{env, fmt, process};

// TODO: go to time impl block to find todo
#[derive(Default, Debug)]
struct Time {
    years: Option<u64>,
    days: Option<u64>,
    hours: Option<u64>,
    minutes: Option<u64>,
    seconds: Option<u64>,
}

impl Time {
    fn new() -> Self {
        Time::default()
    }
    fn parse_time(self, unit: Unit, time: u64) -> Self {
        match unit {
            Unit::Years => self,
            Unit::Days => {
                let years = time / 365;
                self.convert_time(Unit::Days, Unit::Years, time, 365)
                    .parse_time(Unit::Years, years)
            }
            Unit::Hours => {
                let days = time / 24;
                self.convert_time(Unit::Hours, Unit::Days, time, 24)
                    .parse_time(Unit::Days, days)
            }
            Unit::Minutes => {
                let hours = time / 60;
                self.convert_time(Unit::Minutes, Unit::Hours, time, 60)
                    .parse_time(Unit::Hours, hours)
            }
            Unit::Seconds => {
                let minutes = time / 60;
                self.convert_time(Unit::Seconds, Unit::Minutes, time, 60)
                    .parse_time(Unit::Minutes, minutes)
            }
            Unit::Invalid => self,
        }
    }
    fn convert_time(self, from_unit: Unit, to_unit: Unit, time: u64, single_unit: u64) -> Self {
        let bigger_unit = time / single_unit;
        let mut small_unit = time;
        if bigger_unit > 0 {
            small_unit = time % single_unit;
        }
        self.set_time(from_unit, small_unit)
            .set_time(to_unit, bigger_unit)
    }
    fn set_time(self, unit: Unit, value: u64) -> Self {
        match unit {
            Unit::Years => Self {
                years: Some(value),
                ..self
            },
            Unit::Days => Self {
                days: Some(value),
                ..self
            },
            Unit::Hours => Self {
                hours: Some(value),
                ..self
            },
            Unit::Minutes => Self {
                minutes: Some(value),
                ..self
            },
            Unit::Seconds => Self {
                seconds: Some(value),
                ..self
            },
            Unit::Invalid => self,
        }
    }
    // TODO: impl fn to add time structs or overload + for time struct
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut time = String::new();
        if let Some(v) = self.years {
            if v != 0 {
                time = format!("{} {} {}", time, v, "years");
            }
        }
        if let Some(v) = self.days {
            if v != 0 {
                time = format!("{} {} {}", time, v, "days");
            }
        }
        if let Some(v) = self.hours {
            if v != 0 {
                time = format!("{} {} {}", time, v, "hours");
            }
        }
        if let Some(v) = self.minutes {
            if v != 0 {
                time = format!("{} {} {}", time, v, "minutes");
            }
        }
        if let Some(v) = self.seconds {
            if v != 0 {
                time = format!("{} {} {}", time, v, "seconds");
            }
        }
        write!(f, "{}", time)
    }
}

enum Unit {
    Years,
    Days,
    Hours,
    Minutes,
    Seconds,
    Invalid,
}

impl Unit {
    fn parse_unit(t: &str) -> (Self, u64) {
        let start = t.chars().nth(0).unwrap();
        if (start as u8) > 57 || (start as u8) < 48 {
            return (Self::Invalid, 0);
        }
        let parse_value = |s| {
            t.split(s)
                .nth(0)
                .unwrap()
                .parse::<u64>()
                .unwrap_or_default()
        };
        if t.ends_with("yr") || t.ends_with("y") || t.ends_with("years") {
            return (Self::Years, parse_value('y'));
        } else if t.ends_with("dy") || t.ends_with("d") || t.ends_with("days") {
            return (Self::Days, parse_value('d'));
        } else if t.ends_with("hr") || t.ends_with("h") || t.ends_with("hours") {
            return (Self::Hours, parse_value('h'));
        } else if t.ends_with("min") || t.ends_with("m") || t.ends_with("minutes") {
            return (Self::Minutes, parse_value('m'));
        } else if t.ends_with("sec") || t.ends_with("s") || t.ends_with("seconds") {
            return (Self::Seconds, parse_value('s'));
        } else {
            return (Self::Invalid, 0);
        }
    }
}

fn main() {
    let err_msg = || {
        let program_name = env::args().nth(0).unwrap();
        eprint!(
            "maybe you forgot to provide an input or provided an invalid input\n\n USAGE: {0} `your input`\n\n Examples:\n\t{0} 1200s\n\t{0} 547min\n\t{0} 2300yr",
            program_name
        );
        process::exit(1);
    };
    match env::args().nth(1) {
        Some(t) => {
            let (unit, time) = Unit::parse_unit(&t);
            if let Unit::Invalid = unit {
                err_msg();
            }
            let normalized_time = Time::new();
            println!(
                "{} is same as {}\n\nAn year is considered as 365 days in calculation",
                &t,
                normalized_time.parse_time(unit, time)
            );
        }
        None => {
            err_msg();
        }
    }
}
