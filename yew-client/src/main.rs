pub mod form;
pub mod input_button;

use crate::form::Form;
use crate::input_button::InputButton;

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
    //https://codepen.io/shawnc8160/pen/xxRYOWg
    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div id="root"
            style="
            display: block;
            color: rgba(0, 0, 0, 0.87);
            margin: 0;
            font-size: 0.875rem;
            font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
            font-weight: 400;
            line-height: 1.43;
            letter-spacing: 0.01071em;
            background-color: #fff;
            -webkit-font-smoothing: antialiased;
            box-sizing: inherit;
            "
            >
            <main
            class="MuiContainer-root MuiContainer-maxWidthXs"
            style="
            max-width: 444px;
            width: 100%;
            display: block;
            box-sizing: border-box;
            margin-left: auto;
            margin-right: auto;
            padding-left: 16px;
            padding-right: 16px;
            "
            >
            <div class="makeStyles-paper-1"
            style="
            display: flex;
            margin-top: 64px;
            align-items: center;
            flex-direction: column;
            "
            >
            <div
            class="MuiAvatar-root MuiAvatar-circular makeStyles-avatar-2 MuiAvatar-colorDefault"
            style="
            margin: 8px;
            background-color: #19857b;
            color: #fff;
            width: 40px;
            height: 40px;
            display: flex;
            overflow: hidden;
            position: relative;
            font-size: 1.25rem;
            align-items: center;
            flex-shrink: 0;
            font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
            line-height: 1;
            user-select: none;
            border-radius: 50%;
            justify-content: center;
            "
            >
            <svg
            style="
                width: 75%;
                height: 75%;
                fill: currentColor;
                width: 1em;
                height: 1em;
                display: inline-block;
                font-size: 1.5rem;
                transition: fill 200ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
                flex-shrink: 0;
                user-select: none;
            "
            class="MuiSvgIcon-root MuiAvatar-fallback" focusable="false" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path>
            </svg>
            </div>
            <h1
            class="MuiTypography-root MuiTypography-h5"
            style="
            font-size: 1.5rem;
            font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
            font-weight: 400;
            line-height: 1.334;
            letter-spacing: 0em;
            margin: 0;
            box-sizing: inherit;
            display: block;
            font-size: 2em;
            margin-block-start: 0.67em;
            margin-block-end: 0.67em;
            margin-inline-start: 0px;
            margin-inline-end: 0px;
            font-weight: bold;
            "
            >{"Sign up"}</h1>
            <form class="makeStyles-form-3" novalidate=true
            style="
                width: 100%;
                margin-top: 24px;
                box-sizing: inherit;
                display: block;
            "
            >
            <div
            class="MuiGrid-root MuiGrid-container MuiGrid-spacing-xs-2"
            style="
            width: calc(100% + 16px);
            margin: -8px;
            display: flex;
            flex-wrap: wrap;
            box-sizing: border-box;
            "
            >
            <Form />
            <Form />
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12">
            </div>
            </div>
            <InputButton />
            <div class="MuiGrid-root MuiGrid-container MuiGrid-justify-content-xs-flex-end"
            style="
            justify-content: flex-end;
            width: 100%;
            display: flex;
            flex-wrap: wrap;
            box-sizing: border-box;
            "
            >
            <div class="MuiGrid-root MuiGrid-item"
            style="
            margin: 0;
            box-sizing: border-box;
            display: block;
            "
            >
            <a class="MuiTypography-root MuiLink-root MuiLink-underlineHover MuiTypography-body2 MuiTypography-colorPrimary" href="#"
            style="
            text-decoration: none;
            color: #556cd6;
            font-size: 0.875rem;
            font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
            font-weight: 400;
            line-height: 1.43;
            letter-spacing: 0.01071em;
            margin: 0;
            box-sizing: inherit;
            cursor: pointer;
            "
            >{"Already have an account? Sign in"}</a>
            </div>
            </div>
            </form>
            </div>
            </main>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
