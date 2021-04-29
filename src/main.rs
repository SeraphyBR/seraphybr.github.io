// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};


// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// `Model` describes our app state.
struct Model {
    counter: i32,
}

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}


// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            "This is a counter: ",
            C!["counter"],
            button![model.counter, ev(Ev::Click, |_| Msg::Increment),C!["btn btn-primary"]],
        ],
        div![
            C!["btn btn-primary"],
            "Blue button top"
        ],
        div!["Teste"],
        md!["# Hello World Yoooo 56"]
    ]
}

pub fn main() {
    App::start("app", init, update, view);
}
