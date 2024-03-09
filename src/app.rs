use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::auth_components::{SignIn, SignUp};

use crate::components::admin_panel_components::Main;
use crate::components::admin_panel_components::sidebar::Sidebar;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-axum-frontend.css"/>

        // sets the document title
        <Title text="Admin Panel"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>

                    <Route path="/signin" view=SignIn/>
                    <Route path="/signup" view=SignUp/>

                    <Route path="" view=|| view!{ <Redirect path="/admin" /> }/>
                    <Route path="/admin" view=AdminPanel>
                        <Route path=":entity" view=Main />

                        <Route path="" view=|| view! {
                            <div class="no-choosen-panel">
                                "panel was not chosen"
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}


#[component]
fn AdminPanel() -> impl IntoView {
    view! {
        <div class="container">
            <Sidebar />
            <Outlet />
        </div>
    }
}