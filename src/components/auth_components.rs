use leptos::*;
use log::info;
use leptos::error::Result;
use leptos::html::Input;


use crate::api::{auth::*, users::User};

#[component]
pub fn UserRegiesteredNotification(user: User) -> impl IntoView {
    return view!{
        <div class="user-created-notification">
            User was  successfully created
            <hr/>
            id: {user.id}
            <hr/>
            username: {user.username}
            <hr/>
            role: {user.role}
        </div>
    }
}

#[component]
pub fn SignUp() -> impl IntoView {
    let (get_username, set_username) = create_signal("".to_string());
    let (get_password, set_password) = create_signal("".to_string());

    let (get_user, set_user) = create_signal(Option::<User>::None);

    let signup_action = create_action(move |(username, password): &(String, String)| {
        let username_: String = username.to_owned();
        let password_: String = password.to_owned();
        async move {
            let user = signup(username_, password_).await.unwrap();
            set_user(Option::Some(user))
        }
    });

    view!{
        <div class="auth-form">

            {
                move || {
                    match get_user() {
                        None => view!{}.into_view(),
                        Some(user) => {
                            set_username("".to_string());
                            set_password("".to_string());

                            view!{ <UserRegiesteredNotification user /> }
                        }
                    }
                }
            }

            <input
                placeholder="username"
                on:input=move |ev| set_username(event_target_value(&ev))
                prop:value=get_username
            />

            <input
                type="password"
                placeholder="password"
                on:input=move |ev| set_password(event_target_value(&ev))
                prop:value=get_password
            />

            <input type="button" value="sign up" on:click=move |_| {
                let username = get_username();
                let password = get_password();

                signup_action.dispatch((username, password));
            } />
        </div>
    }
}






#[component]
pub fn SignIn() -> impl IntoView {
    let username_input_ref = create_node_ref::<Input>();
    let password_input_ref = create_node_ref::<Input>();

    let signin_action = create_action(|(username, password): &(String, String)| {
        let username_: String = username.to_owned();
        let password_: String = password.to_owned();
        async move {
            match signin(username_, password_).await {
                Err(err) => {},
                Ok(_) => {
                    let navigate = leptos_router::use_navigate();
                    navigate("/admin", Default::default()); 
                }
            }
        }
    });

    view!{
        <div class="auth-form">
            <input
                placeholder="username"
                node_ref=username_input_ref
            />

            <input
                type="password"
                placeholder="password"
                node_ref=password_input_ref
            />

            <input type="button" value="sign in" on:click=move |_| {
                let username = username_input_ref.get().expect("username input expected to exist");
                let password = password_input_ref.get().expect("password input expected to exist");

                signin_action.dispatch((username.value(), password.value()));
            } />
        </div>
    }
}