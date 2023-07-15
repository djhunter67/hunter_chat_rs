use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::conversation::Conversation;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let mut user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.message.push(user_message);
        });
        // TODO converse
    });

    view! { cx,
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

            // sets the document title
            <Title text="Hunter chat_rs"/>
            <ChatArea conversation/>
            <TypeArea send/>

    }
}
