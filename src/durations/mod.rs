// Copyright (C) 2024 DS/2, Dirk Strauss
//
// This file is part of Common Rust stuff.
//
// Common Rust stuff is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Common Rust stuff is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use chrono::Duration;
use std::error::Error;
use tracing::debug;

/// Formats a duration as a human-readable string.
///
/// # Arguments
///
/// * `d`: the duration to render.
///
/// returns: Result<String, Box<dyn Error, Global>>
///
pub fn format_duration(d: Duration) -> Result<String, Box<dyn Error>> {
    debug!("Will try to render duration: {}", d);
    let mut s = String::new();
    if d.is_zero() {
        debug!("The given duration is zero, will not render anything!");
        return Ok(s);
    }
    let weeks = d.num_weeks();
    let days = d.num_days() - d.num_weeks() * 7;
    let hours = d.num_hours() - d.num_days() * 24;
    let rounded_minutes = d.num_minutes() - d.num_hours() * 60;
    let rounded_seconds = d.num_seconds() - d.num_minutes() * 60;
    let rounded_millis = d.num_milliseconds() - d.num_seconds() * 1000;
    let rounded_micros = d.num_microseconds().unwrap_or_default() - d.num_milliseconds() * 1000;
    let rounded_nanos =
        d.num_nanoseconds().unwrap_or_default() - d.num_microseconds().unwrap_or_default() * 1000;
    debug!(
        "We have the following numbers now: w={}, d={}, h={}, m={}, s={}, ms={}, micros={}, ns={}",
        weeks,
        days,
        hours,
        rounded_minutes,
        rounded_seconds,
        rounded_millis,
        rounded_micros,
        rounded_nanos
    );

    let mut parts = vec![];

    if weeks > 0 {
        let pattern = format!("{}w", weeks);
        parts.push(pattern);
    }
    if days > 0 {
        let pattern = format!("{}d", days);
        parts.push(pattern);
    }
    if hours > 0 {
        let pattern = format!("{}h", hours);
        parts.push(pattern);
    }
    if rounded_minutes > 0 {
        let pattern = format!("{}m", rounded_minutes);
        parts.push(pattern);
    }
    if rounded_seconds > 0 {
        let pattern = format!("{}s", rounded_seconds);
        parts.push(pattern);
    }
    if rounded_millis > 0 {
        let pattern = format!("{}ms", rounded_millis);
        parts.push(pattern);
    }
    if rounded_micros > 0 {
        let pattern = format!("{}μs", rounded_micros);
        parts.push(pattern);
    }
    if rounded_nanos > 0 {
        let pattern = format!("{}ns", rounded_nanos);
        parts.push(pattern);
    }
    s = parts.join(", ");
    debug!("So, rendered result will be: {}", s);
    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::durations::format_duration;
    use chrono::Duration;
    use std::error::Error;
    use std::ops::Add;

    #[test]
    fn test_format_seconds() -> Result<(), Box<dyn Error>> {
        let dur = Duration::seconds(5);
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "5s");
        Ok(())
    }
    #[test]
    fn test_format_default_duration() -> Result<(), Box<dyn Error>> {
        let dur = Duration::default();
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "");
        Ok(())
    }

    #[test]
    fn test_format_seconds_with_millis() -> Result<(), Box<dyn Error>> {
        let dur = Duration::seconds(5).add(Duration::milliseconds(123));
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "5s, 123ms");
        Ok(())
    }

    #[test]
    fn test_format_minutes() -> Result<(), Box<dyn Error>> {
        let dur = Duration::minutes(5);
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "5m");
        Ok(())
    }

    #[test]
    fn test_format_minutes_with_seconds() -> Result<(), Box<dyn Error>> {
        let dur = Duration::minutes(5).add(Duration::seconds(23));
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "5m, 23s");
        Ok(())
    }

    #[test]
    fn test_format_hours_with_minutes_with_seconds() -> Result<(), Box<dyn Error>> {
        let dur = Duration::hours(1)
            .add(Duration::minutes(23))
            .add(Duration::seconds(45));
        let s1 = format_duration(dur)?;
        assert_eq!(s1, "1h, 23m, 45s");
        Ok(())
    }
}
