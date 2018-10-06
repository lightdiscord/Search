#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;

pub mod components;
pub mod engines;

use yew::prelude::*;
use yew::services::ConsoleService;

use self::components::input::Model as SearchInput;
use self::components::engine::Model as EngineButton;

use self::engines::Engine;

pub struct Model {
    console: ConsoleService,
    current: Engine,
}

pub enum Msg {
    Search(String),
    NewEngine(Engine)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            current: engines::list::GOOGLE
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search) => {
                self.console.log(&format!("New search! {}", search));

                let url = self.current.schema.replace("%s", &search);

                js! {
                    window.open(@{url}, "_self");
                }
            },
            Msg::NewEngine(engine) => {
                if self.current == engine {
                    return false;
                }

                self.console.log(&format!("New engine! {}", engine.name));
                self.current = engine;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let show = |x| html! {
            <EngineButton: engine=x, active=x==&self.current, onchoose=|engine| Msg::NewEngine(engine), />
        };

        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title",>{ "Search" }</h1>

                    <div>
                        <SearchInput: onsearch=|search| Msg::Search(search), />
                    </div>

                    <div class="buttons",>
                        { for engines::list::ENGINES.iter().map(show) }
                    </div>
                </div>
            </section>
        }
    }
}
