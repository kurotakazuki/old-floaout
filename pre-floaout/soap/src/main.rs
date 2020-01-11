use orbtk::prelude::*;


fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Soap")
                .position((100.0, 100.0))
                .size(500.0, 300.0)
                .theme(
                    ThemeValue::create()
                        .extension_css(include_str!("res/soap.css"))
                        .build(),
                )
                .resizeable(true)
                .child(TextBlock::create().text("Soap").selector(SelectorValue::new().with("text")).build(ctx))
                .build(ctx)
        })
        .run();
}
