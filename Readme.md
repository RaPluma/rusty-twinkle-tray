# Rusty Twinkle Tray

A small utility for quickly adjusting the brightness of external monitors using the DDC/CI protocol.
The built in laptop panel is supported as well (see [Internal displays](#internal-displays)).

Rusty Twinkle Tray is a work-in-progress rewrite of Twinkle Tray in Rust. A central goal of this rewrite is to start much faster so that monitor brightness can be adjusted as soon as possible after logging in.

## Screenshot
![image](https://github.com/user-attachments/assets/7f4c05d9-865d-4cdf-ae66-8f649b262912)

## Features
- Can automatically restore the last set brightness after¹:
  - Changing display settings
  - Waking up from sleep
- Can also control the brightness of the built in laptop panel
- Only one instance runs at a time (a second launch exits immediately)
- Small (~900kb) standalone executable
- Built using native OS controls instead of electron
- As inactive as possible when not in use
- Minimal dependencies

¹*Many monitors tend to "forget" settings set over DDC/CI after temporarily losing power*

## Internal displays
Laptop panels don't speak DDC/CI: Windows neither reports a friendly name for them nor
does `GetPhysicalMonitorsFromHMONITOR` give a usable physical monitor handle. They are
therefore controlled through the `\\.\LCD` device with the
`IOCTL_VIDEO_QUERY_SUPPORTED_BRIGHTNESS`, `IOCTL_VIDEO_QUERY_DISPLAY_BRIGHTNESS` and
`IOCTL_VIDEO_SET_DISPLAY_BRIGHTNESS` ioctls (the same mechanism the Windows mobility
center uses), which reports brightness on the usual 0 - 100 scale.

A monitor is treated as an internal display when it has no friendly name *and* the
internal brightness interface is available, so machines without such a panel are
unaffected. If the display driver doesn't expose the interface (some hybrid GPU setups
hide it), the panel simply keeps behaving like before.

Internal panels are listed with the model reported by the display itself, because Windows
only knows the generic "Integrated Monitor" for them: the monitor name from the EDID when
it provides one, otherwise the EDID manufacturer + product code (e.g. `BOE0B40`). You can
still rename it in the settings window.

## Language
The UI is available in English and Chinese. The language follows the system UI language
by default; it can be forced with the `Language` key in the config file
(`%APPDATA%\rusty-twinkle-tray.ini`):

```ini
[General]
Language=zh-CN   ; auto (default), en-US or zh-CN
```

## Precompiled Binaries

| [**DOWNLOAD**](https://github.com/RaPluma/rusty-twinkle-tray/releases/latest) |
|-----------------------------------------------------------------------------------------------------------------|

> This fork ships its own builds: [latest release](https://github.com/RaPluma/rusty-twinkle-tray/releases/latest)
> (adds the built in laptop panel support and an English/Chinese UI).
> The upstream project lives at [sidit77/rusty-twinkle-tray](https://github.com/sidit77/rusty-twinkle-tray).

## Building
This program is written in [Rust](https://www.rust-lang.org/) and requires a working Rust installation if you want to compile it yourself.

This program can be built by simply running the following:
```shell
cargo build --release
```

## Contributing

### Toolkit
This program uses the default [UWP Controls](https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.controls?view=winrt-22621) hosted inside an XAML island.

Unfortunately, the Microsoft-provided [`windows-rs` crate](https://microsoft.github.io/windows-docs-rs/doc/windows/) does not include bindings for the `Window.UI.Xaml` namespace. To nevertheless use it with Rust, this project directly uses [`windows-bindgen`](https://crates.io/crates/windows-bindgen) to manually generate the missing bindings (`libs/windows-ext`). However, the generated bindings are quite large (~1.5m LOC; ~1m of that in a single file). To keep the IDE happy, this project additionally uses a post-processor (`/libs/codegen`) that deletes many unused functions. The file `libs/windows-ext/Codegen.toml` controls which functions are kept. To regenerate the bindings after editing this file, run `just update-bindings` (requires [just](https://github.com/casey/just)).

### Todos
- [ ] Improve the mess that is the current UI code
  - Implement a reactive system using something like [reactive_graph](https://lib.rs/crates/reactive_graph)?
- [x] Settings window
  - [x] Toggle autostart
  - [x] Toggle brightness restore
  - [ ] Rename monitors
  - [ ] Reorder monitors
  - [ ] Change various delays/timeouts
- [x] Hotkeys
  - [ ] Allow for the customization of hotkeys
  - [ ] Refresh the monitor brightness when the hotkey is pressed
- [ ] Fluent design for Windows 11+
- [x] Respect Dark/Light system setting
- [x] Attempt to restore brightness after waking up from sleep
- [x] Support integrated laptop screens connected over I2C
- [x] Support monitor hot plugging
- [ ] Handle auto-hiding taskbar
- [ ] Improve themes
- [x] Adapt icon to system theme
- [x] Correctly handle display scaling


## License

MIT License
