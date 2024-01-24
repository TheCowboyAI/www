use leptos::*;

#[component]
pub fn AccessGPTs() -> impl IntoView {
  view! {
    <button type="button" 
      onclick="window.location.href='/getaccess';"
      class="text-white 
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
        me-2 
        mb-2"
    >
      {"Access our GPTs"}
    </button>
  }
}

