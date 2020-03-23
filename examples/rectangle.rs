use ldcore::{event_loop::*, graphics::*, input::*, window::*, Context};

fn main() {
    let mut cx = Context::new(
        EventSettings::new(),
        WindowSettings::new("ldcore rectangles", [640, 480]).exit_on_esc(true),
    )
    .unwrap();

    while let Some(event) = cx.next() {
        event.render(|args| {
            cx.draw(args.viewport(), |gcx, cx| {
                clear([1.0; 4], cx);
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // red
                    [0.0, 0.0, 100.0, 100.0],
                    gcx.transform,
                    cx,
                );
            });
        });
    }
}
