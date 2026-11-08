//! Windows registry read helpers for Fabfilter Pro G.

pub const APP_KEY: &str = "Software\\Fabfilter\\Fabfilter Pro G";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Fabfilter Pro G";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Fabfilter Pro G") },
        RegValue { name: "Version",     value: String::from("v4.6.0") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Fabfilter Pro G") },
        RegValue { name: "Publisher",       value: String::from("Fabfilter") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Fabfilter Pro G\\uninstall.exe\"") },
    ]
}
