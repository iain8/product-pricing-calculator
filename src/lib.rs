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

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["md:grid md:grid-cols-3 md:gap-6"],
            left_column(),
            right_column(model)
        ]
    ]
}

fn left_column() -> Node<Msg> {
    div![
        C!["md:col-span-1"],
        div![
            C!["px-4 sm:px-0"],
            h3![
                C!["text-lg font-medium leading-6 text-gray-900"],
                "Wholesale pricing calculator"
            ]
        ]
    ]
}

fn right_column(model: &Model) -> Node<Msg> {
    div![
        C!["mt-5 md:mt-0 md:col-span-2"],
        form![
            div![
                C!["shadow sm:rounded-md sm:overflow-hidden"],
                div![
                    C!["px-4 py-5 bg-white space-y-6 sm:p-6"],
                    input_field(
                        "pin_quantity".to_string(), 
                        "Quantity", 
                        "Number of pins ordered", 
                        attrs!{At::Name => "pin-quantity", At::Placeholder => "Pin quantity", At::Value => model.pin_quantity}
                    ),
                    input_field(
                        "pin_cost".to_string(), 
                        "Total cost", 
                        "Total cost of order", 
                        attrs!{At::Name => "pin-total-cost", At::Placeholder => "Pin total cost", At::Value => model.pin_cost}
                    ),
                    input_field(
                        "hours_worked".to_string(), 
                        "Hours worked", 
                        "Hours worked on item", 
                        attrs!{At::Name => "hours-worked", At::Placeholder => "Hours worked", At::Value => model.hours_worked}
                    ),
                    input_field(
                        "hourly_wage".to_string(), 
                        "Hourly wage", 
                        "Cost of one hours work", 
                        attrs!{At::Name => "hourly-wage", At::Placeholder => "Hourly wage", At::Value => model.hourly_wage}
                    ),
                    input_field(
                        "overhead_percentage".to_string(), 
                        "Overhead percentage", 
                        "Overheads as percentage of cost", 
                        attrs!{At::Name => "overhead-percentage", At::Placeholder => "Overhead percentage", At::Value => model.overhead_percentage},
                    ),
                    input_field(
                        "pin_unit_cost".to_string(), 
                        "Unit cost", 
                        "Cost of one pin", 
                        attrs!{At::Name => "pin-unit-cost", At::Placeholder => "Pin unit cost", At::Value => model.pin_unit_cost},
                    ),
                    div![
                        "Wholesale price per unit: $",
                        model.wholesale_unit_price.to_string()
                    ]
                ]
            ],
        ],
        button![
            C!["ml-5 py-2 px-3 border rounded-md"],
            "Calculate",
            ev(Ev::Click, move |_| Msg::Calculate)
        ]
    ]
}

fn input_field(
    field: String,
    label: &str, 
    description: &str,
    attrs: seed::Attrs
) -> Node<Msg> {
    div![
        label![
            C!["block text-sm font-medium text-gray-700"],
            label
        ],
        div![
            C!["mt-1 rounded-md shadow-sm"],
            input![
                C!["shadow-sm focus:ring-indigo-500 focus:border-indigo-500 mt-1 block w-full sm:text-sm border-gray-300 rounded-md"],
                attrs,
                input_ev(Ev::Input, |value| Msg::Update(field, value))
            ]
        ],
        p![
            C!["mt-2 text-sm text-gray-500"],
            description
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
