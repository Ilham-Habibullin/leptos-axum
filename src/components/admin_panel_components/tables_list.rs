use leptos::*;
use leptos_router::*;
use leptos::ev::scroll;
use leptos::html::Div;

use log::error;
use std::collections::{BTreeMap, BTreeSet};
use serde_json::Value;

use crate::api::users::*;
use crate::api::notes::*;
use crate::common_types::{Pagination, VecOfMaps};

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

#[component]
fn EntityCreationTable(columns: std::collections::BTreeSet<String>) -> impl IntoView {
    view! {
        <div class="new-item-table">
            {
                columns
                    .into_iter()
                    .map(|column| view! { 
                        <div class="new-item-row">
                            <div class="new-item-header">{column}</div>
                            <div class="new-item-cell">
                                <input value="" />
                            </div>
                        </div>
                    })
                    .collect_view()
            }

            <div class="new-item-row">
                <div class="new-item-header">Selectable</div>
                <div class="new-item-cell">
                    <select>
                        <option value="dog">Dog</option>
                        <option value="cat">Cat</option>
                        <option value="hamster">Hamster</option>
                        <option value="parrot">Parrot</option>
                        <option value="spider">Spider</option>
                    </select>
                </div>
            </div>

            <div class="item-table-controls">
                <input type="button" value="create"/>
            </div>
        </div>
    }
}

#[component]
fn TableFromMap(map: BTreeMap<String, Value>) -> impl IntoView {
    view! {
        <div class="item-table-wrapper">
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
                            <div class="item-table-row">
                                <div class="item-table-header">{key}</div>
                                <div class="item-table-cell">
                                    <input type="text" value={val_string} />
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }


            <div class="item-table-controls">
                <input type="button" value="update"/>
                <input type="button" value="delete"/>
                <input type="button" value="drop"/>
            </div>
        </div>
    }
}

#[component]
pub fn TablesList(entity_name: String) -> impl IntoView {
    let (pagination, set_pagination) = create_signal::<Pagination>(Pagination::default());
    let (maps, set_maps) = create_signal::<VecOfMaps>(vec![]);
    let (count, set_count) = create_signal::<String>("0".to_string());
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
            <div class="main-content">
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

            <div class="main-sidebar">
                <div class="main-sidebar-item"></div>
                <div class="main-sidebar-item"></div>
            </div>
        </div>


    }
}