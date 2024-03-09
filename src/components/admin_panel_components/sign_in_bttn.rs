use leptos::*;
use leptos_router::*;

#[component]
pub fn SignInBttn() -> impl IntoView {
    view!{
        <A href="/signin">
            Sign In
        </A>
    }
}