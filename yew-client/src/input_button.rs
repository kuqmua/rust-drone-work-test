use yew::prelude::*;

pub enum Msg {
    AddOne,
}
pub struct InputButton {
    value: i64,
}

impl Component for InputButton {
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
          <button
            class="MuiButtonBase-root MuiButton-root MuiButton-contained makeStyles-submit-4 MuiButton-containedPrimary MuiButton-fullWidth"
            tabindex="0"
            type="submit"
            style="
              margin: 24px 0px 16px;
              width: 100%;
              color: #fff;
              background-color: #556cd6;
              box-shadow: 0px 3px 1px -2px rgb(0 0 0 / 20%), 0px 2px 2px 0px rgb(0 0 0 / 14%), 0px 1px 5px 0px rgb(0 0 0 / 12%);
              padding: 6px 16px;
              font-size: 0.875rem;
              min-width: 64px;
              box-sizing: border-box;
              transition: background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,border 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
              font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
              font-weight: 500;
              line-height: 1.75;
              border-radius: 4px;
              letter-spacing: 0.02857em;
              text-transform: uppercase;
              color: inherit;
              border: 0;
              cursor: pointer;
              margin: 0;
              display: inline-flex;
              outline: 0;
              padding: 0;
              position: relative;
              align-items: center;
              user-select: none;
              border-radius: 0;
              vertical-align: middle;
              -moz-appearance: none;
              justify-content: center;
              text-decoration: none;
              background-color: transparent;
              -webkit-appearance: none;
              -webkit-tap-highlight-color: transparent;
              -webkit-writing-mode: horizontal-tb !important;
              font-style: ;
              font-variant-ligatures: ;
              font-variant-caps: ;
              font-variant-numeric: ;
              font-variant-east-asian:
              text-rendering: auto;
              word-spacing: normal;
              text-indent: 0px;
              text-shadow: none;
              text-align: center;
            "
          >
            <span
              class="MuiButton-label"
              style="
                width: 100%;
                display: inherit;
                align-items: inherit;
                justify-content: inherit;
                box-sizing: inherit;
                color: #fff;
                background-color: #556cd6;
                padding: 6px 16px;
                font-size: 0.875rem;
                min-width: 64px;
                box-sizing: border-box;
                transition: background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,border 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 500;
                line-height: 1.75;
                border-radius: 4px;
                letter-spacing: 0.02857em;
                text-transform: uppercase;
                border: 0;
                cursor: pointer;
                margin: 0;
                display: inline-flex;
                outline: 0;
                padding: 0;
                position: relative;
                align-items: center;
                user-select: none;
                border-radius: 0;
                vertical-align: middle;
                -moz-appearance: none;
                justify-content: center;
                text-decoration: none;
                background-color: transparent;
                -webkit-appearance: none;
                -webkit-tap-highlight-color: transparent;
                -webkit-writing-mode: horizontal-tb !important;
                font-style: ;
                font-variant-ligatures: ;
                font-variant-caps: ;
                font-variant-numeric: ;
                font-variant-east-asian: ;
                font-stretch: ;
                text-rendering: auto;
                word-spacing: normal;
                text-indent: 0px;
                text-shadow: none;
                text-align: center;
              "
            >
              {"Sign Up"}
            </span>
            <span
              class="MuiTouchRipple-root"
              style="
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                z-index: 0;
                overflow: hidden;
                position: absolute;
                border-radius: inherit;
                pointer-events: none;
                box-sizing: inherit;
                color: #fff;
                background-color: #556cd6;
                box-shadow: 0px 3px 1px -2px rgb(0 0 0 / 20%), 0px 2px 2px 0px rgb(0 0 0 / 14%), 0px 1px 5px 0px rgb(0 0 0 / 12%);
                background-color: #e0e0e0;
                color: rgba(0, 0, 0, 0.87);
                padding: 6px 16px;
                font-size: 0.875rem;
                min-width: 64px;
                box-sizing: border-box;
                transition: background-color 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,box-shadow 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms,border 250ms cubic-bezier(0.4, 0, 0.2, 1) 0ms;
                font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                font-weight: 500;
                line-height: 1.75;
                border-radius: 4px;
                letter-spacing: 0.02857em;
                text-transform: uppercase;
                border: 0;
                cursor: pointer;
                margin: 0;
                display: inline-flex;
                outline: 0;
                padding: 0;
                position: relative;
                align-items: center;
                user-select: none;
                border-radius: 0;
                vertical-align: middle;
                -moz-appearance: none;
                justify-content: center;
                text-decoration: none;
                background-color: transparent;
                -webkit-appearance: none;
                -webkit-tap-highlight-color: transparent;
                -webkit-writing-mode: horizontal-tb !important;
                font-style: ;
                font-variant-ligatures: ;
                font-variant-caps: ;
                font-variant-numeric: ;
                font-variant-east-asian: ;
                font-stretch: ;
                text-rendering: auto;
                word-spacing: normal;
                text-indent: 0px;
                text-shadow: none;
                text-align: center;
              "
            >
            </span>
          </button>
        }
    }
}
