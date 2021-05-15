// requires the serde and anyhow crates

use serde::Deserialize;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{
    format::{Json, Nothing},
    prelude::*,
};

// #[derive(Deserialize, Debug, Clone)]
// pub struct ISSPosition {
//     latitude: String,
//     longitude: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// pub struct ISS {
//     message: String,
//     timestamp: i32,
//     iss_position: ISSPosition,
// }
use shared::{ApiAIModel, ModelsApiResponse};

#[derive(Debug)]
pub enum Msg {
    GetListModels,
    ReceiveResponse(Result<ModelsApiResponse, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    models: Option<ModelsApiResponse>,
    link: ComponentLink<Self>,
    error: Option<String>,
}
/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_iss_location(&self) -> Html {
        match self.models {
            Some(ref model_list) => {
                html! {
                    <>
                        <p>{ "The List of available model is:" }</p>
                        // <p>{ format!("Latitude: {}", model_list.data.iter().next().unwrap().model_name) }</p>
                        <ul>{ for model_list.data.iter().map(|f| Self::view_file(f)) }</ul>
                    </>
                }
            }
            None => {
                html! {
                     <button onclick=self.link.callback(|_| Msg::GetListModels)>
                         { "What model are available?" }
                     </button>
                }
            }
        }
    }
    fn view_file(data: &ApiAIModel) -> Html {
        html! {
            <li>{ format!("Latitude: {:?}",data) }</li>
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            models: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetListModels => {
                // 1. build the request
                let request = Request::get("http://127.0.0.1:8000/api/v1/admin/models/list")
                    .body(Nothing)
                    .expect("Could not build request.");
                // 2. construct a callback
                let callback = self.link.callback(
                    |response: Response<Json<Result<ModelsApiResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                // 3. pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(location) => {
                        self.models = Some(location);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_iss_location() }
                { self.view_error() }
            </>
        }
    }
}

fn main() {
    // set_panic_hook();
    // ConsoleService::log("hello");
    yew::start_app::<FetchServiceExample>();
}
