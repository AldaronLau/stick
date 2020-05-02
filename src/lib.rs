#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/stick/res/controller.png",
    html_favicon_url = "https://libcala.github.io/stick/res/controller.png"
)]

mod event;
mod gamepad;
mod port;

#[cfg_attr(target_arch = "wasm32", path = "ffi/wasm32.rs")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    cfg_attr(target_os = "linux", path = "ffi/linux.rs"),
    cfg_attr(target_os = "android", path = "ffi/android.rs"),
    cfg_attr(target_os = "macos", path = "ffi/macos.rs"),
    cfg_attr(target_os = "ios", path = "ffi/ios.rs"),
    cfg_attr(target_os = "windows", path = "ffi/windows.rs"),
    cfg_attr(
        any(
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "bitrig",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        path = "ffi/bsd.rs"
    ),
    cfg_attr(target_os = "fuchsia", path = "ffi/fuchsia.rs"),
    cfg_attr(target_os = "redox", path = "ffi/redox.rs"),
    cfg_attr(target_os = "none", path = "ffi/none.rs"),
    cfg_attr(target_os = "dummy", path = "ffi/dummy.rs")
)]
mod ffi;

pub use event::Event;
pub use gamepad::Gamepad;
pub use port::Port;
