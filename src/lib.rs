#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;

pub mod components;

use yew::prelude::*;
use yew::services::ConsoleService;

use self::components::input::Model as SearchInput;

pub struct Model {
    console: ConsoleService,
}

pub enum Msg {
    Search(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search) => {
                self.console.log(&format!("New search! {}", search));

                let url = format!("https://google.com/search?q={}", search);

                js! {
                    window.open(@{url}, "_self");
                }
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title",>{ "Search" }</h1>

                    <div>
                        <SearchInput: onsearch=|search| Msg::Search(search), />
                    </div>
                </div>
            </section>
        }
    }
}
