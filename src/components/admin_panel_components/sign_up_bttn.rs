use leptos::*;
use leptos_router::*;

#[component]
pub fn SignUpBttn() -> impl IntoView {
    view!{
        <A href="/signup">
            Sign Up
        </A>
    }
}