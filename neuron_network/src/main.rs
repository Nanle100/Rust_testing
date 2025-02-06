// fn main() {
//     println!("Hello, world!");w
// }

use actix_web::{post, get, web, App, HttpServer, Responder, HttpResponse};
use tokio::sync::Mutex;
use std::sync::Arc;
use rocket::{post as rocket_post, routes, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct EvaluationRequest {
    question: String,
    ai_response: String,
    expected_answer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EvaluationResult {
    accuracy_score: f32,
    clarity_score: f32,
    relevance_score: f32,
    final_score: f32,
}

struct AppState {
    evaluations: Mutex<HashMap<String, EvaluationResult>>,
}

#[post("/evaluate")]
async fn evaluate(data: web::Json<EvaluationRequest>, state: web::Data<Arc<AppState>>) -> impl Responder {
    let accuracy = 0.9;  // Placeholder logic
    let clarity = 0.85;
    let relevance = 0.95;
    let final_score = (accuracy + clarity + relevance) / 3.0;

    let result = EvaluationResult {
        accuracy_score: accuracy,
        clarity_score: clarity,
        relevance_score: relevance,
        final_score,
    };

    let mut evaluations = state.evaluations.lock().await;
    evaluations.insert(data.question.clone(), result.clone());

    HttpResponse::Ok().json(result)
}

#[get("/history")]
async fn history(state: web::Data<Arc<AppState>>) -> impl Responder {
    let evaluations = state.evaluations.lock().await;
    HttpResponse::Ok().json(&*evaluations)
}

#[rocket_post("/feedback")]
fn feedback(feedback: Json<EvaluationRequest>) -> Json<String> {
    Json(format!("Feedback received for question: {}", feedback.question))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(AppState {
        evaluations: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(evaluate)
            .service(history)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
