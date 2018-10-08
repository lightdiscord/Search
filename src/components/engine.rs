use yew::prelude::*;

use super::super::{ engines, utils };
use self::engines::Engine;
use self::utils::Conditional;

pub struct Model {
    engine: Engine,
    onclick: Option<Callback<Engine>>,
    ondoubleclick: Option<Callback<Engine>>,
    active: bool,
}

pub enum Message {
    Click,
    DoubleClick,
}

#[derive(PartialEq, Clone)]
pub struct Properties {
    pub onclick: Option<Callback<Engine>>,
    pub ondoubleclick: Option<Callback<Engine>>,
    pub engine: Engine,
    pub active: bool,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            onclick: None,
            ondoubleclick: None,
            engine: Engine::default(),
            active: false,
        }
    }
}

impl Into<Model> for Properties {
    fn into(self) -> Model {
        Model {
            onclick: self.onclick,
            ondoubleclick: self.ondoubleclick,
            engine: self.engine,
            active: self.active
        }
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        props.into()
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        let action = match message {
            Message::Click => &mut self.onclick,
            Message::DoubleClick => &mut self.ondoubleclick,
        };

        if let Some(ref mut callback) = action {
            callback.emit(self.engine.clone());
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onclick = props.onclick;
        self.ondoubleclick = props.ondoubleclick;
        self.active = props.active;
        self.engine = props.engine;

        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let mut classes = vec!["button"];
        classes.conditional_push("is-active", self.active);
        let classes = classes.join(" ");

        html! {
            <button class=classes,
                onclick=|_| Message::Click,
                ondoubleclick=|_| Message::DoubleClick,>
                { self.engine.metas().name }
            </button>
        }
    }
}
