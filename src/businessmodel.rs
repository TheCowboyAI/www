use leptos::*;

#[component]
pub fn BusinessModel() -> impl IntoView {
    view! {
        <div class="mt-10 flex items-center animate-fadeIn">
            <div class="flex-row"> 
                <h1 class="font-bold text-white text-4xl md:text-6xl">
                {"Business Model"}
                </h1>
                <p class="mt-5">
                {"A business model is crucial because it serves as a blueprint for your company, guiding every aspect of its operation. Here's why it's essential:"}
                </p>
                <ol
                    class=" 
                        mr-5 py-2
                        text-black text-xl md:text-2xl lg:text-3xl 
                    "
                >
                    <li>
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Direction and Clarity"}
                            </summary>
                            <p>
                            {"It offers a clear understanding of your business core components, such as your value proposition, customer segments, and revenue streams, ensuring that your efforts are aligned and purposeful."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Strategic Decision-Making"}
                            </summary>
                            <p>
                            {"A well-defined business model aids in making informed decisions. It helps you understand your market, your competition, and your customers, allowing you to strategize effectively."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">      
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Resource Allocation"}
                            </summary>
                            <p>
                            {"It helps in identifying and allocating key resources and activities efficiently, ensuring that your business operates smoothly and effectively."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Risk Management"}
                            </summary>
                            <p>
                            {"By understanding the market and customer needs, you can anticipate potential risks and devise strategies to mitigate them."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Investor Confidence"}
                            </summary>
                            <p>
                            {"A robust business model demonstrates to investors and stakeholders that you have a viable, well-thought-out approach to achieving success, which can help in securing funding and support."}
                                    </p>
                        </details>
                    </li>
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Measuring Success"}
                            </summary>
                            <p>
                            {"It provides a framework for measuring progress and performance, allowing you to track your business growth and make necessary adjustments."}
                            </p>
                        </details>
                    </li>
                    <li class="mt-5">                    
                        <details class="text-black text-sm">
                            <summary class="text-white text-base font-bold">
                            {"Customer Understanding"}
                            </summary>
                            <p>
                            {"A good business model keeps you customer-focused, ensuring that you meet their needs and solve their problems, which is crucial for business sustainability and growth."}
                            </p>
                        </details>
                    </li>
                </ol>
                <p class="mt-5">
                {"
                A business model is not just about how you make money but about how you create, deliver, and capture value. It's a roadmap for success, helping you to navigate the complex business landscape efficiently and effectively.
                "}
                </p>
            </div>
        </div>
    }
}
