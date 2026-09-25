# ollama-harness-rs

A terminal UI for chatting with local [Ollama](https://ollama.com) models, built with `ratatui` + `tokio`.

I've built somewhat working harness for local ollama models using Ink and React, but it was painfully laggy, so I wanted to rebuild it via rust. So this repo is a rewrite of an earlier Ink/React version (see [`reference/create-assistant.tsx`](reference/create-assistant.tsx) for the reference implementation of the create-assistant flow). It's around 80% manual coding and 20% AI assisted, the purpose of the project is to get comfortable with rust. The project is unfinished because I discovered LMStudio and OpenCode :) This one was fun to build, I liked working on controls and command or file autocompletes. rust is fun!   

## Demo

https://github.com/user-attachments/assets/05ee614e-35e9-4f7d-b521-0bf09f9cf234


## Running

Requires a running Ollama server on the default address (`http://localhost:11434`) with at least one model pulled.

```bash
cargo run
```

Quick start: `/create-assistant` → pick a model → point it at a prompt from `system_prompts/` → `/select-assistant` → chat.

## What works

**Chat screen**
- Vim-style modes: **Normal** (`e` to edit, `q` to quit, `↑`/`↓` to scroll messages) and **Editing** (`Esc` to leave, `Enter` to submit).
- Text input with cursor movement (`←`/`→`), backspace, and horizontal scrolling for long input.
- With an active assistant, messages are sent to the model with the assistant's system prompt and the chat history, and the full reply is rendered (no streaming yet). A `Thinking…` bubble shows while waiting; errors (e.g. Ollama not running) show up in the chat. Reasoning / `<think>` blocks are hidden.
- The input border turns green and shows the active assistant's name and model.
- Debug side panel showing log output (toggle via `USE_DEBUG` in `src/shared/constants.rs`).

**Autocomplete** (shown under the input)
- `/` at the start of the message → command suggestions.
- `@` anywhere → filesystem path suggestions (supports `~` and relative paths).
- `↑`/`↓` to pick, `Tab` to accept.

**Commands**
- `/list-models`: lists local Ollama models in the chat.
- `/create-assistant`: create an assistant (see below).
- `/select-assistant`: pick an assistant to chat with. Loads its system prompt and clears the chat.
- `/remove-assistant`: `Space` to mark one or more assistants, `Enter` to delete their configs.

`Esc` closes any command window.

**Create Assistant journey**
1. Name: text input
2. Model: select list of local models
3. System prompt path: text input with file autocomplete
4. Use RAG: `Y`/`N` (anything else is rejected)

Answered steps are shown as a history above the current step. On the last step the assistant is saved to `harness_configs/<name>.yml` (existing files are never overwritten):

```yaml
name: pirate
model: gemma4:e4b-mlx
system_prompt_path: system_prompts/general_purpose.md
use_rag: false
```

`system_prompts/` has a couple of sample prompts.

## Where I left off

**Next up**
- [ ] Show replies as they're being written instead of waiting for the whole thing. Right now the app just hangs until the model is done.
- [ ] Finish the RAG part.
- [ ] Check that the system prompt file actually exists while creating an assistant, not only when selecting it.
- [ ] Make `@file` mentions in a message actually send the file to the model.

**Known rough edges**
- The create form keeps two copies of the input state (wrapper + form), synced by hand in both directions. Should be one.
- `Tab` in the system prompt step inserts an `@` prefix into the path (stripped again when loading).
- `detect_token` in `src/widgets/input.rs` slices the input by char index as if it were a byte index, so non-ASCII text can panic.
- A few `unwrap()`s that can panic: `src/services/file_explorer.rs` (dir entries, non-UTF-8 paths) and `src/widgets/input_label.rs` (`$HOME` lookup).

## Layout

```
src/
  main.rs                    App state, event loop, command dispatch
  shared/                    State types (input, autocomplete, window, commands, messages,
                             assistant config, logger)
  services/
    assistant_configs        Save / load / remove assistant YAML configs, read system prompts
    chat                     Send the conversation to Ollama, strip <think> blocks
    file_explorer            Directory listing for @-autocomplete
  widgets/                   Input, input label/autocomplete, message list, select,
                             debug block, generic journey wrapper, create / select /
                             remove assistant journeys
system_prompts/              Sample system prompts
harness_configs/             Saved assistants (gitignored)
```
