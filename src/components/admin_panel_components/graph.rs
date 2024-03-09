use leptos::*;

#[component]
pub fn Graph() -> impl IntoView {
    view! {
        <div class="graph">
            // <p>graph</p>
            <canvas width="360" height="360"></canvas>
        </div>
    }
}