use yew::prelude::*;
use super::super::engines::Engine;

pub struct Model {
    engine: Engine,
    onchoose: Option<Callback<Engine>>,
    active: bool,
}

pub enum Msg {
    Choose,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onchoose: Option<Callback<Engine>>,
    pub engine: Engine,
    pub active: bool,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            onchoose: None,
            engine: Engine::default(),
            active: false,
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            onchoose: props.onchoose,
            engine: props.engine,
            active: props.active
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Choose => {
                if let Some(ref mut callback) = self.onchoose {
                    callback.emit(self.engine.clone());
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onchoose = props.onchoose;
        self.active = props.active;
        self.engine = props.engine;
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let mut classes = vec!["button"];

        if self.active {
            classes.push("is-active");
        }

        html! {
            <button
                class=classes.join(" "),
                onclick=|_| Msg::Choose,>
                { &self.engine.name }
            </button>
        }
    }
}
