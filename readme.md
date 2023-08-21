<body>
  <div id="main"><link rel="stylesheet" href="https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css"><div class="min-h-screen flex items-center justify-center bg-gradient-to-r from-blue-400 to-purple-500"><div class="w-full max-w-md bg-white p-6 rounded-lg shadow-md"><h2 class="text-3xl font-semibold mb-4 text-center text-gray-800">Welcome to the Test App</h2><p class="mb-4 text-gray-600">This is a simple test app built with Rust and Dioxus.</p><p class="mb-4 text-gray-600">Feel free to explore and experiment with the app.</p><p class="mb-2 text-gray-600">Please note that this is for demonstration purposes only.</p><p class="text-sm text-gray-500">Created by kord::rs/ts</p><span>You can click on the links below to navigate:</span><div class="mt-2"><ul class="space-y-2"><li><a data-dioxus-id="7" href="/dashboard" dioxus-prevent-default="onclick" class="ttext-gray hover:text-blue-500 cursor-pointer" id="" rel="" target="">Dashboard!</a></li><li><a data-dioxus-id="5" href="/login" dioxus-prevent-default="onclick" class="text-gray hover:text-blue-500 cursor-pointer" id="" rel="" target="">Login</a></li><li><a data-dioxus-id="3" href="/profile" dioxus-prevent-default="onclick" class="text-gray hover:text-blue-500 cursor-pointer" id="" rel="" target="">Profile</a></li></ul></div></div></div></div>
  <script type="module">
    import init from "/./assets/dioxus/name.js";
    init("/./assets/dioxus/name_bg.wasm").then(wasm => {
      if (wasm.__wbindgen_start == undefined) {
        wasm.main();
      }
    });
  </script>
  

<script>// Dioxus-CLI
// https://github.com/DioxusLabs/cli

(function () {
  var protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  var url = protocol + '//' + window.location.host + '/_dioxus/ws';
  var poll_interval = 8080;
  var reload_upon_connect = () => {
      window.setTimeout(
          () => {
              var ws = new WebSocket(url);
              ws.onopen = () => window.location.reload();
              ws.onclose = reload_upon_connect;
          },
          poll_interval);
  };

  var ws = new WebSocket(url);
  ws.onmessage = (ev) => {
      if (ev.data == "reload") {
          window.location.reload();
      }
  };
  ws.onclose = reload_upon_connect;
})()</script><div id="loom-companion-mv3" ext-id="liecbddmkiiihnedobmlmillhodjkdmb"><section id="shadow-host-companion"></section></div></body>