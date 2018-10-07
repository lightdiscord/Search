use yew::prelude::*;
use super::super::engines::Engine;

pub struct Model {
    engine: Engine,
    onchoose: Option<Callback<Engine>>,
    onfavorite: Option<Callback<Engine>>,
    active: bool,
}

pub enum Msg {
    Choose,
    Favorite,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub onchoose: Option<Callback<Engine>>,
    pub onfavorite: Option<Callback<Engine>>,
    pub engine: Engine,
    pub active: bool,
}

impl<'a> Default for Props {
    fn default() -> Self {
        Props {
            onchoose: None,
            onfavorite: None,
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
            onfavorite: props.onfavorite,
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
            },
            Msg::Favorite => {
                if let Some(ref mut callback) = self.onfavorite {
                    callback.emit(self.engine.clone());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.onchoose = props.onchoose;
        self.onfavorite = props.onfavorite;
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
                onclick=|_| Msg::Choose,
                ondoubleclick=|_| Msg::Favorite,>
                { &self.engine.name }
            </button>
        }
    }
}
