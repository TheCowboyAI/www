pub mod navigate;
pub mod footer;
pub mod svgbutton;
pub mod tooling;
pub mod landing;
pub mod workflow;
pub mod dashboard;
pub mod events;
pub mod people;
pub mod ourgpts;
pub mod solutions;
pub mod workform;
pub mod gptaccessform;
pub mod businessmodel;
pub mod businessevolution;
pub mod simplewords;
pub mod aicomposable;
pub mod eventsourcing;
pub mod modeledintelligence;
pub mod gametheory;
pub mod context;
pub mod generatecontent;
pub mod deterministic;

use crate::navigate::Navigate;
use crate::landing::Landing;
use crate::dashboard::Dashboard;
use crate::tooling::Tooling;
use crate::workflow::Workflow;
use crate::events::Events;
use crate::people::People;
use crate::solutions::Solutions;
use crate::ourgpts::OurGPTs;
use crate::businessmodel::BusinessModel;
use crate::businessevolution::BusinessEvolution;
use crate::simplewords::SimpleWords;
use crate::aicomposable::AIComposable;
use crate::eventsourcing::EventSourcing;
use crate::modeledintelligence::ModeledIntelligence;
use crate::gametheory::GameTheory;
use crate::context::Context;
use crate::generatecontent::GenerateContent;
use crate::deterministic::Deterministic;

use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app flex min-width-100">
            <aside class="sticky top-0 left-0">
                <Navigate />
            </aside>
            <main class="pl-5">
                <Router>
                    <Routes>
                        <Route path="/" view=Landing />
                        <Route path="/tooling" view=Tooling />
                        <Route path="/dashboard" view=Dashboard />
                        <Route path="/workflow" view=Workflow />
                        <Route path="/events" view=Events />
                        <Route path="/people" view=People />
                        <Route path="/gpts" view=OurGPTs />
                        <Route path="/customwork" view=Solutions />
                        <Route path="/solutions" view=Solutions />
                        <Route path="/businessmodel" view=BusinessModel />
                        <Route path="/simplewords" view=SimpleWords />
                        <Route path="/businessevolution" view=BusinessEvolution />
                        <Route path="/aicomposable" view=AIComposable />
                        <Route path="/eventsourcing" view=EventSourcing />
                        <Route path="/modeledintelligence" view=ModeledIntelligence />
                        <Route path="/gametheory" view=GameTheory />
                        <Route path="/generatecontent" view=GenerateContent />
                        <Route path="/context" view=Context />
                        <Route path="/deterministic" view=Deterministic />
                        <Route path="/*any" view=Landing />
                    </Routes>
                </Router>
            </main>
        </div>
    }
}
