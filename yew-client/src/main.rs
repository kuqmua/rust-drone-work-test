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
            <div id="root">
            <main class="MuiContainer-root MuiContainer-maxWidthXs">
            <div class="makeStyles-paper-1">
            <div class="MuiAvatar-root MuiAvatar-circular makeStyles-avatar-2 MuiAvatar-colorDefault">
            <svg class="MuiSvgIcon-root MuiAvatar-fallback" focusable="false" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"></path>
            </svg>
            </div>
            <h1 class="MuiTypography-root MuiTypography-h5">{"Sign up"}</h1>
            // <form class="makeStyles-form-3" novalidate="">
            <div class="MuiGrid-root MuiGrid-container MuiGrid-spacing-xs-2">
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12 MuiGrid-grid-sm-6">
            <div class="MuiFormControl-root MuiTextField-root MuiFormControl-fullWidth">
            <label class="MuiFormLabel-root MuiInputLabel-root MuiInputLabel-formControl MuiInputLabel-animated MuiInputLabel-outlined Mui-required Mui-required" data-shrink="false" for="firstName" id="firstName-label">
            {"First Name"}
            <span aria-hidden="true" class="MuiFormLabel-asterisk MuiInputLabel-asterisk">
            {"*"}
            </span>
            </label>
            <div class="MuiInputBase-root MuiOutlinedInput-root MuiInputBase-fullWidth MuiInputBase-formControl">
            <input
            aria-invalid="false"
            autocomplete="fname"
            id="firstName"
            name="firstName"
            // required=""
            type="text"
            class="MuiInputBase-input MuiOutlinedInput-input"
            // value=""
            />
            <fieldset aria-hidden="true" class="PrivateNotchedOutline-root-5 MuiOutlinedInput-notchedOutline">
            <legend class="PrivateNotchedOutline-legendLabelled-7">
            <span>{"First Name&nbsp;*"}</span>
            </legend>
            </fieldset>
            </div>
            </div>
            </div>
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12 MuiGrid-grid-sm-6">
            <div class="MuiFormControl-root MuiTextField-root MuiFormControl-fullWidth">
            <label class="MuiFormLabel-root MuiInputLabel-root MuiInputLabel-formControl MuiInputLabel-animated MuiInputLabel-outlined Mui-required Mui-required" data-shrink="false" for="lastName" id="lastName-label">
            {"Last Name"}
            <span aria-hidden="true" class="MuiFormLabel-asterisk MuiInputLabel-asterisk">{" *"}</span>
            </label>
            <div class="MuiInputBase-root MuiOutlinedInput-root MuiInputBase-fullWidth MuiInputBase-formControl">
            <input
            aria-invalid="false"
            autocomplete="lname"
            id="lastName"
            name="lastName"
            // required=""
            type="text"
            class="MuiInputBase-input MuiOutlinedInput-input"
            // value=""
            />
            <fieldset aria-hidden="true" class="PrivateNotchedOutline-root-5 MuiOutlinedInput-notchedOutline">
            <legend class="PrivateNotchedOutline-legendLabelled-7">
            <span>{"Last Name&nbsp;*"}</span>
            </legend>
            </fieldset>
            </div>
            </div>
            </div>
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12">
            <div class="MuiFormControl-root MuiTextField-root MuiFormControl-fullWidth">
            <label class="MuiFormLabel-root MuiInputLabel-root MuiInputLabel-formControl MuiInputLabel-animated MuiInputLabel-outlined Mui-required Mui-required" data-shrink="false" for="email" id="email-label">
            {"Email Address"}
            <span aria-hidden="true" class="MuiFormLabel-asterisk MuiInputLabel-asterisk">{" *"}</span>
            </label>
            <div class="MuiInputBase-root MuiOutlinedInput-root MuiInputBase-fullWidth MuiInputBase-formControl">
            <input 
            aria-invalid="false" 
            autocomplete="email" 
            id="email" 
            name="email" 
            // required="" 
            type="text" 
            class="MuiInputBase-input MuiOutlinedInput-input" 
            // value=""
            />
            <fieldset aria-hidden="true" class="PrivateNotchedOutline-root-5 MuiOutlinedInput-notchedOutline">
            <legend class="PrivateNotchedOutline-legendLabelled-7">
            <span>{"Email Address&nbsp;*"}</span>
            </legend>
            </fieldset>
            </div>
            </div>
            </div>
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12">
            <div class="MuiFormControl-root MuiTextField-root MuiFormControl-fullWidth">
            <label class="MuiFormLabel-root MuiInputLabel-root MuiInputLabel-formControl MuiInputLabel-animated MuiInputLabel-outlined Mui-required Mui-required" data-shrink="false" for="password" id="password-label">
            {"Password"}
            <span aria-hidden="true" class="MuiFormLabel-asterisk MuiInputLabel-asterisk">{" *"}</span>
            </label>
            <div class="MuiInputBase-root MuiOutlinedInput-root MuiInputBase-fullWidth MuiInputBase-formControl">
            <input 
            aria-invalid="false" 
            autocomplete="current-password" 
            id="password" 
            name="password" 
            // required="" 
            type="password" 
            class="MuiInputBase-input MuiOutlinedInput-input" 
            // value=""
            />
            <fieldset aria-hidden="true" class="PrivateNotchedOutline-root-5 MuiOutlinedInput-notchedOutline">
            <legend class="PrivateNotchedOutline-legendLabelled-7">
            <span>{"Password&nbsp;*"}</span>
            </legend>
            </fieldset>
            </div>
            </div>
            </div>
            <div class="MuiGrid-root MuiGrid-item MuiGrid-grid-xs-12">
            <label class="MuiFormControlLabel-root">
            <span class="MuiButtonBase-root MuiIconButton-root PrivateSwitchBase-root-9 MuiCheckbox-root MuiCheckbox-colorPrimary MuiIconButton-colorPrimary" aria-disabled="false">
            <span class="MuiIconButton-label">
            <input 
            class="PrivateSwitchBase-input-12" 
            type="checkbox" 
            data-indeterminate="false" 
            value="allowExtraEmails"
            />
            <svg class="MuiSvgIcon-root" focusable="false" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M19 5v14H5V5h14m0-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z">
            </path>
            </svg>
            </span>
            <span class="MuiTouchRipple-root"></span>
            </span>
            <span class="MuiTypography-root MuiFormControlLabel-label MuiTypography-body1">{"I want to receive inspiration, marketing promotions and updates via email."}</span>
            </label>
            </div>
            </div>
            <button class="MuiButtonBase-root MuiButton-root MuiButton-contained makeStyles-submit-4 MuiButton-containedPrimary MuiButton-fullWidth" tabindex="0" type="submit">
            <span class="MuiButton-label">{"Sign Up"}</span>
            <span class="MuiTouchRipple-root"></span>
            </button>
            <div class="MuiGrid-root MuiGrid-container MuiGrid-justify-content-xs-flex-end">
            <div class="MuiGrid-root MuiGrid-item">
            <a class="MuiTypography-root MuiLink-root MuiLink-underlineHover MuiTypography-body2 MuiTypography-colorPrimary" href="#">{"Already have an account? Sign in"}</a>
            </div>
            </div>
            // </form>
            </div><div class="MuiBox-root MuiBox-root-13"><p class="MuiTypography-root MuiTypography-body2 MuiTypography-colorTextSecondary MuiTypography-alignCenter">{"Copyright © "}<a class="MuiTypography-root MuiLink-root MuiLink-underlineHover MuiTypography-colorInherit" href="https://material-ui.com/">{"Your Website"}</a>{" 2021."}</p>
            </div>
            </main>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
