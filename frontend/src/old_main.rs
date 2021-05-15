#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use console_error_panic_hook::set_once as set_panic_hook;

use serde::Deserialize;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::reader::{File, FileChunk, FileData, ReaderService, ReaderTask};
use yew::services::ConsoleService;
use yew::{
    format::{Json, Nothing},
    prelude::*,
};
use yew::{html, ChangeData, Component, ComponentLink, Html, ShouldRender};
type Chunks = bool;

pub enum Msg {
    Loaded(FileData),
    Chunk(Option<FileChunk>),
    Files(Vec<File>, Chunks),
    ToggleByChunks,
}

pub struct Model {
    link: ComponentLink<Model>,
    tasks: Vec<ReaderTask>,
    files: Vec<String>,
    by_chunks: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            tasks: vec![],
            files: vec![],
            by_chunks: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Loaded(file) => {
                let info = format!("file: {:?}", file);
                self.files.push(info);
                true
            }
            Msg::Chunk(Some(chunk)) => {
                let info = format!("chunk: {:?}", chunk);
                self.files.push(info);
                true
            }
            Msg::Files(files, chunks) => {
                let mut reader_service = ReaderService::new();
                for file in files.into_iter() {
                    let task = {
                        if chunks {
                            let callback = self.link.callback(Msg::Chunk);
                            reader_service
                                .read_file_by_chunks(file, callback, 100)
                                .unwrap()
                        } else {
                            let callback = self.link.callback(Msg::Loaded);
                            reader_service.read_file(file, callback).unwrap()
                        }
                    };
                    self.tasks.push(task);
                }
                true
            }
            Msg::ToggleByChunks => {
                self.by_chunks = !self.by_chunks;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let flag = self.by_chunks;
        html! {
            <div>
                <div>
                    <h1>{ "Choose a file to upload to see the uploaded bytes" }</h1>
                    <input type="file" multiple=true onchange=self.link.callback(move |value| {
                            let mut result = Vec::new();
                            if let ChangeData::Files(files) = value {
                                let files = js_sys::try_iter(&files)
                                    .unwrap()
                                    .unwrap()
                                    .map(|v| File::from(v.unwrap()));
                                result.extend(files);
                            }
                            Msg::Files(result, flag)
                        })
                    />
                </div>
                <div>
                    <label>{ "By chunks" }</label>
                    <input type="checkbox" checked=flag onclick=self.link.callback(|_| Msg::ToggleByChunks) />
                </div>
                <ul>
                    { for self.files.iter().map(|f| Self::view_file(f)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_file(data: &str) -> Html {
        html! {
            <li>{ data }</li>
        }
    }
}
struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <ybc::Navbar
                classes="is-info"
                padded=true
                navbrand=html!{
                    <ybc::NavbarItem>
                        <ybc::Title classes="has-text-white" size=ybc::HeaderSize::Is4>{"Trunk | Yew | YBC"}</ybc::Title>
                    </ybc::NavbarItem>
                }
                navstart=html!{}
                navend=html!{
                    <>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes="is-black is-outlined" rel="noopener noreferrer" target="_blank" href="https://github.com/thedodd/trunk">
                            {"Trunk"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes="is-black is-outlined" rel="noopener noreferrer" target="_blank" href="https://yew.rs">
                            {"Yew"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor classes="is-black is-outlined" rel="noopener noreferrer" target="_blank" href="https://github.com/thedodd/ybc">
                            {"YBC"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    </>
                }
            />

            <ybc::Hero
                classes="is-light"
                size=ybc::HeroSize::FullheightWithNavbar
                body=html!{
                    <ybc::Container classes="is-centered">
                    <ybc::Tile ctx=Ancestor>
                        <ybc::Tile ctx=Parent size=ybc::TileSize::Twelve>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-success">
                                    <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"Trunk"}</ybc::Subtitle>
                                    <p>{"Trunk is a WASM web application bundler for Rust."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-success">
                                    <ybc::Icon size=ybc::Size::Large classes="is-pulled-right"><img src="yew.svg"/></ybc::Icon>
                                    <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">
                                        {"Yew"}
                                    </ybc::Subtitle>
                                    <p>{"Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                            <ybc::Tile ctx=Parent>
                                <ybc::Tile ctx=Child classes="notification is-success">
                                    <Model> </Model>
                                    // <ybc::Subtitle size=ybc::HeaderSize::Is3 classes="has-text-white">{"YBC"}</ybc::Subtitle>
                                    // <p>{"A Yew component library based on the Bulma CSS framework."}</p>
                                </ybc::Tile>
                            </ybc::Tile>
                        </ybc::Tile>
                    </ybc::Tile>
                    </ybc::Container>
                }>
            </ybc::Hero>
            </>
        }
    }
}

fn main() {
    set_panic_hook();
    ConsoleService::log("hello");
    // Show off some feature flag enabling patterns.
    #[cfg(feature = "demo-abc")]
    {
        ConsoleService::log("feature `demo-abc` enabled");
    }
    #[cfg(feature = "demo-xyz")]
    {
        ConsoleService::log("feature `demo-xyz` enabled");
    }

    yew::start_app::<App>();
}
