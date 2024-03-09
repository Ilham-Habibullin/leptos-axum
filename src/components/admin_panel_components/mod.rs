pub mod sign_in_bttn;
pub mod sign_out_bttn;
pub mod sign_up_bttn;
pub mod tables_list;
pub mod graph;
pub mod table;
pub mod panels;
pub mod imgs_list;
pub mod sidebar;

use leptos::*;
use leptos_router::*;

use crate::components::admin_panel_components::tables_list::TablesList;
use crate::components::admin_panel_components::imgs_list::ImgsList;
use crate::components::admin_panel_components::graph::Graph;
use crate::components::admin_panel_components::table::Table;
use crate::components::admin_panel_components::panels::Panels;

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
                    "tables" => view!{<Table />},
                    "panels" => view!{<Panels />},
                    "graphs" => view!{<Graph />},
                    other => panic!("unexpected entity: {}", other),
                }
            }
        </div>
    }
}