use log::*;
use yew::format::Json;
use yew::prelude::*;
use yew_router::{prelude::*, Switch};

pub struct App {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/art"]
    Art,
    #[to = "/apps"]
    Apps,
    #[to = "/Mods"]
    Mods,
    #[to = "/"]
    Home,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <><nav>
                <div class="nav-menu">
                    <RouterAnchor<AppRoute> route=AppRoute::Home/* style="text-decoration: underline"*/>{"Home"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Art>{"3D"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Apps>{"Apps"}</RouterAnchor<AppRoute>>
                    <RouterAnchor<AppRoute> route=AppRoute::Mods>{"Mods"}</RouterAnchor<AppRoute>>
            //<!--      <a href="index.html">How-Tos</a>-->
            //<!--      <a href="index.html">Photography</a>-->
                </div>
                <div style="text-align: center">
                    <RouterAnchor<AppRoute> route=AppRoute::Home><img src="icon.png" alt="FabianLars' Logo" style="width:100px" /></RouterAnchor<AppRoute>>
                </div>
                <div class="nav-outbound" style="text-align: right">
                    <a href="https://github.com/FabianLars"><img src="img/GitHub-Mark-Light-32px.png" alt="GitHub Logo" /></a>
                </div>
            </nav>
            <div>
                <Router<AppRoute>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html! {"Home"},
                            AppRoute::Art => html! {"Home"},
                            AppRoute::Apps => html! {"Home"},
                            AppRoute::Mods => html! {"Home"},
                        }
                    })
                    redirect = Router::redirect(|_| {
                        AppRoute::Home
                    })
                />
            </div></>
        }
    }
}