# ollama-harness-rs

A terminal UI for chatting with local [Ollama](https://ollama.com) models, built with `ratatui` + `tokio` + `ollama-rs`.

I've built somewhat working harness for local ollama models using Ink and React, but it was painfully laggy, so I wanted to rebuild it via rust. So this repo is a rewrite of an earlier Ink/React version (see [`reference/create-assistant.tsx`](reference/create-assistant.tsx) for the reference implementation of the create-assistant flow). It's around 80% manual coding and 20% AI assisted, the purpose of the project is to get comfortable with rust. The project is unfinished because I discovered LMStudio and OpenCode :) This one was fun to build, I liked working on controls and command or file autocompletes. rust is fun!   

## Demo

https://github.com/user-attachments/assets/05ee614e-35e9-4f7d-b521-0bf09f9cf234


## Running

Requires a running Ollama server on the default address (`http://localhost:11434`).

```bash
cargo run
```

## What works

**Chat screen**
- Vim-style modes: **Normal** (`e` to edit, `q` to quit, `↑`/`↓` to scroll messages) and **Editing** (`Esc` to leave, `Enter` to submit).
- Text input with cursor movement (`←`/`→`) and backspace.
- Submitted text is appended to the message list (scrollable, wrapped).
- Debug side panel showing log output (toggle via `USE_DEBUG` in `src/shared/constants.rs`).

**Autocomplete** (shown under the input)
- `/` at the start of the message → command suggestions.
- `@` anywhere → filesystem path suggestions (supports `~`).
- `↑`/`↓` to pick, `Tab` to accept.

**Commands**
- `/list-models`: lists local Ollama models in the chat.
- `/create-assistant`: opens the Create Assistant journey (see below). `Esc` closes it.

**Create Assistant journey** (partially done)
1. Name: text input ✅
2. Model: select list of local models ✅
3. System prompt path: text input with `@`-style file autocomplete ✅
4. Use RAG: Yes/No text input ✅ (value is recorded, but the flow stops here)

Answered steps are shown as a history above the current step.

## Where I left off

Last commit: `wip assistant form`, in the middle of the Create Assistant journey.

**Next up**
- [ ] Steps after "Use RAG": `EmbeddingModel` and `RagFilesPath` (the enum variants and state fields already exist in `src/widgets/create_assistant_form.rs`, but nothing handles or renders them yet). Skip them when RAG is "No".
- [ ] `Confirmation` / `Completed` steps: show a summary, then actually save the assistant somewhere.
- [ ] Maybe swap the Yes/No text input for the `SelectWidget`.
- [ ] Return to the chat screen when the journey finishes.

**Not started yet**
- [ ] Sending chat messages to a model. User messages are currently only stored locally, and nothing calls Ollama for a response. (The `stream` feature of `ollama-rs` is already enabled.)
- [ ] Choosing an active model/assistant for the chat.
- [ ] Using the `@file` references in prompts (they autocomplete but aren't read).

**Known rough edges**
- `SelectState::next/previous` will panic on an empty model list (`items.len() - 1` underflow), e.g. if Ollama isn't running.
- A few `unwrap()`s that can panic: `src/services/file_explorer.rs` (dir entries, non-UTF-8 paths) and `src/widgets/input_label.rs` (`$HOME` lookup).

## Layout

```
src/
  main.rs                  App state, event loop, command dispatch
  shared/                  State types (input, autocomplete, window, commands, messages, logger)
  services/file_explorer   Directory listing for @-autocomplete
  widgets/                 Input, input label/autocomplete, message list, select,
                           debug block, generic journey wrapper, create-assistant form
```
