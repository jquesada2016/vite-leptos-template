#[macro_use]
extern crate tracing;

use leptos::*;
use tracing_subscriber::prelude::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() {
  tracing_subscriber::fmt()
    .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
    .without_time()
    .pretty()
    .with_max_level(tracing::Level::TRACE)
    .init();

  info!("WASM successfully initialized");

  mount_to_body(|cx| view! { cx, <App /> });
}

#[component]
fn App(cx: Scope) -> impl IntoView {
  view! { cx,
      <div class="h-screen w-full flex justify-center items-center">
          <h1 class="text-5xl">"Hello, leptos!"</h1>
      </div>
  }
}
