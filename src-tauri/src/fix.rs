//! Module for fixing other problems

/// Fix `WebKit` error for nvidia drivers
///
/// # Explanation
///
/// `Nvidia` has [broken](https://gitlab.gnome.org/GNOME/gnome-control-center/-/issues/2498) the driver
/// for `WebKit` and is not going to fix it.
///
/// Therefore, the best solution is to disable some features in `WebKit`.
/// The error only occurs on **Linux**!
///
/// ## If this function did not work
///
/// You can specify ```nvidia_drm.modeset=1``` in the kernel parameters
///
/// This worked for me personally
///
/// My personal message to Nvidia. [Link](https://youtu.be/JbovJbKALzA?si=LEMMk1Wp1fw8ggOH)
fn webkit() {
    #[cfg(target_family = "unix")]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
}

pub fn all() {
    webkit();
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_family = "unix")]
    fn webkit() {
        super::webkit();
    }
}
