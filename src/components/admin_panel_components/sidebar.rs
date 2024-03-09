use std::str::FromStr;

use leptos::*;
use leptos_router::*;

use crate::api::auth::MeError;
use crate::components::admin_panel_components::{
    sign_in_bttn::SignInBttn,
    sign_out_bttn::SignOutBttn,
    sign_up_bttn::SignUpBttn,
};

#[component]
fn UserInfoIfNotToken() -> impl IntoView {
    view! {
        <SignInBttn />
        <SignUpBttn />
    }
}

#[component]
fn UserInfo() -> impl IntoView {
    let me = create_local_resource(|| (), |_| async {
        let result = crate::api::auth::me().await;
        result
    });

    view! {
        <div class="auth-info">
            {
                move || {
                    match me.get() {
                        Some(user) => match user {
                            Ok(user) => view!{
                                <SignOutBttn user />
                            },
                            Err(err) => match err {
                                MeError::Unauthorized => UserInfoIfNotToken().into_view(),
                                MeError::GlooNetError(_value) => view!{
                                    <Redirect path="/signin" />
                                },
                            }
                        },
                        None => UserInfoIfNotToken().into_view()
                    }
    
                }
            }
        </div>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {

    let sidebar_wrapper_ref = create_node_ref::<leptos::html::Div>();
    // let sidebar_hide_bttn_ref = create_node_ref::<leptos::html::Img>();

    let hide_sidebar = move |_| {

        if let Some(el) = sidebar_wrapper_ref.get_untracked() {
            let sidebar_classlist = el.class_list();
            let hidden_string = &web_sys::js_sys::JsString::from_str("hidden").unwrap();
            let array = &web_sys::js_sys::Array::new();

            array.push(&hidden_string);

            let _ = match sidebar_classlist.contains("hidden") {
                true => sidebar_classlist.remove(array),
                false => sidebar_classlist.add(array)
            };
        }

    };


    view! {
        <div class="sidebar-wrapper" node_ref=sidebar_wrapper_ref>
            <div class="sidebar">

                <div class="sidebar-header">
                    <img
                        id="sidebar-hide-bttn"
                        class="sidebar-hide-bttn icon icon_light"
                        src="/svg/sidebar_white.svg"
                        style="width: 34px; height: 34px;"
                        on:click=hide_sidebar
                    />

                    <div class="sidebar-header-divider-wrapper">
                        <div class="sidebar-header-divider"></div>
                    </div>
                </div>

                <UserInfo />

                <ul>
                    <p class="delimiter">Visuals</p>

                    <A href="tables" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/table-alt.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/table-alt_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Tables</span>
                        </li>
                    </A>

                    <A href="panels" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/chart-tree-map.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/chart-tree-map_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Panels</span>
                        </li>
                    </A>

                    <A href="graphs" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/chart-scatter.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/chart-scatter_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Graphs</span>
                        </li>
                    </A>

                    <p class="delimiter">Implemented</p>

                    <SidebarListItem />

                    <A href="notes" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/table-list.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/table-list_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Notes</span>
                        </li>
                    </A>

                    <A href="users" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Users</span>
                        </li>
                    </A>

                    <A href="basics" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Basic users</span>
                        </li>
                    </A>

                    <A href="moderators" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-plus.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-plus_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Moderators</span>
                        </li>
                    </A>

                    <A href="admins" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-shield.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-shield_white.svg" style="width: 24px; height: 24px;"/>
                            <span class="sidebar-item-text">Admins</span>
                        </li>
                    </A>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn SidebarListItem() -> impl IntoView {
    view!{
        <A class="sidebar-item-with-list" href="cats" exact=true>
            <li>
                <img class="icon icon_dark" src="/svg/table-list.svg" style="width: 24px; height: 24px;"/>
                <img class="icon icon_light" src="/svg/table-list_white.svg" style="width: 24px; height: 24px;"/>
                <span class="sidebar-item-text">Cats</span>
            </li>
            <div class="nested-sidebar-list">
                <ul>
                    <li>Maine</li>
                    <li>Persian</li>
                    <li>Sphynx</li>
                    <li>Scottish</li>
                </ul>
            </div>
        </A>
    }
}
