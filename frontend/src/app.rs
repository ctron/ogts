use crate::{
    about,
    pages::{self, AppRoute},
};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(Application)]
pub fn application() -> Html {
    let logo = html! (
        <Brand src="images/logo.png" alt="OGTS" />
    );

    let sidebar = html_nested!(
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavExpandable title="Home">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{ "Overview" }</NavRouterItem<AppRoute>>
                    </NavExpandable>
                    <NavExpandable title="Main">
                        <NavRouterItem<AppRoute> to={AppRoute::Child}>{ "Children" }</NavRouterItem<AppRoute>>
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    );

    let callback_docs = Callback::from(|_| {
        spawn_local(async {
            let _ = tauri_sys::shell::open("https://dentrassi.de").await;
        })
    });

    let backdropper = use_backdrop();

    let callback_about = Callback::from(move |_| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<about::About/>));
        }
    });

    let tools = html!(
        <Toolbar>
            <ToolbarItem>
                <AppLauncher
                    position={Position::Right}
                    toggle={Icon::QuestionCircle}
                >
                    <AppLauncherItem onclick={callback_docs}>{ "Documentation" }</AppLauncherItem>
                    <AppLauncherItem onclick={callback_about}>{ "About" }</AppLauncherItem>
                </AppLauncher>
            </ToolbarItem>
        </Toolbar>
    );

    html!(
        <Router<AppRoute>>
            <Page {logo} {sidebar} {tools}>
                <RouterSwitch<AppRoute> {render}/>
            </Page>
        </Router<AppRoute>>
    )
}

fn render(route: AppRoute) -> Html {
    log::info!("Route: {route:?}");
    match route {
        AppRoute::Index => html!(<pages::Index/>),
        AppRoute::Child => html!(<pages::ChildList/>),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            name2,
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                </a>
            </div>

            <p>{"Click on the Tauri and Yew logos to learn more."}</p>

            <p>
                {"Recommended IDE setup: "}
                <a href="https://code.visualstudio.com/" target="_blank">{"VS Code"}</a>
                {" + "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{"Tauri"}</a>
                {" + "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{"rust-analyzer"}</a>
            </p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
