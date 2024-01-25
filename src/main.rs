pub mod navigate;
pub mod footer;
pub mod svgbutton;
pub mod tooling;
pub mod landing;
pub mod workflow;
pub mod dashboard;
pub mod events;
pub mod people;
pub mod getaccess;
pub mod accessgpts;
pub mod customwork;

use crate::navigate::Navigate;
use crate::landing::Landing;
use crate::dashboard::Dashboard;
use crate::tooling::Tooling;
use crate::workflow::Workflow;
use crate::events::Events;
use crate::people::People;
use crate::getaccess::GetAccess;
use crate::customwork::CustomWork;

use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="bg-transparent app flex">
            <aside class="bg-transparent sticky top-0 left-0">
                <Navigate />
            </aside>
            <main class="bg-transparent pl-5">
                <Router>
                    <Routes>
                        <Route path="/" view=Landing />
                        <Route path="/dashboard" view=Dashboard />
                        <Route path="/tooling" view=Tooling />
                        <Route path="/workflow" view=Workflow />
                        <Route path="/events" view=Events />
                        <Route path="/people" view=People />
                        <Route path="/getaccess" view=GetAccess />
                        <Route path="/customwork" view=CustomWork />
                    </Routes>
                </Router>
            </main>
        </div>
    }
}
