use crate::model::conversation::Conversation;
use leptos::*;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use llm::models::Llama;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    })
    .await
    .unwrap();

    use llm::KnownModel;
    let character_name = "### Assistant";
}
