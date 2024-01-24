pub mod navigate;
pub mod footer;
pub mod svgbutton;
pub mod tooling;
pub mod landing;
pub mod workflow;
pub mod dashboard;
pub mod events;
pub mod people;

use crate::navigate::Navigate;
use crate::footer::Footer;
use crate::landing::Landing;
use crate::dashboard::Dashboard;
use crate::tooling::Tooling;
use crate::workflow::Workflow;
use crate::events::Events;
use crate::people::People;

use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-tl from-black via-blue-500 to-purple-900 app flex">
            <aside class="h-screen sticky top-0 left-0 bottom-0">
                <Navigate />
            </aside>
            <main class="pl-5">
                <Router>
                    <Routes>
                        <Route path="/" view=Landing />
                        <Route path="/dashboard" view=Dashboard />
                        <Route path="/tooling" view=Tooling />
                        <Route path="/workflow" view=Workflow />
                        <Route path="/events" view=Events />
                        <Route path="/people" view=People />
                    </Routes>
                </Router>
            </main>
        </div>
        <div class="absolute bottom-0 w-full">
            <Footer />
        </div>
    }
}
