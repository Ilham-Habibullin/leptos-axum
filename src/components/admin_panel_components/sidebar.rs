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
    view! {
        <div class="sidebar-wrapper">
            <div class="sidebar">
                <UserInfo />

                <ul>

                    <p class="delimiter">Visuals</p>

                    <A href="tables" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/table-alt-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/table-alt-com_white.svg" style="width: 24px; height: 24px;"/>
                            Tables
                        </li>
                    </A>

                    <A href="panels" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/chart-tree-map-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/chart-tree-map-com_white.svg" style="width: 24px; height: 24px;"/>
                            Panels
                        </li>
                    </A>

                    <A href="graphs" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/chart-scatter-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/chart-scatter-com_white.svg" style="width: 24px; height: 24px;"/>
                            Graphs
                        </li>
                    </A>

                    <p class="delimiter">Implemented</p>

                    <SidebarListItem />

                    <A href="notes" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/table-list-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/table-list-com_white.svg" style="width: 24px; height: 24px;"/>
                            Notes
                        </li>
                    </A>

                    <A href="users" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-com_white.svg" style="width: 24px; height: 24px;"/>
                            Users
                        </li>
                    </A>

                    <A href="basics" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-com_white.svg" style="width: 24px; height: 24px;"/>
                            Basic users
                        </li>
                    </A>

                    <A href="moderators" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-plus-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-plus-com_white.svg" style="width: 24px; height: 24px;"/>
                            Moderators
                        </li>
                    </A>

                    <A href="admins" exact=true>
                        <li>
                            <img class="icon icon_dark" src="/svg/user-shield-com.svg" style="width: 24px; height: 24px;"/>
                            <img class="icon icon_light" src="/svg/user-shield-com_white.svg" style="width: 24px; height: 24px;"/>
                            Admins
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
                <img class="icon icon_dark" src="/svg/table-list-com.svg" style="width: 24px; height: 24px;"/>
                <img class="icon icon_light" src="/svg/table-list-com_white.svg" style="width: 24px; height: 24px;"/>
                Cats
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