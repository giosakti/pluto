use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct AgentsResponse {
    agents: Vec<Agent>,
}

#[derive(Serialize)]
pub struct Agent {
    // Placeholder for future agent fields
}

pub async fn list_agents() -> Json<AgentsResponse> {
    Json(AgentsResponse { agents: vec![] })
}
