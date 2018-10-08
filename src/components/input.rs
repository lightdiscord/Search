use yew::prelude::*;

pub struct Model {
    onsearch: Option<Callback<String>>,
    text: String,
}

pub enum Message {
    Search,
    GotInput(String),
    Clear,
    Keypress(KeyPressEvent),
}

impl Default for Model {
    fn default() -> Self {
        Self {
            onsearch: None,
            text: "".into()
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub onsearch: Option<Callback<String>>,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            onsearch: None,
        }
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            onsearch: props.onsearch,
            ..Default::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Search => {
                if let Some(ref mut callback) = self.onsearch {
                    callback.emit(self.text.clone());
                }
                false
            },
            Message::Clear => {
                self.update(Message::GotInput("".into()))
            },
            Message::GotInput(value) => {
                self.text = value;
                true
            },
            Message::Keypress(event) => {
                if event.key() == "Enter" {
                    self.update(Message::Search)
                } else {
                    false
                }
            }
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
                onkeypress=|e| Message::Keypress(e),
                oninput=|e| Message::GotInput(e.value),>
            </input>
        }
    }
}
