#[cfg(not(target_os = "windows"))]
use ansi_term::Color::*;

#[cfg(target_os = "windows")]
pub fn prompt<S: Into<String>>(s: S) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn prompt<S: Into<String>>(s: S) -> String {
    Blue.bold().paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
pub fn err<S: Into<String>>(s: S) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn err<S: Into<String>>(s: S) -> String {
    Red.paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
pub fn res<S: Into<String>>(s: S) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn res<S: Into<String>>(s: S) -> String {
    Green.paint(s.into()).to_string()
}