fn main() {
    #[cfg(windows)]
    embed_windows_icon();
}

/// Embed the app icon as a Win32 resource so the executable shows it in
/// Explorer, the taskbar, and Alt-Tab. Regenerate `assets/AppIcon.ico` from
/// `assets/icons/logo.svg` when the logo changes.
#[cfg(windows)]
fn embed_windows_icon() {
    const ICON: &str = "assets/AppIcon.ico";
    println!("cargo:rerun-if-changed={ICON}");
    println!("cargo:rerun-if-changed=build.rs");

    let mut resource = winresource::WindowsResource::new();
    resource.set_icon(ICON);
    if let Err(error) = resource.compile() {
        println!("cargo:warning=failed to embed the app icon: {error}");
    }
}
