// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::ops::{Index, IndexMut};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        hourly_wage: 0,
        hours_worked: 0,
        overhead_percentage: 0,
        pin_cost: 0,
        pin_quantity: 0,
        pin_unit_cost: 0,
        wholesale_unit_price: 0.0
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    hourly_wage: i32,
    hours_worked: i32,
    overhead_percentage: i32,
    pin_cost: i32,
    pin_quantity: i32,
    pin_unit_cost: i32,
    wholesale_unit_price: f32
}

impl Index<&'_ str> for Model {
    type Output = i32;

    fn index(&self, key: &str) -> &Self::Output {
        match key {
            "hourly_wage" => &self.hourly_wage,
            "hours_worked" => &self.hours_worked,
            "overhead_percentage" => &self.overhead_percentage,
            "pin_cost" => &self.pin_cost,
            "pin_quantity" => &self.pin_quantity,
            "pin_unit_cost" => &self.pin_unit_cost,
            _ => panic!("unknown field: {}", key),
        }
    }
}

impl IndexMut<&'_ str> for Model {
    fn index_mut(&mut self, key: &str) -> &mut i32 {
        match key {
            "hourly_wage" => &mut self.hourly_wage,
            "hours_worked" => &mut self.hours_worked,
            "overhead_percentage" => &mut self.overhead_percentage,
            "pin_cost" => &mut self.pin_cost,
            "pin_quantity" => &mut self.pin_quantity,
            "pin_unit_cost" => &mut self.pin_unit_cost,
            _ => panic!("unknown field: {}", key),
        }
    }
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    Update(String, String),
    Calculate
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Update(field, value) => {
            model[&field] = value.parse::<i32>().unwrap_or(0);
        },
        Msg::Calculate => {
            let pin_quantity: i32 = if model.pin_quantity > 0 { model.pin_quantity } else { 1 };

            model.pin_unit_cost = model.pin_cost / pin_quantity;

            let total_labour_cost: i32 = model.hourly_wage * model.hours_worked;
            let subtotal: i32 = model.pin_cost + total_labour_cost;
            let total = (subtotal * 1 + (model.overhead_percentage / 100)) as f32;

            model.wholesale_unit_price = total / pin_quantity as f32;
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        h1!["Wholesale pricing calculator"],
        div![
            form![
                div![
                    label!["Quantity"],
                    input![
                        attrs!{At::Name => "pin-quantity", At::Placeholder => "Pin quantity", At::Value => model.pin_quantity},
                        input_ev(Ev::Input, |value| Msg::Update("pin_quantity".to_string(), value))
                    ],
                ],
                div![
                    label!["Total cost"],
                    input![
                        attrs!{At::Name => "pin-total-cost", At::Placeholder => "Pin total cost", At::Value => model.pin_cost},
                        input_ev(Ev::Input, |value| Msg::Update("pin_cost".to_string(), value))
                    ],
                ],
                div![
                    label!["Hours worked"],
                    input![
                        attrs!{At::Name => "hours-worked", At::Placeholder => "Hours worked", At::Value => model.hours_worked},
                        input_ev(Ev::Input, |value| Msg::Update("hours_worked".to_string(), value))
                    ],
                ],
                div![
                    label!["Hourly wage"],
                    input![
                        attrs!{At::Name => "hours-worked", At::Placeholder => "Hourly wage", At::Value => model.hourly_wage},
                        input_ev(Ev::Input, |value| Msg::Update("hourly_wage".to_string(), value))
                    ],
                ],
                div![
                    label!["Overhead percentage"],
                    input![
                        attrs!{At::Name => "hours-worked", At::Placeholder => "Overhead percentage", At::Value => model.overhead_percentage},
                        input_ev(Ev::Input, |value| Msg::Update("overhead_percentage".to_string(), value))
                    ],
                ],
                div![
                    label!["Unit cost"],
                    input![
                        attrs!{At::Name => "pin-unit-cost", At::Placeholder => "Pin unit cost", At::Value => model.pin_unit_cost}
                    ]
                ],
                div![
                    "Wholesale price per unit: $",
                    model.wholesale_unit_price.to_string()
                ]
            ]
        ],
        button![
            "Calculate",
            ev(Ev::Click, move |_| Msg::Calculate)
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
