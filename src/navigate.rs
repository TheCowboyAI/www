use leptos::*;
use crate::footer::Footer;

#[component]
pub fn Navigate() -> impl IntoView {
    view! {
    // mini menu
    <div class="w-24 md:w-36 lg:w-48">
        <nav class="flex flex-col items-center pt-3 pl-3">
            <a href="/">
              <div class="py-4 flex flex-row svg-container items-center w-20 md:w-32">
                  <img src="assets/logo.svg" />
                  <div class="text-xs md:text-base font-bold caption text-center">Cowboy AI</div>
              </div>
            </a>

              <div class="mt-5 flex flex-col space-y-3 > * + *">

                  <a href="/tooling" 
                    class="p-2 rounded-lg text-white 
                    hover:text-black hover:bg-white 
                    hover:font-bold hover:border-black 
                    hover:border-2
                    hover:border-solid">
                   <svg 
                      class="h-8 w-8 md:h-12 md:w-12" 
                      fill="#000000" 
                      width="800px" 
                      height="800px" 
                      viewBox="0 0 512 512" 
                      enable-background="new 0 0 512 512" 
                    >
                    <title>AI Tooling</title>
                    <g>
                    <path d="M183.088,338.025h303.796c2.694,0,5.245-1.187,6.978-3.252c1.733-2.065,2.468-4.794,2-7.441   c-0.012-0.047-3.542-20.103-6.562-41.238c-5.156-36.111-49.319-46.958-54.63-48.139c-0.006,0-0.006-0.006-0.012-0.006   c0,0-35.975-7.892-44.988-10.668c-9.891-3.05-17.522-10.206-21.64-19.432c32.29-14.982,49.426-51.782,52.191-88.831v-4.035h12.413   c5.032,0,9.114-4.082,9.114-9.114c0-5.032-4.082-9.114-9.114-9.114h-8.117l-0.92-9.304c-2.545-25.751-20.227-47.148-45.035-54.493   l-12.763-3.786v-4.059c0-5.032-4.076-9.114-9.114-9.114h-43.226c-5.026,0-9.102,4.07-9.114,9.09l-0.012,4.035l-13.072,3.869   c-24.713,7.334-42.383,28.623-45.012,54.244l-0.979,9.517h-7.933c-5.038,0-9.114,4.082-9.114,9.114   c0,5.032,4.076,9.114,9.114,9.114h12.775l0.012,4.723c2.735,36.408,19.765,72.982,51.871,88.018   c-4.112,9.286-11.76,16.495-21.681,19.557c-9.025,2.777-45,10.668-45,10.668c-0.006,0-0.012,0.006-0.012,0.006   c-5.316,1.181-49.474,12.027-54.63,48.139c-3.02,21.135-6.551,41.19-6.562,41.238c-0.469,2.646,0.267,5.376,2,7.441   C177.843,336.838,180.394,338.025,183.088,338.025z M471.255,288.682c1.685,11.76,3.519,23.188,4.836,31.115H452.6v-55.134   C461.323,269.867,469.677,277.616,471.255,288.682z M373.376,50.427c17.676,5.245,30.267,20.483,32.083,38.829l0.742,7.5h-40.401   V48.178L373.376,50.427z M347.571,34.229v62.527h-25.152l0.136-62.527H347.571z M264.383,89.102   c1.875-18.24,14.46-33.406,32.059-38.639l7.856-2.326l-0.107,48.619h-40.597L264.383,89.102z M268.323,119.019v-4.035h133.688   l0.012,3.347c-2.937,39.007-24.749,78.37-66.865,78.37C293.048,196.7,271.236,157.337,268.323,119.019z M335.158,214.928   c5.334,0,10.342-0.593,15.166-1.489c2.617,6.557,6.456,12.419,11.256,17.361l-26.22,21.295l-26.742-21.598   c4.587-4.842,8.283-10.621,10.935-17.136C324.507,214.311,329.664,214.928,335.158,214.928z M293.362,241.599l36.289,29.306   c1.667,1.353,3.697,2.029,5.726,2.029c2.035,0,4.07-0.676,5.744-2.041l35.838-29.104c2.356,1.121,55.188,14.288,57.413,14.958   v63.05H235.6v-63.05C237.825,256.077,283,251.5,293.362,241.599z M198.717,288.682c1.578-11.066,9.933-18.815,18.655-24.019v55.134   h-23.491C195.198,311.87,197.032,300.43,198.717,288.682z"/>
                    <path d="M16,27.548v456.917c0,6.361,5.18,11.535,11.547,11.535h80.197c6.367,0,11.547-5.174,11.547-11.535V27.548   c0-6.373-5.18-11.547-11.547-11.547H27.547C21.18,16.001,16,21.175,16,27.548z M34.228,34.229h66.835V76.76H73.721   c-5.032,0-9.114,4.082-9.114,9.114c0,5.032,4.082,9.114,9.114,9.114h27.342v30.38H73.721c-5.032,0-9.114,4.082-9.114,9.114   c0,5.032,4.082,9.114,9.114,9.114h27.342v30.38H73.721c-5.032,0-9.114,4.082-9.114,9.114s4.082,9.114,9.114,9.114h27.342v30.38   H73.721c-5.032,0-9.114,4.082-9.114,9.114c0,5.032,4.082,9.114,9.114,9.114h27.342v30.38H73.721c-5.032,0-9.114,4.082-9.114,9.114   c0,5.032,4.082,9.114,9.114,9.114h27.342v30.38H73.721c-5.032,0-9.114,4.082-9.114,9.114s4.082,9.114,9.114,9.114h27.342v30.38   H73.721c-5.032,0-9.114,4.082-9.114,9.114c0,5.032,4.082,9.114,9.114,9.114h27.342v30.38H73.721c-5.032,0-9.114,4.082-9.114,9.114   c0,5.032,4.082,9.114,9.114,9.114h27.342v42.531H34.228V34.229z"/>
                    <path d="M483.241,495.999c7.031,0,12.757-5.72,12.757-12.757v-65.613c0-7.037-5.726-12.769-12.757-12.769H242.987   c-2.611,0-74.104,34.165-74.104,34.165c-4.355,2.172-7.061,6.539-7.061,11.416c0,4.865,2.706,9.233,7.055,11.404   c0,0,71.505,34.153,74.11,34.153H483.241z M477.77,477.771h-26.38v-54.683h26.38V477.771z M186.814,450.441l43.41-21.717v43.422   L186.814,450.441z M248.452,423.088h184.711v54.683H248.452V423.088z"/>
                    </g>
                    </svg>
                  </a>

                  <a href="/workflow" 
                  class="p-2 rounded-lg text-white 
                    hover:text-black hover:bg-white 
                    hover:font-bold hover:border-black 
                    hover:border-2
                    hover:border-solid">
                    <svg class="h-8 w-8 md:h-12 md:w-12" viewBox="0 0 24 24"
                        width="800px" height="800px" viewBox="0 0 24 24">
                        <title>Workflow</title>
                        <path d="M21 18v-4h-5.042L13 11.042V8.95a3.5 3.5 0 1 0-1 0v2.074L9.024 14H4v4H2v5h5v-5H5v-3h3.958l3.532 3.533L16.024 15H20v3h-2v5h5v-5zM6 22H3v-3h3zm4-16.5A2.5 2.5 0 1 1 12.5 8 2.5 2.5 0 0 1 10 5.5zm2.485 11.633l-2.6-2.6 2.5-2.5h.2l2.5 2.5zM22 22h-3v-3h3z"/>
                      </svg>
                  </a>

                  <a href="/events" 
                    class="p-2 rounded-lg text-white 
                      hover:text-black hover:bg-white 
                      hover:font-bold hover:border-black 
                      hover:border-2
                      hover:border-solid">
                    <svg class="h-8 w-8 md:h-12 md:w-12" viewBox="0 0 24 24">
                          <title>Events</title>
                          <path
                              d="M19 19H5V8h14m-3-7v2H8V1H6v2H5c-1.11 0-2 .89-2
                              2v14a2 2 0 002 2h14a2 2 0 002-2V5a2 2 0
                              00-2-2h-1V1m-1 11h-5v5h5v-5z" />
                      </svg>
                  </a>

                  <a href="/dashboard"
                    class="p-2 rounded-lg text-white 
                           hover:text-black hover:bg-white 
                           hover:font-bold hover:border-black 
                           hover:border-2
                           hover:border-solid">
                      <svg 
                        class="h-8 w-8 md:h-12 md:w-12 md:h-12 md:w-12" 
                        viewBox="0 0 1920 1920">
                          <title>Dashboard</title>
                          <path 
                            fill="black" 
                            width="800px" 
                            height="800px" 
                            fill-rule="evenodd"
                            d="M833.935 1063.327c28.913 170.315 64.038 348.198 83.464 384.79 27.557 51.84 92.047 71.944 144 44.387 51.84-27.558 71.717-92.273 44.16-144.113-19.426-36.593-146.937-165.46-271.624-285.064Zm-43.821-196.405c61.553 56.923 370.899 344.81 415.285 428.612 56.696 106.842 15.811 239.887-91.144 296.697-32.64 17.28-67.765 25.411-102.325 25.411-78.72 0-154.955-42.353-194.371-116.555-44.386-83.802-109.102-501.346-121.638-584.245-3.501-23.717 8.245-47.21 29.365-58.277 21.346-11.294 47.096-8.02 64.828 8.357ZM960.045 281.99c529.355 0 960 430.757 960 960 0 77.139-8.922 153.148-26.654 225.882l-10.39 43.144h-524.386v-112.942h434.258c9.487-50.71 14.231-103.115 14.231-156.084 0-467.125-380.047-847.06-847.059-847.06-467.125 0-847.059 379.935-847.059 847.06 0 52.97 4.744 105.374 14.118 156.084h487.454v112.942H36.977l-10.39-43.144C8.966 1395.137.044 1319.128.044 1241.99c0-529.243 430.645-960 960-960Zm542.547 390.686 79.85 79.85-112.716 112.715-79.85-79.85 112.716-112.715Zm-1085.184 0L530.123 785.39l-79.85 79.85L337.56 752.524l79.849-79.85Zm599.063-201.363v159.473H903.529V471.312h112.942Z"/>
                      </svg>
                  </a>

                  <a href="/people" 
                    class="p-2 rounded-lg text-white 
                      hover:text-black hover:bg-white 
                      hover:font-bold hover:border-black 
                      hover:border-2
                      hover:border-solid">
                     <svg class="h-8 w-8 md:h-12 md:w-12" viewBox="0 0 24 24">
                          <title>People</title>
                          <path
                              d="M12 4a4 4 0 014 4 4 4 0 01-4 4 4 4 0 01-4-4 4 4 0
                              014-4m0 10c4.42 0 8 1.79 8 4v2H4v-2c0-2.21 3.58-4
                              8-4z"
                          ></path>
                      </svg>
                  </a>

                  <a href="/customwork" 
                    class="p-2 rounded-lg text-white 
                      hover:text-black hover:bg-white 
                      hover:font-bold hover:border-black 
                      hover:border-2
                      hover:border-solid">
                       <svg class="h-8 w-8 md:h-12 md:w-12" version="1.1" id="SCIENCE" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
                          width="800px" height="800px" viewBox="0 0 1800 1800" enable-background="new 0 0 1800 1800" xml:space="preserve">
                          <title>Custom Work</title>
                          <g>
                        <path fill="#000000" d="M890.431,974.791c69.867,0,126.714-56.838,126.714-126.704c0-69.867-56.847-126.705-126.714-126.705
                          c-69.867,0-126.7,56.838-126.7,126.705C763.73,917.953,820.564,974.791,890.431,974.791z M890.431,782.932
                          c35.933,0,65.163,29.226,65.163,65.155c0,35.928-29.23,65.159-65.163,65.159c-35.928,0-65.154-29.231-65.154-65.159
                          C825.276,812.158,854.503,782.932,890.431,782.932z"/>
                        <path fill="#000000" d="M1393.528,849.313c107.104-90.789,190.083-183.293,228.049-264.898c-9.547,1.934-19.43,2.959-29.544,2.959
                          c-13.436,0-26.451-1.824-38.848-5.184c-20.757,36.129-52.147,77.005-93.216,120.522c-33.35,35.331-71.835,71.25-114.557,107.113
                          c-53.068-42.077-111.368-83.756-173.604-124.039c-4.372-67.951-11.445-134.917-21.162-198.771
                          c112.563-39.035,214.152-61.298,293.431-62.93c2.182-22.314,9.32-43.18,20.294-61.515c-3.63-0.083-7.229-0.192-10.965-0.192
                          c-84.31,0-192.959,22.611-313.084,63.409C1095.904,186.437,1011.979,4.434,892.464,4.434
                          c-119.523,0-203.453,182.002-247.859,421.353c-120.133-40.798-228.778-63.409-313.088-63.409c-0.375,0-0.724,0.018-1.1,0.018
                          c11.079,18.457,18.265,39.497,20.425,61.986c77.738,3.207,175.549,25.221,283.43,62.633
                          c-9.717,63.854-16.791,130.823-21.167,198.779c-62.231,40.283-120.526,81.954-173.594,124.03
                          c-42.723-35.859-81.208-71.783-114.557-107.113c-40.27-42.661-71.242-82.8-92.002-118.405c-9.727,2.007-19.793,3.067-30.104,3.067
                          c-14.146,0-27.821-2.034-40.789-5.738c37.355,82.281,121.011,175.863,229.337,267.678
                          c-41.208,35.039-78.546,70.143-111.206,104.753c-122.351,129.632-165.36,236.671-124.393,309.549
                          c27.241,48.456,86.356,73.026,175.706,73.026c84.314,0,192.959-22.607,313.097-63.414
                          c24.544,132.286,61.167,247.014,109.194,323.216c9.032-20.979,22.769-39.462,39.855-54.159
                          c-18.356-33.799-35.767-76.254-51.313-126.133c-15.473-49.611-28.637-104.691-39.362-163.867
                          c61.358-23.117,125.033-50.542,189.49-81.762c64.452,31.22,128.123,58.645,189.481,81.757
                          c-10.729,59.177-23.894,114.261-39.362,163.872c-15.909,51.043-33.772,94.35-52.609,128.533
                          c16.847,15.207,30.182,34.227,38.69,55.668c49.245-76.158,86.688-192.578,111.659-327.129
                          c120.13,40.811,228.783,63.418,313.102,63.418c89.336,0,148.461-24.574,175.697-73.031
                          c40.972-72.873-2.038-179.912-124.388-309.544C1472.069,919.455,1434.731,884.351,1393.528,849.313z M1296.368,849.505
                          c-37.753,29.492-78.184,58.771-120.802,87.491c1-29.188,1.519-58.396,1.519-87.482c0-29.096-0.519-58.313-1.519-87.5
                          C1218.185,790.729,1258.615,820.013,1296.368,849.505z M669.376,849.514c0-43.538,1.091-86.514,3.186-128.655
                          c35.701-22.472,72.638-44.477,110.582-65.809c36.478-20.508,73.009-39.851,109.321-58.016
                          c36.308,18.165,72.838,37.508,109.312,58.016c37.953,21.329,74.881,43.337,110.582,65.805c2.099,42.137,3.181,85.117,3.181,128.66
                          c0,43.529-1.082,86.504-3.181,128.638c-35.706,22.472-72.638,44.48-110.591,65.817c-36.474,20.5-73,39.848-109.303,58.008
                          c-36.308-18.16-72.843-37.508-109.312-58.012c-37.953-21.338-74.885-43.346-110.591-65.818
                          C670.467,936.018,669.376,893.043,669.376,849.514z M1107.445,645.493c-24.697-14.966-49.878-29.688-75.514-44.097
                          c-23.257-13.072-46.558-25.683-69.832-37.853c44.146-20.364,87.635-38.786,129.942-55.07
                          C1098.47,552.443,1103.618,598.281,1107.445,645.493z M742.337,282.868C784.679,147.061,840.793,65.98,892.464,65.98
                          c51.663,0,107.781,81.081,150.119,216.888c15.469,49.617,28.633,104.696,39.358,163.864
                          c-61.354,23.113-125.029,50.537-189.477,81.762c-64.457-31.225-128.127-58.649-189.49-81.762
                          C713.708,387.563,726.864,332.484,742.337,282.868z M692.873,508.469c42.308,16.284,85.802,34.711,129.947,55.075
                          c-23.274,12.17-46.571,24.78-69.828,37.853c-25.635,14.408-50.825,29.13-75.522,44.097
                          C681.301,598.281,686.45,552.443,692.873,508.469z M609.357,762.019c-1.004,29.187-1.527,58.399-1.527,87.495
                          c0,29.086,0.523,58.295,1.523,87.478c-42.618-28.716-83.04-57.999-120.796-87.482C526.313,820.017,566.735,790.734,609.357,762.019
                          z M331.504,1275.095c-63.618,0-106.965-14.792-122.062-41.64c-25.312-45.035,17.864-133.687,115.508-237.147
                          c33.415-35.418,72.005-71.416,114.841-107.362c53.796,42.586,112.301,84.358,173.302,123.996
                          c4.372,68.047,11.445,135.112,21.18,199.063C517.268,1252.575,412.053,1275.095,331.504,1275.095z M692.873,1190.541
                          c-6.427-44.002-11.581-89.874-15.412-137.125c24.955,15.172,50.17,29.941,75.535,44.197c23.252,13.072,46.553,25.683,69.832,37.852
                          C778.68,1155.83,735.181,1174.261,692.873,1190.541z M962.1,1135.465c23.274-12.169,46.575-24.779,69.823-37.844
                          c25.36-14.26,50.577-29.029,75.535-44.201c-3.831,47.247-8.979,93.119-15.416,137.116
                          C1049.734,1174.261,1006.245,1155.83,962.1,1135.465z M1575.478,1233.455c-15.097,26.848-58.443,41.64-122.053,41.64
                          c-80.558,0-185.769-22.52-302.777-63.091c9.73-63.95,16.813-131.012,21.18-199.063c61.001-39.634,119.51-81.404,173.311-123.996
                          c42.827,35.946,81.417,71.944,114.841,107.362C1557.614,1099.769,1600.795,1188.42,1575.478,1233.455z"/>
                        <path fill="#000000" d="M323.685,429.749c0-70.146-57.069-127.211-127.215-127.211c-70.142,0-127.207,57.064-127.207,127.211
                          c0,70.142,57.065,127.211,127.207,127.211C266.616,556.96,323.685,499.891,323.685,429.749z M131.817,429.749
                          c0-35.649,29.003-64.657,64.653-64.657c35.653,0,64.661,29.008,64.661,64.657c0,35.649-29.008,64.657-64.661,64.657
                          C160.82,494.406,131.817,465.398,131.817,429.749z"/>
                        <path fill="#000000" d="M1603.526,302.538c-70.146,0-127.211,57.064-127.211,127.211c0,70.142,57.064,127.211,127.211,127.211
                          c70.146,0,127.211-57.069,127.211-127.211C1730.737,359.603,1673.672,302.538,1603.526,302.538z M1603.526,494.406
                          c-35.653,0-64.657-29.008-64.657-64.657c0-35.649,29.004-64.657,64.657-64.657s64.656,29.008,64.656,64.657
                          C1668.183,465.398,1639.18,494.406,1603.526,494.406z"/>
                        <path fill="#000000" d="M892.983,1541.145c-70.142,0-127.206,57.064-127.206,127.211s57.064,127.211,127.206,127.211
                          c70.15,0,127.22-57.064,127.22-127.211S963.134,1541.145,892.983,1541.145z M892.983,1733.013
                          c-35.649,0-64.652-29.003-64.652-64.657c0-35.652,29.003-64.656,64.652-64.656c35.658,0,64.666,29.004,64.666,64.656
                          C957.649,1704.01,928.642,1733.013,892.983,1733.013z"/>
                      </g>
                      </svg>
                    </a>
                  </div>
                  <div class="mt-10">
                  <Footer />
                  </div>
              </nav>
    </div>
    }
}
