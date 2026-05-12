/// OpenAI-compatible proxy endpoints
///
/// Exposes:
/// - POST /v1/chat/completions  — transparent proxy to AtomGit LLM gateway
/// - POST /v1/completions       — transparent proxy (legacy)
/// - GET  /v1/models            — locally-configured models in OpenAI format
/// - POST /v1/responses         — Responses API compatible proxy (used by Codex)
///
/// All handlers automatically inject the OAuth token from ~/.atomcode/auth.toml
/// (with auto-refresh), so callers can use any api_key value.
use axum::{
    body::Body,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use futures::StreamExt;
use serde_json::json;

const GATEWAY_BASE: &str = "https://api-ai.gitcode.com/v1";

fn build_client(timeout_secs: u64) -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .user_agent(atomcode_core::ATOMCODE_USER_AGENT)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

fn error_response(status: StatusCode, msg: impl std::fmt::Display) -> Response {
    let body = format!("{{\"error\":{{\"message\":\"{msg}\"}}}}");
    (status, [(header::CONTENT_TYPE, "application/json")], body).into_response()
}

// ============================================================================
// Chat Completions proxy (transparent)
// ============================================================================

/// POST /v1/chat/completions — transparent proxy to AtomGit LLM gateway.
pub(crate) async fn proxy_chat_completions(
    _headers: HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    proxy_post("/chat/completions", body, 300).await
}

/// POST /v1/completions — transparent proxy (legacy completions endpoint).
pub(crate) async fn proxy_completions(
    _headers: HeaderMap,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    proxy_post("/completions", body, 300).await
}

/// GET /v1/models — return locally-configured AtomGit models in OpenAI format.
pub(crate) async fn proxy_models() -> impl IntoResponse {
    use atomcode_core::config::Config;
    let config_path = Config::default_path();
    let Ok(config) = Config::load(&config_path) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(json!({"error": {"message": "failed to load config"}})),
        )
            .into_response();
    };

    let models: Vec<serde_json::Value> = config
        .providers
        .values()
        .map(|p| {
            json!({
                "id": p.model,
                "object": "model",
                "created": 0,
                "owned_by": "atomgit"
            })
        })
        .collect();

    axum::Json(json!({ "object": "list", "data": models })).into_response()
}

// ============================================================================
// Responses API proxy (used by OpenAI Codex with wire_api = "responses")
//
// Converts between OpenAI Responses API ↔ Chat Completions API since the
// AtomGit gateway only supports /v1/chat/completions.
// ============================================================================

/// POST /v1/responses — Responses API compatible proxy.
pub(crate) async fn proxy_responses(
    _headers: HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    let req: serde_json::Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, e),
    };

    let is_stream = req.get("stream").and_then(|v| v.as_bool()).unwrap_or(false);
    let model = req
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("GLM-5.1")
        .to_string();

    // Convert Responses API request → Chat Completions request
    let mut chat_req = build_chat_from_responses(&req);
    chat_req["stream"] = json!(is_stream);

    let chat_body = match serde_json::to_vec(&chat_req) {
        Ok(b) => axum::body::Bytes::from(b),
        Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e),
    };

    let token = match atomcode_core::auth::get_valid_token() {
        Ok(t) => t,
        Err(e) => return error_response(StatusCode::UNAUTHORIZED, e),
    };

    let url = format!("{GATEWAY_BASE}/chat/completions");
    let upstream = match build_client(300)
        .post(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .body(chat_body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, e),
    };

    let http_status = upstream.status();
    if !http_status.is_success() {
        let axum_status =
            StatusCode::from_u16(http_status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
        let err_body = upstream.bytes().await.unwrap_or_default();
        return (axum_status, err_body).into_response();
    }

    if is_stream {
        stream_responses_sse(upstream, model)
    } else {
        let bytes = upstream.bytes().await.unwrap_or_default();
        let chat: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(json!({}));
        let resp = chat_to_responses_body(&chat, &model);
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            serde_json::to_string(&resp).unwrap_or_default(),
        )
            .into_response()
    }
}

/// Convert an OpenAI Responses API request into a Chat Completions request.
fn build_chat_from_responses(req: &serde_json::Value) -> serde_json::Value {
    let model = req.get("model").cloned().unwrap_or(json!("GLM-5.1"));
    let mut messages: Vec<serde_json::Value> = Vec::new();

    // `instructions` → system message
    if let Some(inst) = req.get("instructions").and_then(|v| v.as_str()) {
        if !inst.is_empty() {
            messages.push(json!({"role": "system", "content": inst}));
        }
    }

    // `input` → messages
    match req.get("input") {
        Some(serde_json::Value::String(s)) => {
            messages.push(json!({"role": "user", "content": s}));
        }
        Some(serde_json::Value::Array(items)) => {
            for item in items {
                let role = item
                    .get("role")
                    .and_then(|v| v.as_str())
                    .unwrap_or("user");
                let content = extract_text_content(item);
                messages.push(json!({"role": role, "content": content}));
            }
        }
        _ => {}
    }

    let mut chat = json!({ "model": model, "messages": messages });

    // Forward compatible fields
    for field in &["temperature", "top_p", "n", "stop", "presence_penalty", "frequency_penalty"] {
        if let Some(v) = req.get(field) {
            chat[field] = v.clone();
        }
    }
    if let Some(v) = req.get("max_output_tokens") {
        chat["max_tokens"] = v.clone();
    }

    chat
}

/// Extract plain text from a Responses API content field (string or parts array).
fn extract_text_content(item: &serde_json::Value) -> String {
    match item.get("content") {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Array(parts)) => parts
            .iter()
            .filter_map(|part| {
                let ty = part.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if ty == "input_text" || ty == "text" {
                    part.get("text").and_then(|v| v.as_str()).map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(""),
        _ => String::new(),
    }
}

/// Convert a Chat Completions response body → Responses API response body.
fn chat_to_responses_body(chat: &serde_json::Value, model: &str) -> serde_json::Value {
    let raw_id = chat
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("chatcmpl-unknown");
    let resp_id = raw_id.replace("chatcmpl-", "resp_");
    let msg_id = format!("msg_{}", resp_id.trim_start_matches("resp_"));

    let content = chat
        .pointer("/choices/0/message/content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let usage = chat.get("usage").cloned().unwrap_or(json!({}));
    let model_name = chat.get("model").and_then(|v| v.as_str()).unwrap_or(model);
    let now = unix_now();

    json!({
        "id": resp_id,
        "object": "response",
        "created_at": chat.get("created").and_then(|v| v.as_u64()).unwrap_or(now),
        "model": model_name,
        "status": "completed",
        "error": null,
        "incomplete_details": null,
        "output": [{
            "type": "message",
            "id": msg_id,
            "role": "assistant",
            "status": "completed",
            "content": [{"type": "output_text", "text": content, "annotations": []}]
        }],
        "usage": {
            "input_tokens": usage.get("prompt_tokens").and_then(|v| v.as_u64()).unwrap_or(0),
            "output_tokens": usage.get("completion_tokens").and_then(|v| v.as_u64()).unwrap_or(0),
            "total_tokens": usage.get("total_tokens").and_then(|v| v.as_u64()).unwrap_or(0),
            "input_tokens_details": {"cached_tokens": 0},
            "output_tokens_details": {"reasoning_tokens": 0}
        }
    })
}

/// Stream chat completions SSE → Responses API SSE events.
fn stream_responses_sse(upstream: reqwest::Response, model: String) -> Response {
    let resp_id = format!("resp_{}", uuid::Uuid::new_v4().simple());
    let msg_id = format!("msg_{}", uuid::Uuid::new_v4().simple());
    let created_at = unix_now();

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<axum::body::Bytes, std::io::Error>>(64);

    tokio::spawn({
        let resp_id = resp_id.clone();
        let msg_id = msg_id.clone();
        let model = model.clone();
        async move {
            // Send initial Responses API events
            let initial = build_initial_sse(&resp_id, &msg_id, &model, created_at);
            if tx.send(Ok(axum::body::Bytes::from(initial))).await.is_err() {
                return;
            }

            // Read upstream chat completions SSE and forward as delta events
            let mut full_text = String::new();
            let mut pending = String::new();
            let mut byte_stream = upstream.bytes_stream();

            'outer: while let Some(chunk) = byte_stream.next().await {
                let bytes = match chunk {
                    Ok(b) => b,
                    Err(_) => break,
                };
                pending.push_str(&String::from_utf8_lossy(&bytes));

                loop {
                    match pending.find('\n') {
                        None => break,
                        Some(pos) => {
                            let line = pending[..pos].trim().to_string();
                            pending = pending[pos + 1..].to_string();

                            if let Some(data) = line.strip_prefix("data: ") {
                                if data.trim() == "[DONE]" {
                                    break 'outer;
                                }
                                if let Ok(val) =
                                    serde_json::from_str::<serde_json::Value>(data)
                                {
                                    let delta = val
                                        .pointer("/choices/0/delta/content")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    if !delta.is_empty() {
                                        full_text.push_str(delta);
                                        let event = format!(
                                            "event: response.output_text.delta\ndata: {}\n\n",
                                            json!({
                                                "type": "response.output_text.delta",
                                                "item_id": msg_id,
                                                "output_index": 0,
                                                "content_index": 0,
                                                "delta": delta
                                            })
                                        );
                                        if tx
                                            .send(Ok(axum::body::Bytes::from(event)))
                                            .await
                                            .is_err()
                                        {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Send done events
            let done = build_done_sse(&resp_id, &msg_id, &model, &full_text, created_at);
            let _ = tx.send(Ok(axum::body::Bytes::from(done))).await;
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    let body = Body::from_stream(stream);
    let mut response = Response::new(body);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream; charset=utf-8"),
    );
    response
        .headers_mut()
        .insert("cache-control", HeaderValue::from_static("no-cache"));
    response
}

fn build_initial_sse(resp_id: &str, msg_id: &str, model: &str, created_at: u64) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "event: response.created\ndata: {}\n\n",
        json!({
            "type": "response.created",
            "response": {
                "id": resp_id, "object": "response", "created_at": created_at,
                "status": "in_progress", "model": model,
                "output": [], "error": null, "incomplete_details": null, "usage": null
            }
        })
    ));

    out.push_str(&format!(
        "event: response.output_item.added\ndata: {}\n\n",
        json!({
            "type": "response.output_item.added",
            "output_index": 0,
            "item": {
                "type": "message", "id": msg_id,
                "role": "assistant", "status": "in_progress", "content": []
            }
        })
    ));

    out.push_str(&format!(
        "event: response.content_part.added\ndata: {}\n\n",
        json!({
            "type": "response.content_part.added",
            "item_id": msg_id, "output_index": 0, "content_index": 0,
            "part": {"type": "output_text", "text": "", "annotations": []}
        })
    ));

    out
}

fn build_done_sse(resp_id: &str, msg_id: &str, model: &str, full_text: &str, created_at: u64) -> String {
    let mut out = String::new();
    let content_part = json!({
        "type": "output_text", "text": full_text, "annotations": []
    });

    out.push_str(&format!(
        "event: response.output_text.done\ndata: {}\n\n",
        json!({
            "type": "response.output_text.done",
            "item_id": msg_id, "output_index": 0, "content_index": 0,
            "text": full_text
        })
    ));

    out.push_str(&format!(
        "event: response.content_part.done\ndata: {}\n\n",
        json!({
            "type": "response.content_part.done",
            "item_id": msg_id, "output_index": 0, "content_index": 0,
            "part": content_part
        })
    ));

    let output_item = json!({
        "type": "message", "id": msg_id, "role": "assistant", "status": "completed",
        "content": [{"type": "output_text", "text": full_text, "annotations": []}]
    });

    out.push_str(&format!(
        "event: response.output_item.done\ndata: {}\n\n",
        json!({
            "type": "response.output_item.done",
            "output_index": 0, "item": output_item
        })
    ));

    out.push_str(&format!(
        "event: response.completed\ndata: {}\n\n",
        json!({
            "type": "response.completed",
            "response": {
                "id": resp_id, "object": "response", "created_at": created_at,
                "status": "completed", "model": model,
                "output": [{"type": "message", "id": msg_id, "role": "assistant",
                    "status": "completed",
                    "content": [{"type": "output_text", "text": full_text, "annotations": []}]}],
                "error": null, "incomplete_details": null, "usage": null
            }
        })
    ));

    out
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ============================================================================
// Shared helpers
// ============================================================================

/// Forward a POST body to the gateway and stream the response back.
async fn proxy_post(path: &str, body: axum::body::Bytes, timeout_secs: u64) -> Response {
    let token = match atomcode_core::auth::get_valid_token() {
        Ok(t) => t,
        Err(e) => return error_response(StatusCode::UNAUTHORIZED, e),
    };

    let url = format!("{GATEWAY_BASE}{path}");
    let upstream = match build_client(timeout_secs)
        .post(&url)
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => return error_response(StatusCode::BAD_GATEWAY, e),
    };

    build_streaming_response(upstream)
}

/// Turn a reqwest::Response into an axum Response, streaming the body.
fn build_streaming_response(upstream: reqwest::Response) -> Response {
    let http_status = upstream.status();
    let axum_status =
        StatusCode::from_u16(http_status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    let content_type = upstream
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/json")
        .to_string();

    let stream = upstream
        .bytes_stream()
        .map(|r| r.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
    let body = Body::from_stream(stream);

    let mut response = Response::new(body);
    *response.status_mut() = axum_status;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/json")),
    );
    response
}
