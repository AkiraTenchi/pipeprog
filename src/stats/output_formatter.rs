use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent, Stylize},
};
use std::io::Write;

pub(crate) fn output_progress(output: &mut impl Write, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{} ", bytes).with(Color::Rgb {
        r: 255,
        g: 204,
        b: 255,
    }));
    let elapsed = style::style(elapsed.with(Color::Rgb {
        r: 102,
        g: 204,
        b: 255,
    }));
    let rate = style::style(format!(" [{:.0}b/s]", rate).with(Color::Rgb {
        r: 204,
        g: 255,
        b: 153,
    }));
    let _ = execute!(
        output,
        cursor::MoveToColumn(0),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = output.flush();
}

pub(crate) trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::output_progress;
    use super::TimeOutput;
    use crossterm::style;
    use crossterm::style::{Color, Stylize};

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3603_u64, "1:00:03"),
            (3723_u64, "1:02:03"),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }

    #[test]
    fn output_progress_test() {
        let mut output: Vec<u8> = Vec::new();
        output_progress(&mut output, 12, "10".to_string(), 400.0);
        let expected = style::style("12 ".with(Color::Rgb {
            r: 255,
            g: 204,
            b: 255,
        }))
        .to_string();
        let expected1 = style::style("10".with(Color::Rgb {
            r: 102,
            g: 204,
            b: 255,
        }))
        .to_string();
        let expected2 = style::style(" [400b/s]".with(Color::Rgb {
            r: 204,
            g: 255,
            b: 153,
        }))
        .to_string();

        let res = format!("{}{}{}", expected, expected1, expected2).into_bytes();

        assert_eq!(output[4..], res)
    }
}
