use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


use crate::components::todo_components::*;
use crate::components::admin_components::*;
use crate::components::auth_components::*;



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

                    <Route path="" view=HomePage/>
                    <Route path="/todo" view=TodoPage/>
                    <Route path="/admin" view=AdminPanel>
                        <Route path=":entity" view=Main />

                        <Route path="" view=|| view! {
                            <div>
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
fn HomePage() -> impl IntoView {

    view! {
        <div class="home-page">
            <nav>
                <A href="admin">"Admin"</A>
                <A href="todo">"Todo"</A>
            </nav>
        </div>
    }

}

/// Renders the home page of your application.
#[component]
fn TodoPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>) = create_signal(vec![]);

    view! {
        <div class="todo-app">
            <TodoInput todos={todos} />
            <TodoList todos={todos} />
        </div>
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