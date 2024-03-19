use gtk::prelude::*;

use glib::ObjectType;
use std::ffi::c_void;

fn main() {
    gtk::init().expect("Failed to initialize gtk");

    // enter display name manually
    let display: glib::GString = "1\\WinSta0\\Default".into();

    let display = gdk::Display::open(&display).expect("display open error");
    println!("dis? {:?}", display.name());

    // enter handle value manually
    let window_reload = cast_native_handle_window(display, 4328684);
    println!("wd?? {:?}", window_reload);

    let pix = window_reload
        .pixbuf(0, 0, window_reload.width(), window_reload.height())
        .unwrap();

    let file_path = "captured_image.png";
    let format = "png";
    let options = vec![];
    pix.savev(file_path, format, &options)
        .expect("Failed to save GdkPixbuf to file");

    loop {}
}

pub fn cast_native_handle_window(display: gdk::Display, handle: usize) -> gtk::gdk::Window {
    #[cfg(target_os = "windows")]
    {
        extern "C" {
            pub fn gdk_win32_window_lookup_for_display(
                display: *mut glib::object::GObject,
                handle: usize,
            ) -> *mut c_void;
        }

        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let wd =
                gdk_win32_window_lookup_for_display(display.as_ptr() as *mut _, handle as usize)
                    as *mut gtk::gdk::Window;

            let wd = std::mem::transmute::<*mut gtk::gdk::Window, gtk::gdk::Window>(wd);

            wd
        }
    }
}
