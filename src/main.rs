use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::{function_component, html, use_state, Callback, Properties, UseStateHandle};
use youtube::search_youtube;

mod env;
mod youtube;

fn main() {
    yew::start_app::<App>();
}

#[derive(Clone, Debug)]
struct Video {
    id: String,
    name: String,
}

#[function_component(App)]
fn app() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_search = {
        let video = video.clone();
        Callback::from(move |text_to_search: String| {
            let video = video.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match search_youtube(text_to_search).await {
                    Ok(video_item) => video.set(Some(Video {
                        id: video_item.id.video_id,
                        name: video_item.snippet.title,
                    })),
                    Err(e) => {
                        web_sys::console::log_1(&e.to_string().into());
                    }
                }
            });
        })
    };

    let video_section = match (*video).clone() {
        Some(video) => html! {
            <VideoSection name={video.name} id={video.id} />
        },
        None => html! {},
    };

    html! {
        <main>
        <VideoControls on_search={on_search} />
        {video_section}
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct VideoControlProps {
    on_search: Callback<String>,
}

#[function_component(VideoControls)]
fn controls(props: &VideoControlProps) -> Html {
    let text_to_search = use_state(|| String::new());

    let handle_input = {
        let text_to_search = text_to_search.clone();
        Callback::from(move |input_event| {
            let text = get_value_from_input_event(input_event);
            text_to_search.set(text);
        })
    };

    let on_search_pressed = {
        let on_search = props.on_search.clone();
        Callback::from(move |_| on_search.emit(text_to_search.to_string()))
    };

    html! {
        <div>
            <div>
                {"Ingresa una palabra"}
            </div>
            <div>
                <input type="text" oninput={handle_input} />
            </div>
            <div><button onclick={on_search_pressed}>{"buscar!"}</button></div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_section(props: &VideoSectionProps) -> Html {
    let yt_url = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <div>
            <iframe width="560" height="315" src={yt_url}></iframe>
        </div>
    }
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
