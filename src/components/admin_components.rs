use leptos::ev::scroll;
use leptos::html::Div;
use leptos_router::*;
use log::error;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use serde_json::Value;

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

                    <A href="basics">
                        <li>Basic users</li>
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

impl Default for Pagination {
    fn default() -> Self {
        Self(5, 0)
    }
}

pub type VecOfMaps = Vec<BTreeMap<String, Value>>;
pub type Count = String;

#[component]
fn TablesList(entity_name: String) -> impl IntoView {
    let (pagination, set_pagination) = create_signal::<Pagination>(Pagination::default());
    let (maps, set_maps) = create_signal::<VecOfMaps>(vec![]);
    let (count, set_count) = create_signal::<Count>("0".to_string());
    let (search, set_search) = create_signal::<String>("".to_string());
    let (prev_search, set_prev_search) = create_signal::<String>("".to_string());

    let fetcher_for_users = move |role| create_local_resource(
        pagination,
        move |pagination| {
            let previous_search = search.get();

            if prev_search.get_untracked() == search.get_untracked() {
                // if search field entry was not changed and fetcher was called due to scrolling then we keep using maps that was previously obtained and conitinue to filling it with new entities
                let maps = maps.get_untracked();
                get_users(pagination, maps, role, search.get_untracked())
            } else {
                // if search field entry changed then we clearing maps and filling it from scratch with newly requested entities
                let x = get_users(pagination, vec![], role, search.get_untracked());
                set_prev_search(previous_search);
                x
            }
        }
    );

    let resource = match entity_name.as_str() {
        "users"      => fetcher_for_users(None),
        "basics"     => fetcher_for_users(Some(0)),
        "moderators" => fetcher_for_users(Some(1)),
        "admins"     => fetcher_for_users(Some(2)),
        "notes"      => create_local_resource(pagination, move |p| get_notes(p, maps.get_untracked())),
        other => panic!("unexpected entity: {}", other)
    };

    let table_ref = leptos::create_node_ref::<Div>();

    let add_items = move || match leptos::window().inner_height() {
        Err(_err) => error!("there was no inner height in window"),
        Ok(inner_height) => match table_ref.get_untracked() {
            None => log::info!("there was no element"),
            Some(element) => {
                let dom_rect = element.get_bounding_client_rect();
                let bottom = dom_rect.bottom();
                let inner_height = inner_height.as_f64().unwrap();
                let item_height = 300.0; // supposed to be double of size of item of list;
                let remaining = inner_height > bottom - item_height;

                let Pagination(limit, offset) = pagination.get_untracked();
                let count_as_number = count().parse::<usize>().unwrap();

                if remaining && offset < count_as_number && search.get_untracked() == prev_search.get_untracked() {
                    set_pagination(Pagination(limit, offset + 5))
                }
            }
        }
    };

    let handle = window_event_listener(scroll, move |_ev| add_items());
    on_cleanup(move || handle.remove());

    // triggers initial obtaining of items
    create_effect(move |_| match resource.get() {
        None => {
            // todo!()
        },
        Some(data) => match data {
            Err(_err) => {
                // todo!()
            },
            Ok((new_vec_of_maps, new_count)) => {
                set_maps(new_vec_of_maps);
                set_count(new_count);

                add_items();
            }
        },
    });

    view!{
        <SearchBar set_search set_pagination />

        <div class="tables-list" node_ref=table_ref>

            <EntityCreationTable columns=BTreeSet::from([
                "Lorem".to_string(),
                "Ipsum".to_string(),
                "Dolor".to_string(),
                "Set".to_string(),
                "Amet".to_string(),
            ]) />

            {
                move || maps()
                    .into_iter()
                    .map(|n| view! { <TableFromMap map=n.clone() /> })
                    .collect_view()
            }
        </div>
    }
}

#[component]
fn EntityCreationTable(columns: std::collections::BTreeSet<String>) -> impl IntoView {
    view! {
        <div>
            <table>
                {
                    columns
                        .into_iter()
                        .map(|column| view! { 
                            <tr>
                                <th>{column}</th>
                                <td>
                                    <input value="" />
                                </td>
                            </tr>
                         })
                        .collect_view()
                }
            </table>
        </div>
    }
}

#[component]
fn ImgsList() -> impl IntoView {
    let (count, _set_count) = create_signal::<usize>(5);

    let resource = create_local_resource(count, fetch_cats);

    view! {
        {
            move || match resource.get() {
                None => view! { <p>"Loading..."</p> }.into_view(),
                Some(data) => match data {
                    Err(_err) => view! { <p>"Err Loading..."</p> }.into_view(),
                    Ok(val) => val
                        .into_iter()
                        .map(|n| view! {<Img url=n />})
                        .collect_view()
                },
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
        <div class="item-table-wrapper">
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
                                <td>
                                    <input type="text" value={val_string} />
                                </td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()
                }
            </table>

            <input type="button" value="update"/>
            <input type="button" value="delete"/>
        </div>
    }
}


#[component]
fn SearchBar(set_search: WriteSignal<String>, set_pagination: WriteSignal<Pagination>) -> impl IntoView {
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
            on:input=move |ev| {
                set_pagination(Pagination::default());
                set_search(event_target_value(&ev))
            }
            type="text" 
            class="searchbar"
            placeholder={get_entity}
        />
        </div>
    }
}
