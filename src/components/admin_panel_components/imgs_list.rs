use leptos::*;
use crate::api::cats::*;

#[component]
pub fn ImgsList() -> impl IntoView {
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