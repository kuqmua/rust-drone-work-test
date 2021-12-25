use yew::prelude::*;

enum Msg {
    AddOne,
}
struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div style="display: flex; justify-content: center; align-items: center; width: 100%; height: 100%">
              <div style="display: flex; justify-content: center; align-items: center; width: 50%; height: 50vh; background-color: red">
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                 <select>
                 <option value="0">{"Select car:"}</option>
                 <option value="1">{"Audi"}</option>
                 <option value="2">{"BMW"}</option>
                 <option value="3">{"Citroen"}</option>
                 <option value="4">{"Ford"}</option>
                 <option value="5">{"Honda"}</option>
                 <option value="6">{"Jaguar"}</option>
                 <option value="7">{"Land Rover"}</option>
                 <option value="8">{"Mercedes"}</option>
                 <option value="9">{"Mini"}</option>
                 <option value="10">{"Nissan"}</option>
                 <option value="11">{"Toyota"}</option>
                 <option value="12">{"Volvo"}</option>
                 </select>
              </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
