#[cfg(not(target_os = "windows"))]
use ansi_term::Color;

const PROMPT_COLOR: Color = Color::Fixed(157);
const CARROT_COLOR: Color = Color::Fixed(251);

#[cfg(target_os = "windows")]
pub fn prompt(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn prompt(s: impl Into<String>) -> String {
    PROMPT_COLOR.bold().paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
pub fn carrot(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn carrot(s: impl Into<String>) -> String {
    CARROT_COLOR.bold().paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
pub fn err(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
pub fn err(s: impl Into<String>) -> String {
    Color::Red.paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
pub fn number(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn number(s: impl Into<String>) -> String {
    Color::Yellow.paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn string(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn string(s: impl Into<String>) -> String {
    Color::Green.paint(s.into()).to_string()
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn boolean(s: impl Into<String>) -> String {
    s.into()
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
pub fn boolean(s: impl Into<String>) -> String {
    Color::Yellow.paint(s.into()).to_string()
}
