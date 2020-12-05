use seed::{prelude::*, *};

pub enum Msg {
    Update(String, String),
    Calculate,
}

pub fn header() -> Node<Msg> {
    header![h1!["Wholesale pricing calculator"]]
}

pub fn input_field(field: String, label: &str, description: &str, attrs: seed::Attrs) -> Node<Msg> {
    div![
        label![
            attrs! {At::For => field},
            label
        ],
        input![
            attrs,
            input_ev(Ev::Input, |value| Msg::Update(field, value))
        ],
        p![description]
    ]
}
