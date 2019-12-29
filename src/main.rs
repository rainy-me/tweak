use orbtk::prelude::*;

fn main() {
      Application::new()
        .window(|ctx| {
            Window::create()
                .title("Tweak")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::create().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}