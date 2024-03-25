use leptos::*;

use crate::cards::Card;

#[component]
pub fn CardTest() -> impl IntoView {
  view! {
    <div class="grid-cols-1 sm:grid md:grid-cols-3 ">
      <Card
        route="https://www.cimlabs.org".to_string()
        imgsrc="assets/CIM-Card.png".to_string() 
        imgalt="CIM".to_string()
        title="Composable Information Machine".to_string()
        text="CIM is a systemized graph of information, every node and edge has a single purpose, designed for composition. We are transitioning from the world of Location-based Hierarchical Files, Rows and Columns, to something else.".to_string()
        footer="Model your success".to_string()
      />
    </div>
  }
}
