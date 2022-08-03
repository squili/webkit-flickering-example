use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use glib::Continue;
use gtk::{
    traits::{ContainerExt, GtkWindowExt, WidgetExt},
    Inhibit, Window, WindowType,
};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

static FLIP: AtomicBool = AtomicBool::new(false);

fn main() -> anyhow::Result<()> {
    gtk::init()?;

    let window = Window::new(WindowType::Toplevel);
    let context = WebContext::new();
    let webview = WebView::with_context(&context);
    webview.load_uri("https://fig.io");
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    let window_ = window.clone();
    webview.connect_show(move |_| {
        let window = window_.clone();
        glib::source::timeout_add_local(Duration::from_millis(300), move || {
            if FLIP.load(Ordering::SeqCst) {
                window.resize(400, 400);
                FLIP.store(false, Ordering::SeqCst)
            } else {
                window.resize(400, 300);
                FLIP.store(true, Ordering::SeqCst)
            }
            Continue(true)
        });
    });

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();

    Ok(())
}
