# chat_input

## Status

PROPOSAL / F9-ready declarative governance.

No runtime implementation.

## Purpose

Define the governed `Chat Input` surface for DocGraph.

The input is keyboard-first, compact, controlled multiline, compatible with long text, and extensible with future voice dictation without changing the rule that chat is intent capture rather than execution.

## Core rule

`Chat Input` captures text intent.

It does not execute commands.
It does not resolve targets implicitly.
It does not call LLM directly.
It does not contain domain logic.

## Keyboard behavior

### Enter

- `Enter` sends the message

Rules:

- `Enter` only sends text currently present in the input
- `Enter` does not execute actions directly
- `Enter` is send-intent only

### Shift+Enter

- `Shift+Enter` inserts a new line

### Ctrl+Enter

- `Ctrl+Enter` may be supported as optional send shortcut for advanced users

## Send button

A send button may be present as fallback.

It is not the primary interaction path.

It must not introduce additional logic beyond the same governed submit intent.

## Multiline input

The input is a multiline textarea.

Rules:

- controlled auto-grow
- maximum height around 5-6 lines
- after the limit, internal scroll is required
- infinite vertical growth is prohibited
- the input must not push the main layout indefinitely

## Long text behavior

Pasting large text must not break UI layout.

Rules:

- internal scroll is required
- `char_count` may be shown optionally
- collapsed preview for very large text may exist optionally

Large text remains editable text input, not a command surface.

## Microphone / dictation

A microphone button may be visible in `Chat Input`.

Interaction:

- first click starts recording
- second click stops recording

## Microphone states

- `idle`
- `recording`
- `transcribing`
- `transcription_ready`
- `mic_unavailable`
- `error`

## Audio activity visualization

Audio activity may be shown as compact waveform or bar visualization.

Rules:

- render it outside the textarea
- do not mix audio activity visual with editable text content
- keep it compact

## Transcription

Audio produces text.

Rules:

- transcription result is inserted into the input
- result remains editable before send
- transcription is never sent automatically
- chat works with text, not audio
- voice does not execute actions

## Availability

If no audio input or STT is available:

- state becomes `mic_unavailable`
- normal text input remains fully available

Voice capability absence must not block keyboard input.

## Conceptual model

### ChatInputDraft

Minimum conceptual fields:

- `raw_text`
- `line_count`
- `char_count`
- `source`
- `has_overflow`
- `submitted_at`, future optional

Field meaning:

- `raw_text`: current editable text
- `line_count`: current visible or logical line count
- `char_count`: text length metric
- `source`: `keyboard` or `voice`
- `has_overflow`: whether input is currently using internal scroll
- `submitted_at`: future timestamp for submit trace when needed

## Input states

Prepared input state may include:

- `idle`
- `typing`
- `multiline`
- `overflow_scroll`
- `recording`
- `transcribing`
- `transcription_ready`
- `mic_unavailable`
- `submitting`
- `error`

Interpretation:

- `idle`: no active input interaction
- `typing`: user is entering text
- `multiline`: content spans more than one line but remains within controlled height
- `overflow_scroll`: content exceeds height limit and uses internal scroll
- `recording`: microphone is actively capturing audio
- `transcribing`: audio is being converted to text in future governed flow
- `transcription_ready`: text from voice capture is ready and editable
- `mic_unavailable`: voice capture cannot be used
- `submitting`: governed send intent is being prepared
- `error`: capture or submit preparation failed safely

## Non-goals

- no command execution
- no implicit target resolution
- no audio persistence by default
- no direct LLM invocation
- no direct STT provider integration

## Failure modes

- `empty_submit`
- `input_too_large_future`
- `mic_permission_denied`
- `mic_unavailable`
- `transcription_failed`
- `submit_while_recording_blocked`

Interpretation:

- `empty_submit`: send was requested with no meaningful text
- `input_too_large_future`: future size policy blocks current input
- `mic_permission_denied`: microphone permission was denied
- `mic_unavailable`: recording or STT capability is unavailable
- `transcription_failed`: voice-to-text failed safely
- `submit_while_recording_blocked`: send is blocked while recording is still active

## Invariants

- `INV-CINPUT-001`: `Enter` MUST prepare send intent only; it MUST NOT execute actions
- `INV-CINPUT-002`: `Shift+Enter` MUST insert newline, not send
- `INV-CINPUT-003`: multiline growth MUST remain bounded
- `INV-CINPUT-004`: internal overflow scroll MUST protect the main layout from unlimited growth
- `INV-CINPUT-005`: voice transcription MUST remain editable before send
- `INV-CINPUT-006`: transcribed text MUST NOT be sent automatically
- `INV-CINPUT-007`: absence of mic or STT MUST NOT block normal text input
- `INV-CINPUT-008`: chat input MUST remain an intent-capture surface, not an execution surface

## Related specs

- `docs/specs/chat_document_flows.md`
- `docs/specs/ui_core.md`
