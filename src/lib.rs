use std::fmt::Display;

#[derive(Debug)]
pub struct Srt {
    pub subtitles: Vec<SubTitle>,
}

#[derive(Debug)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millisecond: u16,
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour
            && self.minute == other.hour
            && self.second == other.second
            && self.millisecond == other.millisecond
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hour, self.minute, self.second, self.millisecond
        )
    }
}

#[derive(Debug)]
pub struct SubTitle {
    pub index: u32,
    pub start: Time,
    pub end: Time,
    pub text: String,
}

impl Display for SubTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} --> {}\n{}\n",
            self.index, self.start, self.end, self.text
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::srt_time;

    use super::*;

    #[test]
    fn subtitle_implements_display() {
        let srt = SubTitle {
            index: 1,
            start: Time {
                hour: 0,
                minute: 1,
                second: 20,
                millisecond: 123,
            },
            end: Time {
                hour: 0,
                minute: 1,
                second: 24,
                millisecond: 145,
            },
            text: String::from("Hello world"),
        };

        assert_eq!(
            format!("{srt}"),
            "1\n00:01:20,123 --> 00:01:24,145\nHello world\n"
        );
    }

    #[test]
    fn parse_time() {
        let time = Time {
            hour: 0,
            minute: 0,
            second: 23,
            millisecond: 123,
        };

        assert_eq!(srt_time("00:00:23,123").unwrap(), ("", time));
    }
}
