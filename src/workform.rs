use leptos::*;

#[component]
pub fn WorkRequestForm() -> impl IntoView {
  view! {
    <form
      class="mr-5" 
      action="https://formsubmit.co/support@thecowboy.ai" method="POST"
    >
      <div class="my-5 p-5 flex flex-col border-black border-2 border-solid rounded-lg">
        <h2 class="text-lg font-bold">
        {"We can deliver for you"}
        </h2>
          <input
            class="mt-2 p-2 rounded-lg" 
            type="text" 
            name="name" 
            id="name" 
            placeholder="Full Name"
            autocomplete="on" 
            required
          />
          <input
            class="mt-2 p-2 rounded-lg" 
            type="email" 
            name="email" 
            id="email" 
            placeholder="Email Address"
            autocomplete="on" 
            required 
          />
          <textarea
            class="mt-2 p-2 rounded-lg" 
            type="text" 
            name="comment" 
            id="comment"
            rows="4"
            placeholder="Your comments"
            required />
          <input 
            type="hidden" 
            name="_subject" 
            id="_subject" 
            value="New Request for Custom Work" 
          />
          <button
            class="
              text-white 
              bg-gradient-to-br 
              from-purple-600 
              to-blue-500 
              hover:bg-gradient-to-bl 
              focus:ring-4 
              focus:outline-none 
              focus:ring-blue-300 
              dark:focus:ring-blue-800 
              font-bold 
              rounded-lg 
              text-base 
              px-5 
              py-2.5 
              text-center 
              mt-5
              mx-20" 
              type="submit"
          >
            Send
          </button>
        </div>
      </form>                
  }
}