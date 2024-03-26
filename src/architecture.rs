use leptos::*;

#[component]
pub fn Architecture() -> impl IntoView {
    view! {
      <div class="mt-5 flex flex-col items-center animate-fadeIn">
        <div class="theme_bg-1-bg-3 rounded-xl p-3 shadow-xl shadow-black">
            <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl" id="architecture">{"Architecture"}</h1>
            <p class="mt-2">{"Cowboy AI uses the concept of a CIM, a Composable Information Machine."}</p>
            <p class="mt-2">{"The Composable Information Machine relies on some dependencies, which have a fitness function for replacement."}</p>
            <p class="mt-2">{"Primarily the CIM is driven by Word Lists, Messages and Relationships. We need a way to make all these communicate and generate a working system, for that we use industry proven tools."}</p>
            <p class="mt-2">{"These dependencies enable us to work with an infinite range of connections to a central nervous system."}</p>
            <p class="mt-2"><img src="./assets/CIMArchitecture.svg" alt="Architecture" /></p>
            <p class="mt-2">{"We control the central nervous system, tools enable it to function. We favor Open Source software we control, as to not be vendor-locked into a business decision we are forced to perform. If you prefer a different tool in these positions, they are adaptable, but heavily opinionated in this current version of CIM."} </p>
            <p class="mt-3 mb-5 font-bold">{"For the latest developments on the CIM Architecture see: "}<a href="http://www.cimlabs.org/technology.html">{"CIM Labs"}</a></p>
        </div>

        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="benthos">
                <img src="assets/logo_hero.svg" class="w-16 h-16" />
                {"Benthos"}
            </summary>
            <div>
                <p class="">{"We can make decisions on message traffic and translate foreign APIs with: "} <a href="https://www.benthos.dev/">{"benthos"}</a></p>
                <p class="mt-2"><img src="./assets/what-is-blob.svg" alt="what is benthos?" /></p>
                <p class="mt-2">{"Benthos sits on the nats pipeline. Any message sent to nats can be transformed by benthos or create branches and triggers to spawn new functionality."}</p>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="nats">
                <img src="assets/nats-horizontal-color.png" class="w-36 h-16" />
                {"Nats"}
            </summary>
            <div>
                <p class="mt-2">{"Our main pipeline is "} 
                    <a href="https://nats.io/">{"nats"}</a>:
                    <img class="mt-5" src="./assets/nats.png" alt="nats" /></p>
                <p class="mt-2">{"Nats servers 3 purposes for us:"}</p>
                <ul>
                    <li>{"message bus"}</li>
                    <li>{"storage gateway"}</li>
                    <li>{"event store"}</li>
                </ul>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="nixos">
                <img src="assets/nixos-hires.png" class="w-64 h-24" />
                {"NixOS"}
            </summary>
            <div>
                <p class="mt-2">{"Finally we have NixOS which enables us to create any deployment infrastructures the CIM requires in a deterministic fashion created with configuration files. see: "} <a href="https://nixos.org/">{"Nixos"}</a></p>
                <p class="mt-2"><img src="./assets/NixOS.png" alt="NixOS" /></p>
                <p class="mt-2">{"The Nix Packages collection"} <a href="https://github.com/NixOS/nixpkgs">{"Nixpkgs"}</a> {"is a set of over 80,000 packages for the Nix package manager."}</p>
                <p class="mt-2">{"With these three components we have a completely portable and self contained knowledge system."}</p>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="operational-workbench---sage">
                <img src="assets/logo.svg" class="h-24 w-24" />
                {"Operational Workbench - Sage"}
            </summary>
            <div>
                <p class="mt-2">{"Sage provides instant access to AI in the context you need it. Asking questions about your business or a process? CIM prepares AI to be able to answer your question appropriately n the given context."}</p>
                <p class="mt-2"><img src="./assets/Sage.png" alt="Sage" /></p>
                <p class="mt-2">{"Sage is a discussion engine. It can keep track of your discussions and automations with many different local and remote AI Engines. We leverage the best of each tool to do it's jobs and make it all available to the CIM over nats messages."}</p>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="advanced-code-generation">
                <img src="assets/settings.svg" class="h-24 w-24" />
                {"Advanced Code Generation"}
            </summary>
            <div>
                <p class="mt-2">{"CIM is also a tool that will help you generate code. Whether you let it operate behind the scenes or you require human developers to incorporate templates. Sage helps build ideas into software quickly by not only leveraging a workflow solution, but also creating the source code and machine deployments necessary to take that idea into production."}</p>
                <p class="mt-2">{"This is not traditional String Replacement of templates as you may have seen in the past, but a full language for creating source code and verifying it's correctness."}</p>
                <div class="my-5 flex flex-col text-center items-center">
                   <img src="./assets/list.png" class="rounded-lg shadow-lg shadow-black" /> 
                   <span class="my-5">{"becomes"}</span>
                   <img src="./assets/rust-enum.png" class="rounded-lg shadow-lg shadow-black" />
                </div>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="cim-operational-structure">
                <img src="assets/science.svg" class="h-24 w-24" />
                {"CIM Operational Structure"}
            </summary>
            <div>
                <p class="mt-2">{"CIM is configuration driven. Benthos is defined by configuration blocks. nats is defined by configuration blocks. NixOS is defined by configuration blocks. CIM is therefore also defined by configuration blocks."}</p>
                <p class="mt-2">{"This enables a poison part of a CIM to just be dropped, recreated by configuration and populated by stored immutable events in seconds or minutes.  This eliminates any need to backup individual systems, or keep them patched for security. The configurations do all that."}</p>
                <p class="mt-2">{"Configurations are stored in plain text, version controlled and partitioned logically."}</p>
                <p class="mt-2">{"If your functionality dies, this is what consistently recreates it."}</p>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="messaging">
                <img src="assets/chat.svg" class="h-24 w-24" />
                {"Messaging"}
            </summary>
            <div>
                <p class="mt-2">{"We operate on the notion of a State Machine, where any change is only brought about by changing the State of the system. State transitions are represented as a sequential series of Events."}</p>
                <p class="mt-2">{"We have three types of messages:"}</p>
                <ul>
                <li><span class="pl-5 font-bold">{"Command: "}</span>{"Tell the system to change state"}</li>
                <li><span class="pl-5 font-bold">{"Query: "}</span>{"Ask the system it's current state"}</li>
                <li><span class="pl-5 font-bold">{"Event: "}</span>{"A change in state has occurred"}</li>
                </ul>
                <p class="mt-2">{"We also have the notion of:"}</p>
                <ul>
                <li><span class="pl-5 font-bold">{"Operator: "}</span>{"Owner of the CIM (admin)"}</li>
                <li><span class="pl-5 font-bold">{"Account: "}</span>{"Organization Tenant under the Operator"}</li>
                <li><span class="pl-5 font-bold">{"User: "}</span>{"Individual Tenant under the Account"}</li>
                </ul>
                <p class="mt-2">{"Every Message has either an Operator, Account or User associated with it as the Owner of the Message."}</p>
                <p class="mt-2">{"This allows for fine-grained authorization of each Message."}</p>
                <p class="mt-2">{"Commands and Queries can only produce Events"}</p>
                <p class="mt-2">{"Commands are idempotent, meaning a duplicate will not reapply the result unless intentionally requested."}</p>
                <p class="mt-2">{"Commands are the only way to change state in the system. Commands produce an Event of the Results."}</p>
                <p class="mt-2">{"All Events are stored as an infinite sequence of events on S3 or any other similar encapsulated object store such as Azure Blog Storage."}</p>
                <p class="mt-2">{"Events are signals that can be Responded to by infinite authorized listeners including automation and AI Agents replaying previous events."}</p>
            </div>
        </details>
        
        <details class="w-full text-left theme_bg-1-bg-3 rounded-xl mt-5 p-3 shadow-xl shadow-black">
            <summary class="cursor-pointer inline font-bold theme_bg-1-color-5 text-lg" id="command-flow">
                <img src="assets/command.svg" class="h-24 w-24" />
                {"Command Flow"}
            </summary>
            <div>
                <p class="mt-2"><img src="./assets/CommandFlow.svg" alt="Command Flow" /></p>
                <p class="mt-2">{"Events can be isolated for either security or to reduce traffic noise. We operate on a Subject based partitioning system tied directly to your Domain to ensure reliability of intent and categorical separation."}</p>
                <p class="mt-2">{"Events can be ephemeral, local resources don't need to transport to the Domain Cluster or they can be sent and received in batches."}</p>
                <p class="mt-2">{"Events and Objects in Cluster Storage can be reallocated to other Clusters and Leaf Nodes in background automation processes."}</p>
                <p class="mt-2">{"Events can be displayed in real-time subscription to create live dashboards of the CIM."}</p>
                <p class="mt-2">{"Everything runs Asynchronously, meaning there are no blocking actions.  They system continues to function while requests are processed."}</p>
                <p class="mt-2">{"Events can be geographically dispersed according to customer need.  Events are immutable once written and therefore become infinitely cacheable."}</p>
                <p class="mt-2">{"Events do not carry large payloads, instead large binary objects are stored in an Object Store with a verifiable ContentID. This CID is what the message carries over the communications system and retrieved as needed by any authorized process. Content IDs are stamps on the data blocks associated with the large binary object and guarantee what is held inside the blocks."}</p>
                <p class="mt-2">{"This is like having a filename that tells you exactly what the file contains and how to read it.  This is not a moving target such as MIME types which were designed for email.  Rather this is a hashed identifier that describes the exact contents of the file.  Metadata is held about each Object as Events so naming and information about the Object can always be added, removed and inter-related without changing the Object as opposed to holding it within the Object itself."}</p>
            </div>
        </details>
        
        <p class="mt-5 p-2 text-center font-bold text-lg w-full theme_bg-1-bg-2 rounded-lg shadow-lg shadow-black theme_bg-1-color-5">{"There is a better way to handle distributed inter-connected information systems,"}</p>
        <p class="mt-5 text-center p-2 font-bold text-xl w-full theme_bg-1-bg-2 rounded-lg shadow-lg shadow-black theme_bg-1-color-5">{"And we are certain, this is it."}</p>
    </div>  
    }
}
