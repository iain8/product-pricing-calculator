#![allow(clippy::wildcard_imports)]

mod ui;

use rust_decimal::prelude::*;
use rusty_money::{Currency, Iso, Money};
use seed::{prelude::*, *};
use crate::ui::{header, input_field, Msg};

struct Model {
    hourly_wage: Money,
    hours_worked: i32,
    overhead_percentage: Decimal,
    pin_cost: Money,
    pin_quantity: i32,
    pin_unit_cost: Money,
    suggested_retail_cost: Money,
    total_labour_cost: Money,
    wholesale_unit_price: Money,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        hourly_wage: Money::new(0, Currency::get(Iso::CAD)),
        hours_worked: 0,
        overhead_percentage: Decimal::new(15, 0),
        pin_cost: Money::new(0, Currency::get(Iso::CAD)),
        pin_quantity: 0,
        pin_unit_cost: Money::new(0, Currency::get(Iso::CAD)),
        suggested_retail_cost: Money::new(0, Currency::get(Iso::CAD)),
        total_labour_cost: Money::new(0, Currency::get(Iso::CAD)),
        wholesale_unit_price: Money::new(0, Currency::get(Iso::CAD)),
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Update(field, value) => {
            match field.as_ref() {
                "hourly_wage" => match Money::from_str(&value[1..], "CAD") {
                    Ok(result) => model.hourly_wage = result,
                    Err(e) => log!("Bad hourly wage value {} ({})", value, e),
                },
                "hours_worked" => model.hours_worked = value.parse::<i32>().unwrap_or(0),
                "overhead_percentage" => {
                    model.overhead_percentage = Decimal::from_str(&value).unwrap()
                }
                "pin_cost" => match Money::from_str(&value[1..], "CAD") {
                    Ok(result) => model.pin_cost = result,
                    Err(e) => log!("Bad pin cost value {} ({})", value, e),
                },
                "pin_quantity" => model.pin_quantity = value.parse::<i32>().unwrap_or(0),
                _ => println!("TODO: error handler"),
            };
        }
        Msg::Calculate => {
            let pin_quantity: i32 = if model.pin_quantity > 0 {
                model.pin_quantity
            } else {
                1
            };

            model.pin_unit_cost = model.pin_cost.clone() / pin_quantity;

            model.total_labour_cost = model.hourly_wage.clone() * model.hours_worked;
            let subtotal: Money = model.pin_cost.clone() + model.total_labour_cost.clone();
            let percentage =
                Decimal::new(1, 1) + (model.overhead_percentage / Decimal::new(100, 1));
            let total = subtotal * percentage;

            model.wholesale_unit_price = total / pin_quantity;
            model.suggested_retail_cost = model.wholesale_unit_price.clone() * 2;
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    main![div![
        C!["container"],
        header(),
        div![
            C!["row"],
            left_column(model),
            right_column(model)
        ]
    ]]
}

fn right_column(model: &Model) -> Node<Msg> {
    div![
        C!["column"],
        table![
            thead![
                tr![
                    th![
                        attrs! {At::from("colspan") => 2},
                        "Results"
                    ]
                ],
            ],
            tbody![
                tr![
                    td!["Manufacture cost per unit"],
                    td![format!("{}", model.pin_unit_cost)],
                ],
                tr![
                    td!["Total labour cost"],
                    td![format!("{}", model.total_labour_cost)],
                ],
                tr![
                    td!["Wholesale price per unit"],
                    td![format!("{}", model.wholesale_unit_price)],
                ],
                tr![
                    td!["Suggested retail cost"],
                    td![format!("{}", model.suggested_retail_cost)],
                ],
            ]
        ]
    ]
}

fn left_column(model: &Model) -> Node<Msg> {
    div![
        C!["column"],
        form![fieldset![
            input_field(
                "pin_quantity".to_string(),
                "Quantity",
                "Number of items ordered",
                attrs! {At::Name => "pin-quantity", At::Placeholder => "Pin quantity", At::Value => model.pin_quantity}
            ),
            input_field(
                "pin_cost".to_string(),
                "Total cost",
                "Total cost of order",
                attrs! {At::Name => "pin-total-cost", At::Placeholder => "Pin total cost", At::Value => model.pin_cost}
            ),
            input_field(
                "hours_worked".to_string(),
                "Hours worked",
                "Hours worked on item",
                attrs! {At::Name => "hours-worked", At::Placeholder => "Hours worked", At::Value => model.hours_worked}
            ),
            input_field(
                "hourly_wage".to_string(),
                "Hourly wage",
                "Cost of one hours work",
                attrs! {At::Name => "hourly-wage", At::Placeholder => "Hourly wage", At::Value => model.hourly_wage}
            ),
            input_field(
                "overhead_percentage".to_string(),
                "Overhead percentage",
                "Overhead as percentage of cost",
                attrs! {At::Name => "overhead-percentage", At::Placeholder => "Overhead percentage", At::Value => model.overhead_percentage},
            ),
            button![
                C!["button"],
                "Calculate",
                ev(Ev::Click, move |_| Msg::Calculate)
            ]
        ],
        ev(Ev::Submit, |event| event.prevent_default())
    ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
