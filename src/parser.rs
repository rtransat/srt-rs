use crate::Time;
use nom::{
    bytes::complete::tag,
    character::complete::{u16, u8},
    sequence::{preceded, tuple},
    *,
};

pub fn srt_time(input: &str) -> IResult<&str, Time> {
    let (input, values) = tuple((
        u8,
        preceded(tag(":"), u8),
        preceded(tag(":"), u8),
        preceded(tag(","), u16),
    ))(input)?;
    Ok((
        input,
        Time {
            hour: values.0,
            minute: values.1,
            second: values.2,
            millisecond: values.3,
        },
    ))
}
