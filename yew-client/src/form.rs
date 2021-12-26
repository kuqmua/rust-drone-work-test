use yew::prelude::*;

pub enum Msg {
    AddOne,
}
pub struct Form {
    value: i64,
}

impl Component for Form {
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
          <div
            class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12 MuiGrid-grid-sm-6"
            style="
              padding: 8px;
              flex-grow: 0;
              max-width: 100%;
              flex-basis: 100%;
              margin: 0;
              box-sizing: border-box;
              display: block;
            "
          >
            <div
              class="MuiFormControl-root MuiTextField-root MuiFormControl-fullWidth"
              style="
                width: 100%;
                border: 0;
                margin: 0;
                display: inline-flex;
                padding: 0;
                position: relative;
                min-width: 0;
                flex-direction: column;
                vertical-align: top;
                box-sizing: inherit;
              "
            >
              <label
                class="MuiFormLabel-root MuiInputLabel-root MuiInputLabel-formControl MuiInputLabel-animated MuiInputLabel-outlined Mui-required Mui-required"
                style="
                  z-index: 1;
                  transform: translate(14px, 20px) scale(1);
                  pointer-events: none;
                  transition: color 200ms cubic-bezier(0.0, 0, 0.2, 1) 0ms,transform 200ms cubic-bezier(0.0, 0, 0.2, 1) 0ms;
                  top: 0;
                  left: 0;
                  position: absolute;
                  display: block;
                  transform-origin: top left;
                  color: rgba(0, 0, 0, 0.54);
                  padding: 0;
                  font-size: 1rem;
                  font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                  font-weight: 400;
                  line-height: 1;
                  letter-spacing: 0.00938em;
                  box-sizing: inherit;
                  cursor: default;
                "
                data-shrink="false"
                for="firstName"
                id="firstName-label"
              >
                {"First Name"}
                <span
                  aria-hidden="true"
                  class="MuiFormLabel-asterisk MuiInputLabel-asterisk"
                  style="
                    box-sizing: inherit;
                    color: rgba(0, 0, 0, 0.54);
                    padding: 0;
                    font-size: 1rem;
                    font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                    font-weight: 400;
                    line-height: 1;
                    letter-spacing: 0.00938em;
                    cursor: default;
                  "
                >
                  {"*"}
                </span>
              </label>
              <div
                class="MuiInputBase-root MuiOutlinedInput-root MuiInputBase-fullWidth MuiInputBase-formControl"
                style="
                  position: relative;
                  border-radius: 4px;
                  width: 100%;
                  color: rgba(0, 0, 0, 0.87);
                  cursor: text;
                  display: inline-flex;
                  position: relative;
                  font-size: 1rem;
                  box-sizing: border-box;
                  align-items: center;
                  font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                  font-weight: 400;
                  line-height: 1.1876em;
                  letter-spacing: 0.00938em;
                "
              >
                <input
                  aria-invalid="false"
                  autocomplete="fname"
                  id="firstName"
                  name="firstName"
                  // required=""
                  type="text"
                  class="MuiInputBase-input MuiOutlinedInput-input"
                  // value=""
                  style="
                    box-shadow: none;
                    padding: 18.5px 14px;
                    font: inherit;
                    color: currentColor;
                    width: 100%;
                    border: 0;
                    height: 1.1876em;
                    margin: 0;
                    display: block;
                    padding: 6px 0 7px;
                    min-width: 0;
                    background: none;
                    box-sizing: content-box;
                    animation-name: mui-auto-fill-cancel;
                    letter-spacing: inherit;
                    animation-duration: 10ms;
                    -webkit-tap-highlight-color: transparent;
                    -webkit-writing-mode: horizontal-tb !important;
                    text-rendering: auto;
                    word-spacing: normal;
                    text-transform: none;
                    text-indent: 0px;
                    text-shadow: none;
                    text-align: start;
                    appearance: auto;
                    -webkit-rtl-ordering: logical;
                    cursor: text;
                  "
                />
                <fieldset aria-hidden="true"
                  class="PrivateNotchedOutline-root-5 MuiOutlinedInput-notchedOutline"
                  style="
                    border-color: rgba(0, 0, 0, 0.23);
                    top: -5px;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    margin: 0;
                    padding: 0 8px;
                    overflow: hidden;
                    position: absolute;
                    border-style: solid;
                    border-width: 1px;
                    border-radius: inherit;
                    pointer-events: none;
                    box-sizing: inherit;
                    display: block;
                    margin-inline-start: 2px;
                    margin-inline-end: 2px;
                    padding-block-start: 0.35em;
                    padding-inline-start: 0.75em;
                    padding-inline-end: 0.75em;
                    padding-block-end: 0.625em;
                    min-inline-size: min-content;
                    border-image: initial;
                    color: rgba(0, 0, 0, 0.87);
                    cursor: text;
                    display: inline-flex;
                    position: relative;
                    font-size: 1rem;
                    box-sizing: border-box;
                    align-items: center;
                    font-family: 'Roboto', 'Helvetica', 'Arial', sans-serif;
                    font-weight: 400;
                    line-height: 1.1876em;
                    letter-spacing: 0.00938em;
                  "
                >
                  <legend
                    class="PrivateNotchedOutline-legendLabelled-7"
                    style="
                      width: auto;
                      height: 11px;
                      display: block;
                      padding: 0;
                      font-size: 0.75em;
                      max-width: 0.01px;
                      text-align: left;
                      transition: max-width 50ms cubic-bezier(0.0, 0, 0.2, 1) 0ms;
                      visibility: hidden;
                      box-sizing: inherit;
                      padding-inline-start: 2px;
                      padding-inline-end: 2px;
                      border-width: initial;
                      border-style: none;
                      border-color: initial;
                      border-image: initial;
                    "
                  >
                    <span
                      style="
                        display: inline-block;
                        padding-left: 5px;
                        padding-right: 5px;
                      "
                    >
                      {"First Name&nbsp;*"}
                    </span>
                  </legend>
                </fieldset>
              </div>
            </div>
          </div>
        }
    }
}
