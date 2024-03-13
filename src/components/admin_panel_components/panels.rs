use leptos::*;

#[component]
pub fn Panels() -> impl IntoView {
    view! {
        <div class="panels">
            <div class="panels-header">
                Lorem ipsum dolor
            </div>

            <div class="panels-body">
                <NewServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
                <ServicePanel000 />
            </div>
        </div>
    }
}

#[component]
pub fn ServicePanel000() -> impl IntoView {
    view! {
        <div class="service-panel_000">
            <div class="service-panel_000-header">
                
                <p class="service-panel_000-header_service-name">service name</p>
                <p class="service-panel_000-header_divider"></p>
                <p class="service-panel_000-header_service-status green-color">online</p>

            </div>

            <div class="service-panel_000-body">

                <div class="service-panel_000-body_message-form">
                    <input type="text" placeholder="send message"/>
                    <select>
                            <option value="service_1">service_1</option>
                            <option value="service_2">service_2</option>
                            <option value="service_3">service_3</option>
                            <option value="service_4">service_4</option>
                            <option value="service_5">service_5</option>
                    </select>
                    <input type="button" value="send"/>
                </div>

                <div class="service-panel_000-body_numbers-preview_0">
                    100
                </div>

                <div class="service-panel_000-body_numbers-preview_0">
                    300
                </div>

                <div class="service-panel_000-body_numbers-preview_0">
                    73221
                </div>

                <div class="service-panel_000-body_numbers-preview_1">
                    250
                </div>



            </div>

            <div class="service-panel_000-controls">
                <input type="button" value="show more"/>
            </div>
        </div>
    }
}

#[component]
pub fn NewServicePanel000() -> impl IntoView {
    view! {
        <div class="new-service-panel_000">
            New service
        </div>
    }
}
