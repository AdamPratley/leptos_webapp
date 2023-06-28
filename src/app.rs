use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Test Leptos Webapp"/>

        // content for this welcome page
        <Router>
            <NavigationBar>
            </NavigationBar>
            <main>
                <body class="bg-black">
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                        <Route path="/test" view=|cx| view! { cx, <TestPage/> }/>
                    </Routes>
                </body>
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
                <h1 class="mx-auto text-5xl text-amber-600">"Welcome to Web App!"</h1>
                <div class="flex justify-center mt-4 bg-black">
                    <button class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg" on:click=on_click>"Click Me: " {count}</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TestPage(cx: Scope) -> impl IntoView {

    view!{ cx,
        <h1 clas="text-white">"Test Page, This means routing works!"</h1>
    }
}

#[component]
fn NavigationBar(cx: Scope) -> impl IntoView {
    
    view!{cx, 
     <div class="max-auto flex h-[5.5rem] max-w-[1344px] w-full px-5 sm:px-10 bg-black items-center">
        <a href="/" class="text-lg text-white">"Web App :)"</a>
        <div class="flex flex-col gap-4 xl:flex-row xl:gap-8 text-slate-400 mx-10">
            <a href="/test">"Route1"</a>
            <a href="/test">"Route2"</a>
        </div>
     </div>
    }
}