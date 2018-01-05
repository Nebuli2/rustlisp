use std::env;
use std::fs;

pub fn user_name() -> String {
    if let Some(home_dir) = env::home_dir() {
        if let Some(name) = home_dir.file_name() {
            if let Some(s) = name.to_str() {
                return s.to_string();
            }
        }
    }
    String::from("user")
}