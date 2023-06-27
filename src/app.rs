use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/test" view=|cx| view! { cx, <TestPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <div class="flex items-center">
            <div class="mx-auto">
                <h1 class="mx-auto">"Welcome to Leptos!"</h1>
                <div class="flex justify-center">
                    <button class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" on:click=on_click>"Click Me: " {count}</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TestPage(cx: Scope) -> impl IntoView {

    view!{ cx,
        <h1>"Test Page, This means routing works!"</h1>
    }
}