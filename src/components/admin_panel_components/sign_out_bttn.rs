use leptos::*;
use leptos_router::*;

use crate::api::users::*;
use crate::api::auth::signout;

#[component]
pub fn SignOutBttn(user: User) -> impl IntoView {
    let signout_action = create_action(|_: &()| {
        async move {
            match signout().await {
                Err(_err) => {
                    todo!()
                },
                Ok(_) => {
                    view!{<Redirect path="/" />}
                }
            }
        }
    });

    view!{
        <div>
            <button class="sign-out-button" type="button" on:click=move |_| signout_action.dispatch(())>
                Sign out
            </button>

            <div class="me-info">

                You logged as user with: 

                <p>id: {user.id}</p>
                <p>username: {user.username}</p>
                <p>role: {user.role}</p>
                // <p>registered at: 26.11.2023</p>
            </div>
        </div>
    }
}