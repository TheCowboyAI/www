use leptos::*;

#[component]
pub fn Architecture() -> impl IntoView {
    view! {
      <div class="mt-5 pr-5 flex flex-col animate-fadeIn">
        <div>
            <h1 class="font-bold text-white text-xl" id="architecture">{"Architecture"}</h1>
            <p class="mt-2">{"Cowboy AI uses the concept of a CIM, a Composable Information Machine."}</p>
            <p class="mt-2">{"The Composable Information Machine relies on some dependencies, which have a fitness function for replacement."}</p>
            <p class="mt-2">{"Primarily the CIM is driven by Word Lists, Messages and Relationships. We need a way to make all these communicate and generate a working system, for that we use industry proven tools."}</p>
            <p class="mt-2">{"These dependencies enable us to work with an infinite range of connections to a central nervous system."}</p>
            <p class="mt-2"><img src="./assets/CIMArchitecture.svg" alt="Architecture" /></p>
            <p class="mb-5">{"We control the central nervous system, tools enable it to function. We favor Open Source software we control, as to not be vendor-locked into a business decision we are forced to perform. If you prefer a different tool in these positions, they are adaptable, but heavily opinionated in this current version of CIM. For the latest developments on the CIM Architecture "} <span class="font-bold">see: </span><a href="http://www.cimlabs.org">{"CIM Labs"}</a></p>
        </div>
        <details>
            <summary class="font-bold text-white text-lg" id="benthos">{"Benthos"}</summary>
            <div>
                <p class="">{"We can make decisions on message traffic and translate foreign APIs with"} <a href="https://www.benthos.dev/">{"benthos"}</a></p>
                <p class="mt-2"><img src="./assets/what-is-blob.svg" alt="what is benthos?" /></p>
                <p class="mt-2">{"Benthos sits on the nats pipeline. Any message sent to nats can be transformed by benthos or create branches and triggers to spawn new functionality."}</p>
            </div>
        </details>
        <details>
            <summary class="font-bold text-white text-lg" id="nats">{"Nats"}</summary>
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
        <details>
            <summary class="font-bold text-white text-lg" id="nixos">{"NixOS"}</summary>
            <div>
                <p class="mt-2">{"Finally we have NixOS which enables us to create any deployment infrastructures the CIM requires in a deterministic fashion created with configuration files."} <a href="https://nixos.org/">{"Nixos"}</a></p>
                <p class="mt-2"><img src="./assets/NixOS.png" alt="NixOS" /></p>
                <p class="mt-2">{"The Nix Packages collection"} <a href="https://github.com/NixOS/nixpkgs">{"Nixpkgs"}</a> {"is a set of over 80,000 packages for the Nix package manager."}</p>
                <p class="mt-2">{"With these three components we have a completely portable and self contained knowledge system."}</p>
            </div>
        </details>
        <details>
            <summary class="font-bold text-white text-lg" id="operational-workbench---sage">{"Operational Workbench - Sage"}</summary>
            <div>
                <p class="mt-2">{"Sage provides instant access to AI in the context you need it. Asking questions about your business or a process? CIM prepares AI to be able to answer your question appropriately n the given context."}</p>
                <p class="mt-2"><img src="./assets/Sage.png" alt="Sage" /></p>
                <p class="mt-2">{"Sage is a discussion engine. It can keep track of your discussions and automations with many different local and remote AI Engines. We leverage the best of each tool to do it's jobs and make it all available to the CIM over nats messages."}</p>
            </div>
        </details>
        <details>
            <summary class="font-bold text-white text-lg" id="advanced-code-generation">{"Advanced Code Generation"}</summary>
            <div>
                <p class="mt-2">{"CIM is also a tool that will help you generate code. Whether you let it operate behind the scenes or you require human developers to incorporate templates. Sage helps build ideas into software quickly by not only leveraging a workflow solution, but also creating the source code and machine deployments necessary to take that idea into production."}</p>
                <p class="mt-2">{"This is not traditional String Replacement of templates as you may have seen in the past, but a full language for creating source code and verifying it's correctness."}</p>
            </div>
        </details>
        <details>
            <summary class="font-bold text-white text-lg" id="cim-operational-structure">{"CIM Operational Structure"}</summary>
            <div>
                <p class="mt-2">{"CIM is configuration driven. Benthos is defined by configuration blocks. nats is defined by configuration blocks. NixOS is defined by configuration blocks. CIM is therefore also defined by configuration blocks."}</p>
                <p class="mt-2">{"This enables a poison part of a CIM to just be dropped, recreated by configuration and populated by stored immutable events in seconds or minutes.  This eliminates any need to backup individual systems, or keep them patched for security. The configurations do all that."}</p>
                <p class="mt-2">{"Configurations are stored in plain text, version controlled and partitioned logically."}</p>
                <p class="mt-2">{"If your functionality dies, this is what consistently recreates it."}</p>
            </div>
        </details>
        <details>
            <summary class="font-bold text-white text-lg" id="messaging">{"Messaging"}</summary>
            <div>
                <p class="mt-2">{"We operate on the notion of a State Machine, where any change is only brought about by changing the State of the system. State transitions are represented as a sequential series of Events."}</p>
                <p class="mt-2">{"We have three types of messages:"}</p>
                <ul>
                <li>{"Command:"}<ul><li>{"Tell the system to change state"}</li></ul></li>
                <li>{"Query:"}<ul><li>{"Ask the system it's current state"}</li></ul></li>
                <li>{"Event:"}<ul><li>{"A change in state has occurred"}</li></ul></li>
                </ul>
                <p class="mt-2">{"We also have the notion of:"}</p>
                <ul>
                <li>{"Operator:"}<ul><li>{"Owner of the CIM (admin)"}</li></ul></li>
                <li>{"Account:"}<ul><li>{"Organization Tenant under the Operator"}</li></ul></li>
                <li>{"User:"}<ul><li>{"Individual Tenant under the Account"}</li></ul></li>
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
        <details>
            <summary class="font-bold text-white text-lg" id="command-flow">{"Command Flow"}</summary>
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
        <p class="mt-2 font-bold text-lg">{"There is a better way to handle distributed inter-connected information systems,"}</p>
        <p class="m-2 font-bold text-xl">{"And we are certain, this is it."}</p>
    </div>  
    }
}
