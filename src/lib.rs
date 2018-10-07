#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;

pub mod components;
pub mod engines;

use yew::prelude::*;
use yew::services::{ ConsoleService, StorageService };
use yew::services::storage::Area;
use yew::format::Json;

use rand::thread_rng;
use rand::seq::SliceRandom;

use self::components::input::Model as SearchInput;
use self::components::engine::Model as EngineButton;

use self::engines::Engine;

const KEY: &'static str = "search.state";

pub struct Model {
    console: ConsoleService,
    engines: Vec<Engine>,
    current: Option<Engine>,
    storage: StorageService,
    state: State
}

pub enum Msg {
    Search(String),
    NewEngine(Engine),
    NewDefault(Engine)
}

#[derive(Serialize, Deserialize)]
pub struct State {
    default_engine: Option<Engine>,
}

impl Default for State {
    fn default() -> Self {
        State {
            default_engine: None
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut engines = engines::list::Engines::to_vec();
        engines.shuffle(&mut thread_rng());

        let mut storage = StorageService::new(Area::Local);

        if let Json(Ok(state)) = storage.restore(KEY) {
            let state: State = state;
        }

        let Json(state) = storage.restore(KEY);
        let state: State = state.unwrap_or_else(|_| State::default());
        let current = state.default_engine.as_ref().or(engines.first())
            .map(ToOwned::to_owned);

        Model {
            console: ConsoleService::new(),
            current,
            engines,
            storage,
            state
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
            },
            Msg::NewDefault(engine) => {
                let engine = Some(engine);
                if self.state.default_engine == engine {
                    return false;
                }

                self.console.log(&format!("New default! {}", engine.as_ref().unwrap().name));
                self.state.default_engine = engine;
                self.storage.store(KEY, Json(&self.state));
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
                <EngineButton: engine=engine,
                    active=active,
                    onchoose=|engine| Msg::NewEngine(engine),
                    onfavorite=|engine| Msg::NewDefault(engine), />
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
