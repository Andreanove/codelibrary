use gpui::{
    div,
    prelude::*,
    px,
    rgb,
    size,
    App,
    Bounds,
    Context,
    Window,
    WindowBounds,
    TitlebarOptions,
    WindowOptions,
};

struct RootView;

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        println!("3. Fungsi render() dijalankan");
        div()
            .flex()
            .size_full()
            .bg(rgb(0x1f1f1f))
            .text_color(rgb(0xcccccc))
            .justify_center()
            .items_center()
            .child("CodeLibrary")
    }
}

fn main() {
    env_logger::init();
    gpui_platform::application().run(|cx: &mut App| {
        println!("1. Event loop GPUI aktif");
        let bounds = Bounds::centered(None, size(px(1024.0), px(680.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: None,
                titlebar: Some(TitlebarOptions {
                    title: Some("CodeLibrary".into()),
                    ..Default::default()
                }),
                focus: true,
                show: true,
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| RootView),
        )
        .unwrap();
        println!("2. Jendela selesai dibuat");

        cx.activate(true);
    });
}