// Antigravity AI engine — integrates Google Antigravity (agy) CLI
// directly into Coucou's Mochi chat interface without requiring external API keys.

use std::sync::Mutex;
use serde::{Deserialize, Serialize};

pub const DEFAULT_MODEL: &str = "gemini-3.8-flash-high";
const MAX_INLINE_TEXT: usize = 200_000;

const SYSTEM_PROMPT: &str = "You are Mochi, a personal AI assistant living at the top of the user's screen. \
You have access to web search, file tools, and terminal commands to assist with research, writing, coding, creating documents, and any tasks. \
When asked to create a document, note, or file, write it and always provide a friendly, clear confirmation and preview/summary of its content in your response text so the user can read it in the chat. \
Always provide a complete text response. Respond in the user's language. \
Keep formatting clean and easy to read (plain text, simple lists, or code snippets; avoid heavy markdown header hashes like ###).";

#[derive(Default)]
pub struct Chat {
    pub conversation_id: Mutex<Option<String>>,
}

impl Chat {
    pub fn reset(&self) {
        *self.conversation_id.lock().unwrap() = None;
    }

    pub fn is_empty(&self) -> bool {
        self.conversation_id.lock().unwrap().is_none()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ChatContext {
    File { name: String, path: String },
    Window { app_name: String, title: String, url: Option<String> },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatReply {
    pub text: String,
}

#[derive(Deserialize)]
struct DeniedAction {
    action: Option<String>,
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct AgyResponse {
    conversation_id: Option<String>,
    #[allow(dead_code)]
    status: Option<String>,
    response: Option<String>,
    error: Option<String>,
    denied_actions: Option<Vec<DeniedAction>>,
}

fn find_agy_binary() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        let local_bin = std::path::PathBuf::from(home).join(".local/bin/agy");
        if local_bin.is_file() {
            return local_bin;
        }
    }
    for path in &["/usr/local/bin/agy", "/usr/bin/agy"] {
        let p = std::path::PathBuf::from(path);
        if p.is_file() {
            return p;
        }
    }
    std::path::PathBuf::from("agy")
}

fn normalize_model(model: &str) -> Option<&'static str> {
    match model {
        "gemini-3.8-flash-high" => Some("gemini-3.8-flash-high"),
        "gemini-3.8-flash-medium" => Some("gemini-3.8-flash-medium"),
        "gemini-3.8-flash-low" => Some("gemini-3.8-flash-low"),
        "gemini-3.7-flash-high" => Some("gemini-3.7-flash-high"),
        "gemini-3.7-flash-medium" => Some("gemini-3.7-flash-medium"),
        "gemini-3.7-flash-low" => Some("gemini-3.7-flash-low"),
        "gemini-3.6-flash-high" => Some("gemini-3.6-flash-high"),
        "gemini-3.1-pro-high" => Some("gemini-3.1-pro-high"),
        "claude-opus-5-5-high" | "claude-opus-5" => Some("claude-opus-5-5-high"),
        "claude-sonnet-5-5-high" | "claude-sonnet-5" => Some("claude-sonnet-5-5-high"),
        "claude-opus-5-5-low" => Some("claude-opus-5-5-low"),
        "claude-sonnet-5-5-low" => Some("claude-sonnet-5-5-low"),
        "gpt-oss-120b-medium" => Some("gpt-oss-120b-medium"),
        _ => None,
    }
}

/// One chat turn using Antigravity.
pub async fn send(
    chat: &Chat,
    model: &str,
    query: String,
    context: Option<ChatContext>,
) -> Result<ChatReply, String> {
    let mut full_prompt = String::new();

    let is_first_turn = chat.is_empty();
    if is_first_turn {
        full_prompt.push_str(&format!("[Instruction: {SYSTEM_PROMPT}]\n\n"));
    }

    if let Some(ctx) = context {
        match ctx {
            ChatContext::File { name, path } => {
                full_prompt.push_str(&format!("[Context: User dropped file \"{name}\" at path \"{path}\"]\n"));
                if let Ok(text) = std::fs::read_to_string(&path) {
                    if text.len() <= MAX_INLINE_TEXT {
                        full_prompt.push_str(&format!("File content:\n{text}\n\n"));
                    }
                }
            }
            ChatContext::Window { app_name, title, url } => {
                let mut text = format!("[Context: Active application \"{app_name}\", Window \"{title}\"");
                if let Some(url) = url {
                    text.push_str(&format!(", URL: {url}"));
                }
                text.push_str("]\n\n");
                full_prompt.push_str(&text);
            }
        }
    }

    full_prompt.push_str(&query);

    let agy_bin = find_agy_binary();
    let conv_id = chat.conversation_id.lock().unwrap().clone();
    let selected_model = normalize_model(model);

    let (stdout, stderr) = tokio::task::spawn_blocking(move || {
        let mut cmd = std::process::Command::new(agy_bin);
        if let Some(ref id) = conv_id {
            cmd.arg("--conversation").arg(id);
        }
        if let Some(m) = selected_model {
            cmd.arg("--model").arg(m);
        }
        cmd.arg("--dangerously-skip-permissions");
        cmd.arg("--output-format").arg("json");
        cmd.arg("-p").arg(&full_prompt);

        if let Ok(home) = std::env::var("HOME") {
            cmd.current_dir(home);
        }

        let output = cmd.output().map_err(|e| format!("Failed to run agy: {e}"))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Antigravity error: {stderr}"));
        }
        Ok((
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    })
    .await
    .map_err(|e| e.to_string())??;

    let json_str = if let (Some(start), Some(end)) = (stdout.find('{'), stdout.rfind('}')) {
        if start <= end {
            &stdout[start..=end]
        } else {
            &stdout
        }
    } else {
        &stdout
    };

    let parsed: AgyResponse = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse Antigravity response: {e}\nOutput: {stdout}"))?;

    if let Some(err) = parsed.error {
        if !err.is_empty() {
            return Err(err);
        }
    }

    let text = parsed.response.unwrap_or_default().trim().to_string();
    if text.is_empty() {
        if let Some(ref denied) = parsed.denied_actions {
            if !denied.is_empty() {
                let actions = denied
                    .iter()
                    .filter_map(|d| d.display_name.as_deref().or(d.action.as_deref()))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(format!("Aksi dibatasi: {actions}"));
            }
        }
        let stderr_trimmed = stderr.trim();
        if !stderr_trimmed.is_empty() {
            return Err(format!("Antigravity error: {stderr_trimmed}"));
        }
        return Err("No response received from Antigravity.".into());
    }

    if let Some(new_conv_id) = parsed.conversation_id {
        if !new_conv_id.is_empty() {
            *chat.conversation_id.lock().unwrap() = Some(new_conv_id);
        }
    }

    // Clean up basic markdown symbols so Coucou's plain-text bubbles display neatly
    let cleaned = text.replace("**", "").replace("### ", "").replace("## ", "");

    Ok(ChatReply { text: cleaned })
}

/// Small standalone base64 encoder — not worth another dependency.
/// Also used for Stripe's basic auth.
pub(crate) fn base64_for(bytes: &[u8]) -> String {
    base64(bytes)
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn base64_matches_rfc4648_vectors() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foob"), "Zm9vYg==");
        assert_eq!(base64(b"fooba"), "Zm9vYmE=");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
