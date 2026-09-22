//! Tiny localization layer.
//!
//! The language is picked from the `Language` key in the config file
//! (`auto`, `en-US` or `zh-CN`) and falls back to the system UI language.
//! Only the languages listed here are supported, everything else uses English.

use std::sync::OnceLock;

use windows::Win32::Globalization::GetUserDefaultUILanguage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese
}

/// All user visible strings of the app.
pub struct Strings {
    // flyout
    pub adjust_brightness: &'static str,
    pub settings: &'static str,
    pub refresh: &'static str,
    // tray icon
    pub change_brightness: &'static str,
    pub quit: &'static str,
    // settings window
    pub settings_title: &'static str,
    pub general: &'static str,
    pub run_on_startup: &'static str,
    pub restore_saved_brightness: &'static str,
    pub controls: &'static str,
    pub icon_scroll: &'static str,
    pub hotkeys_intro: &'static str,
    pub increase_brightness: &'static str,
    pub decrease_brightness: &'static str,
    pub monitors: &'static str,
    pub monitors_hint: &'static str,
    pub failed_to_detect_monitors: &'static str,
    pub advanced: &'static str,
    pub autostart_priority: &'static str,
    // message boxes
    pub failed_enable_icon_scroll: &'static str,
    pub failed_register_hotkey: &'static str,
    // monitor names
    pub internal_display: &'static str
}

static ENGLISH: Strings = Strings {
    adjust_brightness: "Adjust Brightness",
    settings: "Settings",
    refresh: "Refresh",
    change_brightness: "Change Brightness",
    quit: "Quit",
    settings_title: "Rusty Twinkle Tray Settings",
    general: "General",
    run_on_startup: "Automatically run on startup",
    restore_saved_brightness: "Automatically restore saved brightness",
    controls: "Controls",
    icon_scroll: "Adjust the brightness of all displays by scrolling over the tray icon",
    hotkeys_intro: "Adjust the brightness of all displays by pressing the following hotkeys:",
    increase_brightness: "Increase brightness",
    decrease_brightness: "Decrease brightness",
    monitors: "Monitors",
    monitors_hint: "Rename your displays. Leave a field empty to use its default name.",
    failed_to_detect_monitors: "Failed to detect monitors.",
    advanced: "Advanced",
    autostart_priority: "Use higher autostart priority (requires admin permissions)",
    failed_enable_icon_scroll: "Failed to enable icon scroll",
    failed_register_hotkey: "Failed to register hotkey",
    internal_display: "Internal Display"
};

static CHINESE: Strings = Strings {
    adjust_brightness: "调节亮度",
    settings: "设置",
    refresh: "刷新",
    change_brightness: "调节亮度",
    quit: "退出",
    settings_title: "Rusty Twinkle Tray 设置",
    general: "通用",
    run_on_startup: "开机时自动启动",
    restore_saved_brightness: "自动恢复上次保存的亮度",
    controls: "控制方式",
    icon_scroll: "在托盘图标上滚动滚轮，调节所有显示器的亮度",
    hotkeys_intro: "按下面的快捷键，调节所有显示器的亮度：",
    increase_brightness: "提高亮度",
    decrease_brightness: "降低亮度",
    monitors: "显示器",
    monitors_hint: "可以给显示器重命名；留空则使用默认名称。",
    failed_to_detect_monitors: "未能检测到显示器。",
    advanced: "高级",
    autostart_priority: "使用更高的开机启动优先级（需要管理员权限）",
    failed_enable_icon_scroll: "启用滚轮调节失败",
    failed_register_hotkey: "注册快捷键失败",
    internal_display: "内置屏幕"
};

static LANGUAGE: OnceLock<Language> = OnceLock::new();

/// Resolves the language. `setting` is the raw value of the config key,
/// `auto` (or anything unknown) means "follow the system language".
pub fn init(setting: &str) {
    let language = match setting.trim().to_lowercase().as_str() {
        "en" | "en-us" | "english" => Language::English,
        "zh" | "zh-cn" | "zh-hans" | "chinese" => Language::Chinese,
        _ => detect_system_language()
    };
    let _ = LANGUAGE.set(language);
}

pub fn current() -> Language {
    *LANGUAGE.get_or_init(detect_system_language)
}

pub fn strings() -> &'static Strings {
    match current() {
        Language::English => &ENGLISH,
        Language::Chinese => &CHINESE
    }
}

fn detect_system_language() -> Language {
    let lang = unsafe { GetUserDefaultUILanguage() };
    // PRIMARYLANGID(lang) == LANG_CHINESE (0x04)
    if lang & 0x3ff == 0x04 {
        Language::Chinese
    } else {
        Language::English
    }
}
