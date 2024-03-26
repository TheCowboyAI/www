use leptos::*;

use crate::cards::Card;

#[component]
pub fn Landing() -> impl IntoView {
  view! {
    <div class="animate-fadeIn flex flex-col">
      <div>
        <h1 class="bg-[#699EBF] mx-3 rounded-xl py-2 font-bold mt-5 text-3xl text-center text-white shadow-lg shadow-black">
          {"It's a Brave, New World"}
        </h1>
        <h2 class="bg-[#699EBF] mx-3 rounded-xl py-2 font-bold mt-5 text-xl text-center text-white shadow-lg shadow-black">
          {"Overcome Information Overload"}
        </h2>
      </div>
    </div>
    <div class="grid-cols-1 grid md:grid-cols-3 items-justify">
      <Card
        route="/aiintegration".to_string()
        imgsrc="assets/hands.png".to_string() 
        imgalt="AI Integration".to_string()
        title="AI Integration".to_string()
        text="Cowboy AI stands at the forefront of AI integration, expertly navigating the complexities of artificial intelligence to tailor solutions that are perfectly aligned with your business needs. As AI Integration Experts, Cowboy AI ensures that your systems are intelligent, efficient, and strategically designed to drive growth and innovation.".to_string()
        footer="Integrated Integrity".to_string()
        footlink="/aicomposable".to_string()
      />
      <Card
        route="/businessevolution".to_string()
        imgsrc="assets/bizevo.png".to_string() 
        imgalt="evolution".to_string()
        title="Business Evolution".to_string()
        text="A live Business Model lets you evolve. Business evolution is the continuous transformation and adaptation of a business to meet changing market demands, technology advancements, and shifts in consumer behavior. Cowboy AI delivers a model that works today and will continue to transform with you going forward.".to_string()
        footer="Automated Excellence".to_string()
        footlink="businessmodel".to_string()
      />
      <Card
        route="/cim".to_string()
        imgsrc="assets/CIM-Card.png".to_string() 
        imgalt="CIM".to_string()
        title="Composable Information Machine".to_string()
        text="CIM is a new way to see the world of information. We are transitioning from the world of location-based hierarchical files with rows and columns, to something else...".to_string()
        footer="Model your success".to_string()
        footlink="/architecture".to_string()
      />
      <Card
        route="/existingapps".to_string()
        imgsrc="assets/FOSS.png".to_string() 
        imgalt="open source".to_string()
        title="FREE and Open Source".to_string()
        text="This is a machine to control your information and how it is processed. When we say machine... think of the entire internet as a single machine. This is a new enabling technology that will guide you to create a personal information system capable of semantic engagement with all your information, no matter where you choose to create it or store it.".to_string()
        footer="Software freedom for all".to_string()
        footlink="https://github.com/TheCowboyAI/cim-start".to_string()
      />
      <Card
        route="/gpts".to_string()
        imgsrc="assets/custom-gpts.png".to_string() 
        imgalt="Custom GPTs".to_string()
        title="Custom GPTs".to_string()
        text="Cowboy AI excels in creating tailored GPTs and AI Agents that are not just technologically advanced but also intricately aligned with your business model. These intelligent agents are designed to understand your business's unique landscape, communicate with each other, and securely access internal company data to drive informed decisions and actions.".to_string()
        footer="Personal AI Assistants".to_string()
        footlink="/askquestions".to_string()
      />
      <Card
        route="https://microsoft.com/en-US/ai".to_string()
        imgsrc="https://img-prod-cms-rt-microsoft-com.akamaized.net/cms/api/am/imageFileData/RE1Mu3b?ver=5c31".to_string() 
        imgalt="Startup Hub".to_string()
        title="Founders Hub Member".to_string()
        text="Microsoft for Startups Founders Hub helps startups radically accelerate innovation by providing access to industry-leading AI services, expert guidance, and the essential technology needed to build a future-proofed startup.".to_string()
        footer="Build the future".to_string()
        footlink="https://www.microsoft.com/en-us/startups/".to_string()
      />
      <Card
        route="https://mvp.microsoft.com/en-US/".to_string()
        imgsrc="assets/mvp_alumni_logo.png".to_string() 
        imgalt="MVP Alumni".to_string()
        title="MVP Alumni".to_string()
        text="Microsoft recognizes exceptional community leaders for their technical expertise, positive influence on those around them and commitment to solving real world challenges with cutting-edge technology.".to_string()
        footer="10 Consecutive Years".to_string()
        footlink="/customersupport".to_string()
      />
      <div class="mb-10">
      </div>  
    </div>
  }
}
