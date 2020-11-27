// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
// #![allow(clippy::wildcard_imports)]

use rust_decimal::prelude::*;
use rusty_money::{Currency, Iso, Money};
use seed::{prelude::*, *};

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

enum Msg {
    Update(String, String),
    Calculate,
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
    main![
        div![
            C!["relative bg-gray-100"],
            header![
                C!["bg-white shadow"],
                div![
                    C!["max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8"],
                    h1![
                        C!["text-3xl font-bold leading-tight text-gray-900"],
                        "Wholesale pricing calculator"
                    ]
                ]
            ],
            div![
                C!["max-w-7xl mx-auto py-6 sm:px-6 lg:px-8"],
                div![
                    C!["md:grid md:grid-cols-4 md:gap-6"],
                    left_column(model),
                    right_column(model)
                ]
            ]
        ]
    ]
}

fn right_column(model: &Model) -> Node<Msg> {
    div![
        C!["md:col-span-2"],
        div![
            C!["px-4 sm:px-0"],
            table![
                C!["min-w-full divide-y divide-gray-200"],
                thead![
                    tr![
                        th![
                            C!["px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"],
                            attrs! {At::from("colspan") => 2},
                            "Results"
                        ]
                    ],
                ],
                tbody![
                    C!["bg-white divide-y divide-gray-200"],
                    tr![
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                            "Manufacture cost per unit"
                        ],
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm"],
                            format!("{}", model.pin_unit_cost)
                        ],
                    ],
                    tr![
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                            "Total labour cost"
                        ],
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm"],
                            format!("{}", model.total_labour_cost)
                        ],
                    ],
                    tr![
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                            "Wholesale price per unit"
                        ],
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm"],
                            format!("{}", model.wholesale_unit_price)
                        ],
                    ],
                    tr![
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm text-gray-500"],
                            "Suggested retail cost"
                        ],
                        td![
                            C!["px-6 py-4 whitespace-nowrap text-sm"],
                            format!("{}", model.suggested_retail_cost)
                        ],
                    ],
                ]
            ]
        ]
    ]
}

fn left_column(model: &Model) -> Node<Msg> {
    div![
        C!["mt-5 md:mt-0 md:col-span-2"],
        div![
            C!["shadow sm:rounded-md sm:overflow-hidden"],
            form![div![
                C!["px-4 py-5 bg-white space-y-6 sm:p-6"],
                div![
                    C!["grid grid-cols-2 gap-4"],
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
                ],
                div![
                    C!["grid grid-cols-2 gap-4"],
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
                ],
                div![
                    C!["grid grid-cols-2 gap-4"],
                    input_field(
                        "overhead_percentage".to_string(),
                        "Overhead percentage",
                        "Overhead as percentage of cost",
                        attrs! {At::Name => "overhead-percentage", At::Placeholder => "Overhead percentage", At::Value => model.overhead_percentage},
                    ),
                ],
                button![
                    C!["inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"],
                    "Calculate",
                    ev(Ev::Click, move |_| Msg::Calculate)
                ]
            ],
            ev(Ev::Submit, |event| event.prevent_default())
        ],],
    ]
}

fn input_field(field: String, label: &str, description: &str, attrs: seed::Attrs) -> Node<Msg> {
    div![
        label![
            C!["block text-sm font-medium text-gray-700"],
            label
        ],
        div![
            C!["mt-1 rounded-md shadow-sm"],
            input![
                C!["focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-2 border-gray-300 p-1"],
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
