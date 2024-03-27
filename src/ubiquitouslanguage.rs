use leptos::*;

#[component]
pub fn UbiquitousLanguage() -> impl IntoView {
    view! {
        <div class="mt-5 animate-fadeIn">
            <div class="flex flex-col theme_bg-1-bg-3 gap-4 rounded-xl p-3 shadow-xl shadow-black">
                <h1 class="font-bold rounded-lg shadow shadow-black p-2 text-center theme_bg-1-bg-2 theme_bg-1-color-5 text-xl">
                    {"Collaborate with Clarity"}
                </h1>
                <p>
                    {"Ubiquitous Language is like a common language or set of terms that everyone in a business uses. Think of it as everyone being on the same page, using the same words to describe things, which helps avoid confusion and makes it easier for everyone to work together effectively."}
                </p>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"What is Ubiquitous Language?"}
                    </summary>
                    <p>
                        {"Ubiquitous Language is a set of words or terms that everyone in a project or business agrees to use. Instead of having different departments using their own jargon, everyone uses the same language, making communication clearer and more direct."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Breaking Down Barriers"}
                    </summary>
                    <p>
                        {"By using a Ubiquitous Language, it's like breaking down walls between different groups in the business. Whether it's the tech team, the marketing department, or customer service, everyone understands each other better because they're all using the same words."}
                    </p>
                </details>
                <details class="text-black text-sm">
                    <summary class="theme_bg-1-color-5 text-base font-bold">
                        {"Collaborating More Effectively"}
                    </summary>
                    <p>
                        {"When everyone speaks the same language, it's easier to share ideas, solve problems, and make decisions together. It's like having a group project where everyone is on the same wavelength, making teamwork smoother and more productive."}
                    </p>
                </details>

                <p>
                    {"Embracing Ubiquitous Language in a business sets the stage for more effective collaboration. It ensures that everyone, no matter their role, can communicate clearly, understand each other's perspectives, and work together towards common goals."}
                </p>
            </div>
        </div>
    }
}
