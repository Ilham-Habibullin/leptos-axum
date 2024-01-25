use leptos::ev::scroll;
use leptos::html::Div;
use leptos_router::*;
use log::error;
use std::collections::BTreeMap;
use std::future::Future;
use serde_json::Value;
use leptos_router::Redirect;

use log::info;
use leptos::*;

use crate::api::auth::MeError;
use crate::api::cats::*;
use crate::api::users::*;
use crate::api::notes::*;

#[component]
pub fn SignOutBttn(user: User) -> impl IntoView {
    view!{
        <li>
            <A href="/signout">
                Sign out
            </A>

            <div class="me-info">
                <p>id: {user.id}</p>
                <p>username: {user.username}</p>
                <p>role: {user.role}</p>
                // <p>registered at: 26.11.2023</p>
            </div>
        </li>
    }
}

#[component]
pub fn SignInBttn() -> impl IntoView {
    view!{
        <li>
            <A href="/signin">
                Sign In
            </A>
        </li>
    }
}

#[component]
pub fn SignUpBttn() -> impl IntoView {
    view!{
        <li>
            <A href="/signup">
                Sign Up
            </A>
        </li>
    }
}

#[component]
pub fn UserInfo() -> impl IntoView {
    let me = create_local_resource(|| (), |_| async {
        let result = crate::api::auth::me().await;
        result
    });

    view! {
        <ul>
            {
                move || {
                    match me.get() {
                        Some(user) => match user {
                            Ok(user) => view!{
                                <SignOutBttn user />
                            },
                            Err(err) => match err {
                                MeError::Unauthorized => {
                                    // view!{
                                    //     <SignInBttn />
                                    //     <SignUpBttn />
                                    // }.into_view()
                                    view!{<Redirect path="/signin" />}
                                },
                                MeError::GlooNetError(_value) => view!{
                                    <Redirect path="/signin" />
                                },
                            }
                        },
                        None => {
                            view!{
                                <SignInBttn />
                                <SignUpBttn />
                            }.into_view()
                        }
                    }
    
                }
            }
        </ul>
    }
}


#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar-wrapper">
            <div class="sidebar">
                <UserInfo />

                <ul>
                    <A href="cats">
                        <li>Cats</li>
                    </A>

                    <A href="notes">
                        <li>Notes</li>
                    </A>

                    <A href="users">
                        <li>Users</li>
                    </A>

                    <A href="moderators">
                        <li>Moderators</li>
                    </A>

                    <A href="admins">
                        <li>Admins</li>
                    </A>
                </ul>
            </div>
        </div>
    }
}


#[component]
pub fn Main() -> impl IntoView {
    let params = use_params_map();
    let get_entity = move || params.with(|params| params
        .get("entity")
        .cloned()
        .unwrap_or_default());

    view! {
        <div class="main">
            {
                move || match get_entity().as_str() {
                    x @ ("users" | "admins" | "moderators" | "basics" | "notes") => view!{<TablesList entity_name=x.to_string() />},
                    "cats" => view!{<ImgsList />},
                    other => panic!("unexpected entity: {}", other),
                }
            }
        </div>
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pagination(pub usize, pub usize);

pub type VecOfMaps = Vec<BTreeMap<String, Value>>;

pub type Count = String;

type FetcherResponse = leptos::error::Result<(VecOfMaps, Count)>;

type Fetcher = Box<dyn Fn(Pagination, VecOfMaps) -> std::pin::Pin<Box<dyn Future<Output = FetcherResponse>>>>;

fn force_boxed<T>(f: fn(Pagination, VecOfMaps) -> T) -> Fetcher where T: Future<Output = FetcherResponse> + 'static {
    Box::new(move |p, vec| Box::pin(f(p, vec)))
}

#[component]
fn TablesList(entity_name: String) -> impl IntoView {
    let (pagination, set_pagination) = create_signal::<Pagination>(Pagination(5, 0));
    let (maps, set_maps) = create_signal::<VecOfMaps>(vec![]);
    let (count, set_count) = create_signal::<Count>("0".to_string());

    let fetcher = match entity_name.as_str() {
        "users" => force_boxed(|p, maps| get_users(p, maps, None)),
        "admins" => force_boxed(|p, maps| get_users(p, maps, Some(2))),
        "moderators" => force_boxed(|p, maps| get_users(p, maps, Some(1))),
        "basics" => force_boxed(|p, maps| get_users(p, maps, Some(0))),
        "notes" => force_boxed(get_notes),
        other => panic!("unexpected entity: {}", other)
    };

    let resource = create_local_resource(
        pagination,
        move |p| fetcher(p, maps.get_untracked())
    );

    let table_ref = leptos::create_node_ref::<Div>();

    let add_items = move || match leptos::window().inner_height() {
        Err(_err) => error!("there was no inner height in window"),
        Ok(inner_height) => match table_ref.get_untracked() {
            None => info!("there was no element"),
            Some(element) => {
                let dom_rect = element.get_bounding_client_rect();
                let bottom = dom_rect.bottom();
                let inner_height = inner_height.as_f64().unwrap();
                let item_height = 300.0; // supposed to be double of size of item of list;
                let remaining = inner_height > bottom - item_height;

                let Pagination(limit, offset) = pagination.get_untracked();
                let count_as_number = count().parse::<usize>().unwrap();

                // info!("inner height is: {:?} and bottom is: {}, remaingin is {}", inner_height, bottom, remaining);
                if remaining && offset < count_as_number {
                    set_pagination(Pagination(limit, offset + 5))
                }
            }
        }
    };

    window_event_listener(scroll, move |_ev| add_items());

    create_effect(move |_| match resource.get() {
        None => {
            todo!()
        },
        Some(data) => match data {
            Err(_err) => {
                todo!()
            },
            Ok((new_vec_of_maps, new_count)) => {
                set_maps(new_vec_of_maps);
                set_count(new_count);

                add_items();
            }
        },
    });

    view!{
        <SearchBar />

        <div class="tables-list" node_ref=table_ref>
            {
                move || maps()
                    .into_iter()
                    .map(|n| view! {<TableFromMap map=n.clone() />})
                    .collect_view()
            }
        </div>
    }
}

#[component]
fn ImgsList() -> impl IntoView {
    let (count, _set_count) = create_signal::<usize>(5);

    let resource = create_local_resource(count, fetch_cats);

    view! {
        {
            move || {
                match resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(data) => {
                        match data {
                            Err(_err) => view! { <p>"Err Loading..."</p> }.into_view(),
                            Ok(val) => val
                                .into_iter()
                                .map(|n| view! {<Img url=n />})
                                .collect_view()
                        }
                    },
                }
            }
        }
    }
}


#[component]
fn Img(url: String) -> impl IntoView {
    view! {<img src=url> </img>}
}

#[component]
fn TableFromMap(map: BTreeMap<String, Value>) -> impl IntoView {
    view! {
        <table>
            {
                map.iter().map(|(key, val)| {
                    let val_string = match val {
                        Value::Number(val) => val.to_string(),
                        Value::String(val) => val.to_string(),
                        Value::Bool(val) => val.to_string(),
                        Value::Array(_val) => "it was array".to_string(),
                        Value::Object(_val) => "it was object".to_string(),
                        Value::Null => "it was null".to_string(),
                    };

                    view! {
                        <tr>
                            <th>{key}</th>
                            <td>{val_string}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>()
            }
        </table>
    }
}


#[component]
fn SearchBar() -> impl IntoView {
    let params = use_params_map();
    let get_entity = move || {
        let enitity_name = params.with(|params| params
            .get("entity")
            .cloned()
            .unwrap_or_default());

        format!("search {enitity_name}")
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
