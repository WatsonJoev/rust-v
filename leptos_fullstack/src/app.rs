use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-actix.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (text, set_text) = create_signal("".to_string());
    let (logo, set_logo) = create_signal("".to_string());
    let (color, set_color) = create_signal("".to_string());
    let (src, set_src) = create_signal("https://img.shields.io/badge/default-000".to_string());

    view! {
        <link rel="stylesheet" href="https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" />
        <div class="mx-auto p-4 bg-gray-100 h-screen flex justify-center">
            <div class="mx-auto flex flex-col">
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Text"
                </label>
                <input
                    name="Text"
                    class="mb-3 pl-3 mx-auto"
                    value=text.get()
                    on:input=move |e| set_text.set(event_target_value(&e))
                />
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Logo"
                </label>
                <input
                    name="Logo"
                    class="mb-3 pl-3 mx-auto"
                    value=logo.get()
                    on:input=move |e| set_logo.set(event_target_value(&e))
                />
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Color#"
                </label>
                <input
                    name="Color#"
                    class="mb-3 pl-3 mx-auto"
                    value=color.get()
                    on:input=move |e| set_color.set(event_target_value(&e))
                />
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded"
                    on:click=move |_| {
                        let new_src = format!(
                            "https://img.shields.io/badge/{}-{}?logo={}",
                            text.get(), color.get(), logo.get()
                        );
                        set_src.set(new_src);
                    }
                >
                    "Update Src!"
                </button>
                <img class="mt-5" src=src alt=src />
                <img class="mt-5" src="/assets/icons/copy_fill.svg" alt="Testing" />
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
