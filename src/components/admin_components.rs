use leptos_router::*;

use log::info;
use leptos::*;

use crate::api::cats::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar-wrapper">
            <div class="sidebar">
                <ul>
                    <A href="/sign_out">
                        <li>Sign out</li>
                    </A>

                    <div class="me-info">
                        <p>username: Lorem Ipsum</p>
                        <p>role: Administrator</p>
                        <p>registered at: 26.11.2023</p>
                    </div>
                </ul>

                <ul>
                    <A href="cats">
                        <li>Cats</li>
                    </A>
                    
                    <A href="movies">
                        <li>Movies</li>
                    </A>

                    <A href="wallpapers">
                        <li>Wallpapers</li>
                    </A>

                    <A href="memes">
                        <li>Memes</li>
                    </A>

                    <A href="users">
                        <li>Users</li>
                    </A>

                    <A href="moderators">
                        <li>Moderators</li>
                    </A>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn Main() -> impl IntoView {
    let params = use_params_map();
    let get_entity = move || {
        let x = params.with(|params| params.get("entity").cloned().unwrap_or_default());
        return x
    };

    let (cat_count_5, _set_cat_count) = create_signal::<CatCount>(5);
    let (cat_count_10, _set_cat_count) = create_signal::<CatCount>(10);
    let cats5 = create_local_resource(cat_count_5, fetch_cats);
    let cats10 = create_local_resource(cat_count_10, fetch_cats);

    view! {
        <div class="main">
            <SearchBar />

            {
                move || {

                    let x = get_entity();

                    let catter = match x.as_str() {
                        "cats" => {
                            cats5
                        },
                        _ => {
                            cats10
                        }
                    };

                    match catter.get() {
                        None => view! { <p>"Loading..."</p> }.into_view(),
                        Some(data) => {

                            match data {
                                Err(err) => {
                                    view! { <p>"Err Loading..."</p> }.into_view()
                                },
                                Ok(val) => {
                                    view! {

                                        {
                                            val
                                            .into_iter()
                                            .map(|n| view! {
                                                <img src={n}> </img>
                                            })
                                            .collect_view()
                                    }
                                    }.into_view()
                                }
                            }


                        },
                    }

                }
            }

        </div>
    }
}


#[component]
fn SearchBar() -> impl IntoView {
    let params = use_params_map();
    let get_entity = move || {
        let x = params.with(|params| params.get("entity").cloned().unwrap_or_default());
        return format!("search {}", x)
    };

    view! {
        <div class="searchbar-wrapper">
        <input 
            type="text" 
            class="searchbar"
            placeholder={get_entity}
        />
        </div>
    }
}
