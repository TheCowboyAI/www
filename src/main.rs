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
pub mod naturallanguage;
pub mod askquestions;
pub mod codegeneration;
pub mod existingapps;
pub mod newfunctionality;
pub mod cim;
pub mod cimsvg;
pub mod audittrail;
pub mod historicalinsight;
pub mod projections;
pub mod eventstreams;
pub mod predictions;
pub mod realtimemonitoring;
pub mod pushtomobile;
pub mod decisionmaking;
pub mod communications;
pub mod security;
pub mod visualize;
pub mod collaborate;
pub mod ubiquitouslanguage;
pub mod customersupport;
pub mod customgpts;
pub mod workshops;
pub mod workshopform;
pub mod nixoswork;
pub mod aiintegration;
pub mod landingplay;

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
use crate::naturallanguage::NaturalLanguage;
use crate::askquestions::AskQuestions;
use crate::codegeneration::CodeGeneration;
use crate::existingapps::ExistingApps;
use crate::newfunctionality::NewFunctionality;
use crate::cim::Cim;
use crate::audittrail::AuditTrail;
use crate::historicalinsight::HistoricalInsight;
use crate::projections::Projections;
use crate::eventstreams::EventStreams;
use crate::predictions::Predictions;
use crate::realtimemonitoring::RealTimeMonitoring;
use crate::pushtomobile::PushToMobile;
use crate::decisionmaking::DecisionMaking;
use crate::communications::Communications;
use crate::security::Security;
use crate::visualize::Visualize;
use crate::collaborate::Collaborate;
use crate::ubiquitouslanguage::UbiquitousLanguage;
use crate::customersupport::CustomerSupport;
use crate::customgpts::CustomGPTs;
use crate::workshops::Workshops;
use crate::workshopform::WorkshopForm;
use crate::nixoswork::NixOSWork;
use crate::aiintegration::AIIntegration;
use crate::landingplay::Landingplay;

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
                        <Route path="/aiintegration" view=AIIntegration />
                        <Route path="/eventsourcing" view=EventSourcing />
                        <Route path="/modeledintelligence" view=ModeledIntelligence />
                        <Route path="/gametheory" view=GameTheory />
                        <Route path="/generatecontent" view=GenerateContent />
                        <Route path="/context" view=Context />
                        <Route path="/naturallanguage" view=NaturalLanguage />
                        <Route path="/deterministic" view=Deterministic />
                        <Route path="/askquestions" view=AskQuestions />
                        <Route path="/codegeneration" view=CodeGeneration />
                        <Route path="/existingapps" view=ExistingApps />
                        <Route path="/newfunctionality" view=NewFunctionality />
                        <Route path="/cim" view=Cim />
                        <Route path="/audittrail" view=AuditTrail />
                        <Route path="/historicalinsight" view=HistoricalInsight />
                        <Route path="/projections" view=Projections />
                        <Route path="/eventstreams" view=EventStreams />
                        <Route path="/predictions" view=Predictions />
                        <Route path="/realtimemonitoring" view=RealTimeMonitoring />
                        <Route path="/pushtomobile" view=PushToMobile />
                        <Route path="/decisionmaking" view=DecisionMaking />
                        <Route path="/communications" view=Communications />
                        <Route path="/security" view=Security />
                        <Route path="/visualize" view=Visualize />
                        <Route path="/collaborate" view=Collaborate />
                        <Route path="/ubiquitouslanguage" view=UbiquitousLanguage />
                        <Route path="/customersupport" view=CustomerSupport />
                        <Route path="/customgpts" view=CustomGPTs />
                        <Route path="/workshops" view=Workshops />
                        <Route path="/workshopform" view=WorkshopForm />
                        <Route path="/nixoswork" view=NixOSWork />
                        <Route path="/*any" view=Landing />
                        <Route path="/landingplay" view=LandingPlay />
                    </Routes>
                </Router>
            </main>
        </div>
    }
}
