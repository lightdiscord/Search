#[macro_use]
extern crate yew;
extern crate stdweb;

use stdweb::web::Date;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg);
                self.console.log("Bulk action");
            },
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title",>{ "Counter" }</h1>

                    <div class="buttons",>
                        <button class="button", onclick=|_| Msg::Increment,>{ "Increment" }</button>
                        <button class="button", onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                        <button class="button", onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                    </div>

                    <p>{ self.value }</p>
                    <p>{ Date::new().to_string() }</p>
                </div>
            </section>
        }
    }
}
