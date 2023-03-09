use log::info;

use common::{Location, LocationDetailsProps, LocationListProps};
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(LocationsList)]
fn locations_list(
    LocationListProps {
        locations,
        on_click,
    }: &LocationListProps,
) -> Html {
    let on_click = on_click.clone();

    locations
        .iter()
        .map(|location| {
            let on_location_select = {
                let on_click = on_click.clone();
                let location = location.clone();
                Callback::from(move |_| on_click.emit(location.clone()))
            };
            info!("Help");
            html! {
                <p key={location.id} onclick = {on_location_select}>{format!("{}:{}", location.name, location.url)}</p>
            }
        })
        .collect()
}

#[function_component(LocationDetails)]
fn location_details(LocationDetailsProps { location }: &LocationDetailsProps) -> Html {
    html! {
        <div>
            <h3>{location.name.clone()}</h3>
            <p>{location.lat.clone()}</p>
            <p>{location.long.clone()}</p>

        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let locations = use_state(|| vec![]);
    {
        let locations = locations.clone();
        use_effect_with_deps(
            move |_| {
                let locations = locations.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO change path to get back backend API
                    let fetched_locations: Vec<Location> = Request::get(r"\locations_flat.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    locations.set(fetched_locations);
                });
                || ()
            },
            (),
        );
    }

    let selected_location = use_state(|| None);

    let on_location_select = {
        let selected_location = selected_location.clone();
        Callback::from(move |location: Location| selected_location.set(Some(location)))
    };

    let details = selected_location.as_ref().map(|location| {
        html! {
            <LocationDetails location={location.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "Goldilocks" }</h1>
            <div>
                <h3>{"Climbing Locations"}</h3>
              <LocationsList locations={(*locations).clone()}  on_click = {on_location_select.clone()}/>
            </div>
            {for details}

        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
