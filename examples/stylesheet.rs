#[allow(unused)]
use vizia::prelude::*;

// const STYLE: &str = r#"
//     element {
//         width: 300px;
//         height: 100px;
//         background-color: green;
//         background-image: linear-gradient(to right, blue, green 20%, red 30%);
//     }
// "#;

// static test: &str = include_str!("resources/test.css");

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("This example is not supported on wasm - uses filesystem");
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("examples/resources/test.css").expect("Failed");
        // HStack::new(cx, |cx| {
        //     Element::new(cx)
        //         //.size(Pixels(100.0))
        //         //.space(Pixels(100.0))
        //         .class("foo");
        //     //.background_color("red")
        //     //.border_width("10px")
        //     //.border_radius("10px")
        //     //.border_top_left_radius("50%")
        //     //.border_top_right_radius("0")
        //     //.border_top_left_shape("bevel")
        //     //.border_color("rgba(0,255,0,0.5)")
        //     //.outline_width("2px")
        //     //.outline_offset("5px")
        //     //.outline_color("cyan")
        //     //.opacity(1.0f32);
        // })
        // .class("bar");
        HStack::new(cx, |cx| {
            Element::new(cx).size(Pixels(50.0)).background_color(Color::red()).space(Pixels(40.0));
            // .class("foo");
        })
        .size(Pixels(300.0))
        .space(Pixels(50.0))
        .background_color(Color::green());
    })
    .title("Stylesheet")
    .run();
}
