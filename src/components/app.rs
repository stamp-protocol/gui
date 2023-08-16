use crate::{
    components::{
        header::{Header, HeaderAction},
        pages::{
            error::Error,
            main::Main,
        },
        util::page::Page,
    },
    models::{
        darkmode,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    let dark = darkmode::init();
    provide_context(dark);
    view! {
        <div id="root">
            <Title text="get a job" />
            <Router>
                <main class="absolute z-10 top-0 bottom-0 left-0 right-0">
                    <Routes>
                        <Route path="/"
                            view=|| { view! {
                                <Header title="WELCOME TO STAMP!!".into()>
                                    <HeaderAction
                                        id="settings".into()
                                        title="Settings".into()
                                        icon="gear"
                                        href="/--/settings" />
                                </Header>
                                <Page>
                                    <Main/>
                                </Page>
                                <Outlet/>
                            } }
                        >
                            <Route path="" view=|| { view! { "" } } />
                            <Route path="--/settings"
                                view=|| { view! {
                                    <Header
                                        title="SETTINGS".into()
                                        back_url="/" />
                                    <Page>
                                        "hi settings here..."
                                    </Page>
                                } }
                            />
                        </Route>
                        <Route path="/error"
                            view=|| { view! {
                                <Error/>
                            } }
                        />
                        <Route path="/*any"
                            view=|| { view! {
                                <Error title="Page not found".into() desc=format!("The app has routed you to an unknown location: {}", use_location().pathname.get()) />
                            } }
                        />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

