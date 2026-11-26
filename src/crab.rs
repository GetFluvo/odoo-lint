//! `odl check --crab`: when nothing is found, Ferris rolls a lint roller over
//! the terminal. Only on an interactive terminal, never in CI.

use std::io::{self, Write};
use std::time::Duration;

/// Ferris pushing a lint roller to the right.
#[rustfmt::skip]
const CRAB: [&str; 3] = [
    r"    _~^~^~_",
    r"\) /  o o  \ (/--.",
    r"  '_   v   _'   [##]",
];

/// How wide the crab and its roller are.
const WIDTH: usize = 20;

/// The pause between two frames.
pub const FRAME: Duration = Duration::from_millis(20);

/// Whether the sweep may play: stdout is a terminal that moves the cursor,
/// and no CI job is running.
pub fn enabled() -> bool {
    use std::io::IsTerminal;
    let set = |name| std::env::var_os(name).is_some_and(|value| !value.is_empty());
    std::io::stdout().is_terminal() && !set("CI") && std::env::var_os("TERM").is_none_or(|term| term != "dumb")
}

/// The terminal's width from `COLUMNS`, else 80, at most 100.
pub fn columns() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(80)
        .min(100)
}

/// Whether there is a bit of lint at column `column` of the floor.
fn lint(column: usize) -> Option<char> {
    (column * 7 % 5 == 3).then(|| ['.', ',', '\'', '`'][column % 4])
}

/// The crab at column `x` of a terminal `columns` wide: the floor behind it is
/// clean, the floor ahead still has lint.
pub fn frame(x: usize, columns: usize) -> [String; 3] {
    CRAB.map(|line| format!("{}{line}", " ".repeat(x))).map(|mut line| {
        if line.chars().count() == x + WIDTH {
            line.extend((x + WIDTH..columns.saturating_sub(1)).map(|column| lint(column).unwrap_or(' ')));
        }
        line
    })
}

/// Play the sweep on `out`: the crab rolls from the left edge to the right
/// one and stays there.
pub fn sweep(out: &mut impl Write, columns: usize, pause: Duration) -> io::Result<()> {
    let last = columns.saturating_sub(WIDTH + 1);
    for x in 0..=last {
        if x > 0 {
            write!(out, "\x1b[{}A", CRAB.len())?;
        }
        for line in frame(x, columns) {
            write!(out, "\r{line}\x1b[K\n")?;
        }
        out.flush()?;
        if x < last {
            std::thread::sleep(pause);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crab_is_as_wide_as_its_roller() {
        assert_eq!(CRAB.iter().map(|line| line.chars().count()).max(), Some(WIDTH));
    }

    #[test]
    fn floor_has_lint_only_ahead_of_the_roller() {
        let [_, _, floor] = frame(30, 80);
        let (behind, ahead) = floor.split_at(30 + WIDTH);
        assert!(behind.trim_start().starts_with("'_"));
        assert!(!behind[..30].contains(|c: char| !c.is_whitespace()));
        assert!(ahead.contains(|c: char| ".,'`".contains(c)));
        assert!(floor.chars().count() < 80);
    }

    #[test]
    fn crab_ends_at_the_right_edge_on_a_clean_floor() {
        let mut out = Vec::new();
        sweep(&mut out, 40, Duration::ZERO).unwrap();
        let out = String::from_utf8(out).unwrap();
        // 20 frames: the first draws, the other 19 move up and redraw.
        assert_eq!(out.matches("\x1b[3A").count(), 19);
        let last: Vec<&str> = out.rsplit("\x1b[3A").next().unwrap().lines().collect();
        assert_eq!(last[2], format!("\r{}  '_   v   _'   [##]\x1b[K", " ".repeat(19)));
    }

    #[test]
    fn narrow_terminal_shows_one_frame() {
        let mut out = Vec::new();
        sweep(&mut out, 10, Duration::ZERO).unwrap();
        let out = String::from_utf8(out).unwrap();
        assert!(!out.contains("\x1b[3A"));
        assert_eq!(out.lines().count(), 3);
    }
}
