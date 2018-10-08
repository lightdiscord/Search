#[macro_use] extern crate yew;
#[macro_use] extern crate stdweb;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rand;

pub mod utils;
pub mod components;
pub mod engines;
pub mod state;

use yew::prelude::*;
use yew::services::{ ConsoleService, StorageService };

use rand::thread_rng;
use rand::seq::SliceRandom;

use self::components::{ EngineButton, SearchInput };
use self::engines::Engine;
use self::state::State;

pub struct Model {
    console: ConsoleService,
    engines: Vec<Engine>,
    current: Option<Engine>,
    storage: StorageService,
    state: State
}

pub enum Message {
    Search(String),
    ChooseEvent(Engine),
    DefaultEvent(Engine),
    StoreState
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut engines = engines::Engine::to_vec();
        engines.shuffle(&mut thread_rng());

        let (storage, state) = State::retrieve();

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
            Message::Search(search) => if let Some(engine) = &self.current {
                self.console.info("New search!");
                self.console.debug(&search);

                let url = engine.metas().schema.replace("%s", &search);

                js! {
                    window.open(@{url}, "_self");
                }
            },
            Message::ChooseEvent(engine) => {
                let engine = Some(engine);
                if self.current == engine {
                    return false;
                }

                self.console.info("New engine");
                self.console.debug(&engine.as_ref().unwrap().metas().name);

                self.current = engine;
            },
            Message::DefaultEvent(engine) => {
                let engine = Some(engine);

                if self.state.default_engine == engine {
                    self.console.info("Remove default engine.");

                    self.state.default_engine = None;
                } else {
                    self.console.info("New default engine");
                    self.console.debug(&engine.as_ref().unwrap().metas().name);

                    self.state.default_engine = engine;
                }

                self.update(Message::StoreState);
            },
            Message::StoreState => {
                self.state.save(&mut self.storage);

                return false;
            }
        }

        true
    }
}

struct SearchBar;

impl Renderable<Model> for SearchBar {
    fn view(&self) -> Html<Model> {
        html! {
            <div class="field",>
                <label class="label",>{ "Search" }</label>
                <div class="control",>
                    <SearchInput: onsearch=|search| Message::Search(search), />
                </div>
                <p class="help",>{ "Press enter to search" }</p>
            </div>
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let format_engine = |engine: &Engine| {
            let active = self.current.as_ref()
                .map(|current| current == engine)
                .unwrap_or(false);

            html! {
                <div class="control",>
                    <EngineButton: engine=engine,
                        active=active,
                        onclick=|engine| Message::ChooseEvent(engine),
                        ondoubleclick=|engine| Message::DefaultEvent(engine), />
                </div>
            }
        };

        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title is-3",>{ "Search" }</h1>

                    { SearchBar.view() }

                    <div class="field is-grouped is-grouped-multiline",>
                        { for self.engines.iter().map(format_engine) }
                    </div>

                    <h2 class="title is-4",>{ "Introduction" }</h2>
                    <p>
                        { "A random list is generated from a list of known search \
                          engines. You can double click on one of them to make it your \
                          default engine." }
                    </p>
                    <p>
                        { "This application was developped by " }
                        <a href="https://arnaud.sh",>{ "LightDiscord" }</a>
                        { " and it's also on " }
                        <a href="https://github.com/lightdiscord/search",>{ "GitHub" }</a>
                        { "." }
                    </p>
                </div>
            </section>
        }
    }
}
