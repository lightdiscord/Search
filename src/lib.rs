#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;
extern crate rand;

pub mod components;
pub mod engines;

use yew::prelude::*;
use yew::services::ConsoleService;

use rand::thread_rng;
use rand::seq::SliceRandom;

use self::components::input::Model as SearchInput;
use self::components::engine::Model as EngineButton;

use self::engines::Engine;

pub struct Model {
    console: ConsoleService,
    engines: Vec<Engine>,
    current: Option<Engine>,
}

pub enum Msg {
    Search(String),
    NewEngine(Engine)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut engines = engines::list::ENGINES.to_vec();
        engines.shuffle(&mut thread_rng());

        Model {
            console: ConsoleService::new(),
            current: engines.first().map(ToOwned::to_owned),
            engines
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search) => if let Some(engine) = &self.current {
                self.console.log(&format!("New search! {}", search));

                let url = engine.schema.replace("%s", &search);

                js! {
                    window.open(@{url}, "_self");
                }
            },
            Msg::NewEngine(engine) => {
                let engine = Some(engine);
                if self.current == engine {
                    return false;
                }

                self.console.log(&format!("New engine! {}", engine.as_ref().unwrap().name));
                self.current = engine;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let format_engine = |engine| {
            let active = self.current.as_ref()
                .map(|current| current == engine)
                .unwrap_or(false);

            html! {
                <EngineButton: engine=engine, active=active, onchoose=|engine| Msg::NewEngine(engine), />
            }
        };

        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title",>{ "Search" }</h1>

                    <div>
                        <SearchInput: onsearch=|search| Msg::Search(search), />
                    </div>

                    <div class="buttons",>
                        { for self.engines.iter().map(format_engine) }
                    </div>
                </div>
            </section>
        }
    }
}
