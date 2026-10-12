// Chat view — DOM port of PromptView / ChatBubble / TypingDotsView from
// IslandViewContent.swift.

import { h, svg, clear } from "./dom";
import { ICONS } from "./icons";
import { Bridge, type ChatContext } from "../core/bridge";
import { Sound } from "../core/sound";
import { State, type ChatMessage } from "../core/state";
import type { ViewHost, ViewActions } from "./views";
import type { BotEmoteName } from "../core/layout";

let nextId = 1;

function renderFormattedReply(text: string): HTMLElement {
  const el = h("div", { class: "reply-content" });
  const parts = text.split(/(```[\s\S]*?```)/g);
  for (const part of parts) {
    if (part.startsWith("```") && part.endsWith("```")) {
      const inner = part.slice(3, -3);
      const firstLineEnd = inner.indexOf("\n");
      let code = inner;
      let lang = "";
      if (firstLineEnd !== -1) {
        lang = inner.slice(0, firstLineEnd).trim();
        code = inner.slice(firstLineEnd + 1);
      }
      const codeBlock = h(
        "pre",
        { class: "code-block" },
        lang ? h("span", { class: "code-lang", text: lang }) : "",
        h("code", { text: code.trim() }),
      );
      el.append(codeBlock);
    } else if (part.trim()) {
      el.append(h("span", { class: "reply-text", text: part }));
    }
  }
  return el;
}

function bubble(message: ChatMessage): HTMLElement {
  if (message.role === "user") {
    return h(
      "div",
      { class: "chat-row user" },
      h("div", { class: "bubble", text: message.content }),
    );
  }
  return h(
    "div",
    { class: "chat-row assistant" },
    h("div", { class: "reply-bubble" }, renderFormattedReply(message.content)),
  );
}

function typingDots(): HTMLElement {
  return h(
    "div",
    { class: "chat-row assistant typing-row" },
    h("div", { class: "typing" }, h("i"), h("i"), h("i")),
  );
}

/** The coloured chip showing what the question is about (a dropped file). */
function contextChip(label: string): HTMLElement {
  const chip = h("div", { class: "chip" }, h("i", { class: "chip-dot" }), h("span", { text: label }));
  requestAnimationFrame(() => chip.classList.add("settled"));
  return chip;
}

export function buildPrompt(onHeightChange: () => void, actions?: ViewActions): ViewHost {
  const chipRow = h("div", { class: "chip-row" });
  const log = h("div", { class: "chat-log" });
  const input = h("input", {
    type: "text",
    class: "chat-input",
    placeholder: "Ask me anything…",
    spellcheck: "false",
  }) as HTMLInputElement;
  const send = h("button", { class: "send-btn", title: "Send" }, svg(ICONS.arrowUp, 11));
  const bar = h("div", { class: "chat-bar" }, input, send);

  const el = h(
    "div",
    { class: "view" },
    h("div", { class: "card wash chat-card" }, h("div", { class: "chat-body" }, chipRow, log, bar)),
  );
  (el.querySelector(".card") as HTMLElement).style.setProperty("--wash", "rgba(99,102,241,0.5)");

  let sending = false;
  let renderedCount = -1;

  async function submit() {
    const query = input.value.trim();
    if (!query || sending) return;
    input.value = "";

    if (query === "/clear" || query === "/reset") {
      State.chatHistory = [];
      State.promptContext = null;
      State.droppedFile = null;
      void Bridge.chatReset();
      Sound.play("finish");
      State.notify();
      onHeightChange();
      return;
    }

    sending = true;
    Sound.play("send");

    State.chatHistory.push({ id: nextId++, role: "user", content: query });
    State.stateOverride = "thinking";
    State.triggerEmote("surprised");
    State.notify();
    onHeightChange();

    const file = State.droppedFile;
    const context: ChatContext | null =
      State.chatHistory.length === 1 && file ? { kind: "file", name: file.name, path: file.path } : null;

    try {
      const reply = await Bridge.chatSend(query, context);
      State.chatHistory.push({ id: nextId++, role: "assistant", content: reply.text });
      State.stateOverride = null;
      const joys: BotEmoteName[] = ["happy", "wink", "proud"];
      const chosen = joys[Math.floor(Math.random() * joys.length)];
      State.triggerEmote(chosen);
      Sound.play("finish");
    } catch (err) {
      State.stateOverride = null;
      State.triggerEmote("annoyed");
      State.noteMessage = String(err).replace(/^Error:\s*/, "");
      State.view = "note";
      Sound.play("error");
    } finally {
      sending = false;
      State.notify();
      onHeightChange();
      input.focus();
    }
  }

  send.addEventListener("click", () => void submit());
  input.addEventListener("keydown", (e) => {
    if ((e as KeyboardEvent).key === "Enter") {
      e.preventDefault();
      void submit();
      return;
    }
    if ((e as KeyboardEvent).key === "Escape") {
      e.preventDefault();
      actions?.collapse();
      return;
    }
    e.stopPropagation();
  });

  return {
    el,
    sync() {
      const file = State.droppedFile;
      const wantChip = file?.name ?? "";
      if (chipRow.dataset.label !== wantChip) {
        chipRow.dataset.label = wantChip;
        clear(chipRow);
        if (wantChip) chipRow.append(contextChip(wantChip));
      }

      const thinking = State.stateOverride === "thinking";
      const count = State.chatHistory.length + (thinking ? 0.5 : 0);
      if (count !== renderedCount) {
        renderedCount = count;
        clear(log);
        for (const m of State.chatHistory) log.append(bubble(m));
        if (thinking) log.append(typingDots());
        log.scrollTop = log.scrollHeight;
      }

      input.placeholder = State.chatHistory.length === 0 ? "Ask me anything…" : "Continue…";
      input.disabled = sending;
    },
    focus() {
      input.focus();
      input.select();
    },
  };
}
