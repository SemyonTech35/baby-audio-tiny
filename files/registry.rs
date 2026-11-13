//! Windows registry read helpers for Baby Audio Tiny.

pub const APP_KEY: &str = "Software\\Baby\\Baby Audio Tiny";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Baby Audio Tiny";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Baby Audio Tiny") },
        RegValue { name: "Version",     value: String::from("v4.3.17") },
        RegValue { name: "Channel",     value: String::from("beta") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Baby Audio Tiny") },
        RegValue { name: "Publisher",       value: String::from("Baby") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Baby Audio Tiny\\uninstall.exe\"") },
    ]
}
