module Main exposing (main)

import Browser
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Url


-- MAIN


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- MODEL


type alias Model =
    { key : Nav.Key
    , url : Url.Url
    , page : Page
    }


type Page
    = Home
    | Cognition
    | Problems
    | Platform
    | Sage
    | Trust
    | Team
    | Roadmap


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url key =
    ( { key = key
      , url = url
      , page = urlToPage url
      }
    , Cmd.none
    )


urlToPage : Url.Url -> Page
urlToPage url =
    case url.fragment of
        Just "cognition" ->
            Cognition

        Just "problems" ->
            Problems

        Just "platform" ->
            Platform

        Just "sage" ->
            Sage

        Just "trust" ->
            Trust

        Just "team" ->
            Team

        Just "roadmap" ->
            Roadmap

        _ ->
            Home



-- UPDATE


type Msg
    = LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url
    | NavigateTo Page


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key (Url.toString url) )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            ( { model | url = url, page = urlToPage url }
            , Cmd.none
            )

        NavigateTo page ->
            let
                fragment =
                    case page of
                        Home ->
                            ""

                        Cognition ->
                            "#cognition"

                        Problems ->
                            "#problems"

                        Platform ->
                            "#platform"

                        Sage ->
                            "#sage"

                        Trust ->
                            "#trust"

                        Team ->
                            "#team"

                        Roadmap ->
                            "#roadmap"
            in
            ( model, Nav.pushUrl model.key fragment )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    { title = "Cowboy AI - Cognitive Orchestration Platform"
    , body =
        [ div [ class "app" ]
            [ viewNavbar model.page
            , viewContent model.page
            , viewFooter
            ]
        ]
    }


viewNavbar : Page -> Html Msg
viewNavbar currentPage =
    nav [ class "navbar" ]
        [ div [ class "nav-container" ]
            [ div [ class "nav-logo" ] [ text "Cowboy AI" ]
            , ul [ class "nav-menu" ]
                [ viewNavItem "Home" Home currentPage
                , viewNavItem "Cognition" Cognition currentPage
                , viewNavItem "AI at Scale" Problems currentPage
                , viewNavItem "Platform" Platform currentPage
                , viewNavItem "SAGE" Sage currentPage
                , viewNavItem "Security" Trust currentPage
                , viewNavItem "Team" Team currentPage
                , viewNavItem "Roadmap" Roadmap currentPage
                ]
            ]
        ]


viewNavItem : String -> Page -> Page -> Html Msg
viewNavItem label page currentPage =
    li []
        [ a
            [ href "#"
            , onClick (NavigateTo page)
            , classList [ ( "active", page == currentPage ) ]
            ]
            [ text label ]
        ]


viewContent : Page -> Html Msg
viewContent page =
    case page of
        Home ->
            viewHome

        Cognition ->
            viewCognition

        Problems ->
            viewProblems

        Platform ->
            viewPlatform

        Sage ->
            viewSage

        Trust ->
            viewTrust

        Team ->
            viewTeam

        Roadmap ->
            viewRoadmap


viewHome : Html Msg
viewHome =
    section [ id "home", class "hero" ]
        [ div [ class "container" ]
            [ h1 [ class "hero-title" ] [ text "Cowboy AI" ]
            , h2 [ class "hero-subtitle" ] 
                [ text "The Platform For Composable, Cognitive, Audit-Grade AI Swarms" ]
            , p [ class "hero-tagline" ] 
                [ text "Your New Business Brain & Nervous System. Cognition. Evolution. Security. Trust." ]
            , p [ class "hero-description" ] 
                [ text "No-Code Composable Multi-AI Agent Orchestration At Scale" ]
            , div [ class "warning-box" ]
                [ h3 [] [ text "Disclaimer & Warning:" ]
                , p [] [ text "You can now ask and command the system to do anything. In plain English. And it will." ]
                ]
            ]
        ]


viewCognition : Html Msg
viewCognition =
    section [ id "cognition", class "section" ]
        [ div [ class "container" ]
            [ h2 [] [ text "No Cognition? - No AGI!" ]
            , p [ class "subtitle" ] 
                [ text "Intelligence Alone Isn't Enough—Cognition Is the Missing Piece." ]
            , div [ class "quadrants" ]
                [ viewQuadrant "1" "Hardware" "Horsepower" "green"
                , viewQuadrant "2" "LLM" "Raw tooling" "green"
                , viewQuadrant "4" "Your World - The Data" "Clean, Holistic, Accessible, Mathematically Proven" "blue"
                , viewQuadrant "3" "Cognition" "Awareness of tools & State" "blue"
                ]
            , div [ class "content-box" ]
                [ p [] 
                    [ text "For true AGI, systems must "
                    , strong [] [ text "learn, adapt, and evolve" ]
                    , text "—not just process data."
                    ]
                , p [] [ text "Today's AI is trapped in silos, built on imperfect math, with no structure or guardrails." ]
                , p [ class "emphasis" ] [ text "That's why it keeps failing." ]
                , h3 [] [ text "We solved it." ]
                , p [] 
                    [ text "By embedding "
                    , strong [] [ text "cognition at the core" ]
                    , text ", Cowboy AI creates an adaptive, composable architecture that continuously learns, evolves, and safeguards decision-making."
                    ]
                , p [ class "highlight" ] [ text "This is the breakthrough that makes AGI possible." ]
                ]
            ]
        ]


viewQuadrant : String -> String -> String -> String -> Html Msg
viewQuadrant number title description colorClass =
    div [ class ("quadrant " ++ colorClass) ]
        [ span [ class "number" ] [ text number ]
        , h3 [] [ text title ]
        , p [] [ text description ]
        ]


viewProblems : Html Msg
viewProblems =
    section [ id "problems", class "section dark" ]
        [ div [ class "container" ]
            [ h2 [] [ text "AI Is Breaking at Scale" ]
            , div [ class "problem-cards" ]
                [ viewProblemCard "Fragmentation kills velocity" 
                    "Businesses juggle many applications & tools, re-enter data, and never achieve true sync."
                , viewProblemCard "The last 10% explodes compute" 
                    "Teams reach 85-90% automation, then costs and fragility spike without the right structures."
                , viewProblemCard "Lock-in & opacity" 
                    "Cloud-centric AI → runaway spend, brittle workflows, and audits no one can pass."
                ]
            , h3 [] [ text "Root Cause vs Symptom" ]
            , table [ class "comparison-table" ]
                [ thead []
                    [ tr []
                        [ th [] [ text "Root Cause" ]
                        , th [] [ text "Symptom" ]
                        ]
                    ]
                , tbody []
                    [ viewComparisonRow "Fragmented & proprietary stacks" 
                        "Siloed data → duplication waste and a hard throughput ceiling."
                    , viewComparisonRow "Vendor lock-in & cloud-only ops" 
                        "Exploding costs at scale; switching costs paralyze change."
                    , viewComparisonRow "Brittle translations/APIs" 
                        "Scale-load failures; throttling/egress fees bleed budgets."
                    , viewComparisonRow "Imperfect-math / nondeterministic code paths" 
                        "Inconsistent outputs; expensive debugging caps reliability."
                    , viewComparisonRow "Weak governance & no single source of truth" 
                        "Untraceable decisions; audit exposure inflates compliance spend."
                    , viewComparisonRow "Scale threshold (\"last 10%\")" 
                        "Compute spikes and instability; ROI collapses at ~85-90% automation."
                    ]
                ]
            ]
        ]


viewProblemCard : String -> String -> Html Msg
viewProblemCard title description =
    div [ class "card" ]
        [ h3 [] [ text title ]
        , p [] [ text description ]
        ]


viewComparisonRow : String -> String -> Html Msg
viewComparisonRow cause symptom =
    tr []
        [ td [] [ text cause ]
        , td [] [ text symptom ]
        ]


viewPlatform : Html Msg
viewPlatform =
    section [ id "platform", class "section" ]
        [ div [ class "container" ]
            [ h2 [] [ text "Platform First — Cowboy AI CIM" ]
            , p [ class "subtitle" ] 
                [ text "The platform for building, governing, and scaling your business with AI swarms." ]
            , div [ class "feature-grid" ]
                [ viewFeatureCard "📊" "Build Once, Replicate Anywhere" 
                    [ "We're proving it in private lending/fintech first."
                    , "Repeatable & Reproduceable."
                    ]
                , viewFeatureCard "🔧" "Open for Builders" 
                    [ "Publish domain packs, swarms, and connectors other teams can adopt." ]
                , viewFeatureCard "📈" "Proven Milestones" 
                    [ "8M+ micro-transactions processed; hybrid runtime targeting >90% lower AI compute vs cloud-only baselines." ]
                , viewFeatureCard "🧩" "No-code, modular agent swarms" 
                    [ "Like little building blocks AI agents assemble and compose on the fly to execute any task." ]
                ]
            , div [ class "security-banner" ]
                [ span [ class "icon" ] [ text "🔒" ]
                , h3 [] [ text "Immutable, Security First" ]
                , p [] [ text "Immutable records, ransomware-resistant architecture" ]
                ]
            ]
        ]


viewFeatureCard : String -> String -> List String -> Html Msg
viewFeatureCard icon title descriptions =
    div [ class "feature-card" ]
        [ span [ class "icon" ] [ text icon ]
        , h3 [] [ text title ]
        , div [] (List.map (\desc -> p [] [ text desc ]) descriptions)
        ]


viewSage : Html Msg
viewSage =
    section [ id "sage", class "section dark" ]
        [ div [ class "container" ]
            [ h2 [] [ text "SAGE" ]
            , h3 [] [ text "Universal Development Intelligence Platform" ]
            , p [ class "subtitle" ] 
                [ text "Orchestrating 100+ Specialized AI Experts from Silicon to Solution" ]
            , div [ class "sage-features" ]
                [ viewSageCard "🎯 Complete Stack Mastery"
                    [ "Nix Expert: Kernel to cloud expertise with deterministic, reproducible builds"
                    , "Resource Expert: Real-world aware - optimizes for actual memory, CPU, bandwidth constraints"
                    , "Domain & DDD Experts: Extract business essence through Event Storming and mathematical boundaries"
                    , "Infrastructure Experts: Network topology, NATS messaging, distributed systems architecture"
                    ]
                , viewSageCard "🔬 Mathematical Foundation"
                    [ "Provable Correctness: Category Theory and Graph Theory backed decisions"
                    , "CID Chain Coherence: Cryptographically linked, immutable audit trail via IPLD"
                    , "Verified State Transitions: Mathematically guaranteed system invariants"
                    , "Algebraic Message Routing: Subject hierarchies optimized with mathematical precision"
                    ]
                , viewSageCard "🌐 Distributed Intelligence"
                    [ "Zero Single Points of Failure: NATS JetStream based central nervous system"
                    , "Auto-Healing Architecture: Lose a node or datacenter - system adapts instantly"
                    , "Multi-Agent Orchestration: Symphonic coordination of specialized expertise"
                    , "Event-Sourced Truth: Every decision recorded, every state reversible"
                    ]
                , viewSageCard "🚀 Development Acceleration"
                    [ "Proactive Guidance: Anticipates needs, prevents architectural mistakes"
                    , "Living Documentation: BDD/TDD experts ensure specs, tests, and docs stay synchronized"
                    , "Compliance by Design: HIPAA, PCI DSS, GDPR built-in, not bolted-on"
                    , "Context Preservation: Seamless conversations across sessions and agents"
                    ]
                ]
            , div [ class "sage-difference" ]
                [ h3 [] [ text "🧠 The SAGE Difference" ]
                , ul []
                    [ li [] [ text "100+ Specialized AI Agent Experts scaling to billions" ]
                    , li [] [ text "Mathematical Proofs not promises" ]
                    , li [] [ text "Distributed Resilience not single points of failure" ]
                    , li [] [ text "Orchestrated Intelligence not isolated tools. Like small Lego Blocks" ]
                    , li [] [ text "Domain Mastery not generic solutions" ]
                    ]
                , p [ class "tagline" ] 
                    [ text "Transform your development with orchestrated intelligence."
                    , br [] []
                    , text "Deploy with confidence. Scale with certainty. Evolve with intelligence."
                    ]
                ]
            ]
        ]


viewSageCard : String -> List String -> Html Msg
viewSageCard title items =
    div [ class "sage-card" ]
        [ h4 [] [ text title ]
        , ul [] (List.map (\item -> li [] [ text item ]) items)
        ]


viewTrust : Html Msg
viewTrust =
    section [ id "trust", class "section" ]
        [ div [ class "container" ]
            [ h2 [] [ text "Trust by Design:" ]
            , h3 [] [ text "Security, Control, and Explainability at the Core" ]
            , p [ class "subtitle" ] 
                [ text "Cowboy AI delivers secure-by-default automation through a modern, composable architecture engineered for financial-grade integrity and adaptability—on your terms." ]
            , div [ class "trust-grid" ]
                [ viewTrustCard "🏗️ Infrastructure of Integrity"
                    [ "Apple M3 Ultra + NixDarwin for deterministic environments"
                    , "Rust + Bevy Runtime for memory-safe orchestration"
                    , "NATS Messaging for real-time, fault-tolerant communication"
                    ]
                , viewTrustCard "🔐 Security That Scales"
                    [ "YubiKey Hardware Auth + ECC Encryption for transaction-level trust"
                    , "MinIO + Wasabi providing immutable, ransomware-resistant storage"
                    , "AI Model Agnostic -Compatible for privacy-preserving AI deployment, local and cloud"
                    ]
                , viewTrustCard "🧠 Explainable Intelligence"
                    [ "Neo4j Graph Engine + Event Sourcing for auditable decisions"
                    , "Functional Programming + TDD ensuring provable correctness"
                    , "DDD + Conceptual Spaces aligning systems to business intent"
                    ]
                , viewTrustCard "📐 Mathematics of Trust"
                    [ "Category Theory & Graph Theory ensuring formal correctness"
                    , "Gradient Descent Programming for optimized agent decisions"
                    , "Trust proven line by line, decision by decision"
                    ]
                ]
            ]
        ]


viewTrustCard : String -> List String -> Html Msg
viewTrustCard title items =
    div [ class "trust-card" ]
        [ h4 [] [ text title ]
        , ul [] (List.map (\item -> li [] [ text item ]) items)
        ]


viewTeam : Html Msg
viewTeam =
    section [ id "team", class "section dark" ]
        [ div [ class "container" ]
            [ h2 [] [ text "Our Leadership Team" ]
            , div [ class "team-grid" ]
                [ viewTeamMember "Jacob Kopilovitch" "CEO" 
                    "Serial entrepreneur with over 20 years of experience driving strategic vision, business growth, and organizational leadership across multiple industries."
                , viewTeamMember "David Kopilovitch" "COO" 
                    "Operations executive with 17+ years of expertise in back office management, process optimization, and detailed execution of complex business operations."
                , viewTeamMember "Steele Price" "CSO" 
                    "Technology visionary with 40+ years of experience, including 10 years as Microsoft MVP and core team member for .NET language development."
                , viewTeamMember "Kathryn Freeman" "CBO" 
                    "Accomplished entrepreneur and senior advisor with 17+ years guiding fintech and lending companies."
                , viewTeamMember "Mark Sonnenklar" "CLO" 
                    "Premier business and intellectual property attorney with 25+ years of experience in corporate and private law."
                , viewTeamMember "Ryan Plemons" "Senior Architect & AI Lead" 
                    "Veteran technology leader with 30+ years as a senior engineer, specializing in AI implementation."
                ]
            ]
        ]


viewTeamMember : String -> String -> String -> Html Msg
viewTeamMember name role bio =
    div [ class "team-member" ]
        [ h3 [] [ text name ]
        , h4 [] [ text role ]
        , p [] [ text bio ]
        ]


viewRoadmap : Html Msg
viewRoadmap =
    section [ id "roadmap", class "section" ]
        [ div [ class "container" ]
            [ h2 [] [ text "Roadmap" ]
            , div [ class "roadmap-timeline" ]
                [ div [ class "roadmap-phase" ]
                    [ h3 [] [ text "Next 6-12 months: Building the Foundation" ]
                    , ul []
                        [ li [] [ text "Scale Private Lending Market Expansion" ]
                        , li [] [ text "Launch Partner Program" ]
                        , li [] [ text "Expand into adjacent FinTech operations" ]
                        ]
                    ]
                , div [ class "roadmap-phase" ]
                    [ h3 [] [ text "12-18 months: Scaling and Expansion" ]
                    , ul []
                        [ li [] [ text "Secure Second Funding Round & Cultivate Strategic Vertical Partners" ]
                        , li [] [ text "Expand Into Future Verticals: M&A, Robotics, Legal, Supply Chain" ]
                        , li [] [ text "Curate Developer Marketplace" ]
                        ]
                    ]
                ]
            , p [ class "roadmap-footer" ] 
                [ text "Each milestone strengthens our position to dominate the AI orchestration market with our significant head start." ]
            ]
        ]


viewFooter : Html Msg
viewFooter =
    footer [ class "footer" ]
        [ div [ class "container" ]
            [ p [] [ text "© 2024 Cowboy AI. All rights reserved." ]
            ]
        ]