use yew::prelude::*;

pub struct Model {
    onsearch: Option<Callback<String>>,
    text: String,
}

pub enum Msg {
    Search,
    GotInput(String),
    Clear,
    Nope,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onsearch: Option<Callback<String>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            onsearch: None,
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            onsearch: props.onsearch,
            text: "".into()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search => {
                if let Some(ref mut callback) = self.onsearch {
                    callback.emit(self.text.clone());
                }
                false
            },
            Msg::Clear => {
                self.update(Msg::GotInput("".into()))
            },
            Msg::GotInput(value) => {
                self.text = value;
                true
            },
            Msg::Nope => false,

        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onsearch = props.onsearch;
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <input class="input is-medium",
                type="text",
                value=&self.text,
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Search } else { Msg::Nope }
                },
                oninput=|e| Msg::GotInput(e.value),>
            </input>
        }
    }
}
