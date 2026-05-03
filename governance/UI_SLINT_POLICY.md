# UI Slint Policy

This sandbox inherits the active Python UI doctrine that presentation is not the source of truth.

Slint is the active presentation technology for this Rust sandbox shell.

## Rules

- business logic must not live inside Slint views
- filesystem logic must not live inside Slint views
- runtime logic must not live inside Slint views
- Slint must consume i18n and theme layers rather than reimplement them
- `ui_slint` consumes `ui_core`, `ui_i18n`, `ui_theme`, and thin controller-facing runtime boundaries without becoming their owner
- `app_services` must remain independent from Slint widgets
- Slint must not execute tools or external workflows directly
- Slint must not couple directly to LLM providers
- Slint routes and renders; controllers decide; governed runtime layers remain authoritative
- the viewer remains readonly regardless of whether a workflow starts from tree or clip
- workspace tabs are controlled content views, not miniapps or a parallel runtime
- Lume onboarding/help surfaces may be presented by Slint, but their visible text must come from i18n resources
- Slint may represent `GUI_OBJECTS_v1` inside Lume Help, but canonical GUI names and visible help text must come from `resources/help/gui_objects.json` and i18n resources
- Slint must not invent GUI object names, hardcode GUI help text, interpret Lume Help, or execute anything through Lume Help
- Slint must not execute Lume, tools, LLM workflows, external binaries, or sandbox filesystem actions directly
- the DocGraph icon may be rendered in top chrome
- its click handler may only emit `OpenLumeHelpRequested` or equivalent navigation intent
- UI must not hardcode runtime paths or execute logic from the icon
- icon visible text or tooltip must come from i18n

## Current scope

- live shell window with explicit controllers
- governed document tree for existing project documents
- explicit clip surface for document intake and workflow launch
- structured chat surface for text, document references, tool results, and state messages
- tabbed workspace content area
- technical readonly viewer as one workspace tab
- knowledge panel integrated into the shell
- tools panel integrated into the shell
- Lume Help GUI Objects section as readonly presentation of canonical names
- no direct product-runtime ownership

## Current role boundaries

- Slint presents the shell and routes user intent into controllers
- controllers translate UI intent into governed runtime calls and presentation state
- tree actions reference and open existing governed documents; they do not import them again
- clip actions intake external files or launch explicit workflows; they do not replace the tree
- chat stores references and results; it does not become a hidden document store
- workspace tabs host controlled views; they do not define new domain pipelines
- the viewer verifies resolved content readonly; it does not become an editor or extractor

## Future governed editing proposal

In a later phase, Slint may capture viewer selection, expose a contextual instruction popup, and render patch previews or accept/reject/regenerate choices.

Slint must not:

- generate patches
- validate patches
- apply patches
- modify the filesystem
- execute LLM calls directly
- execute tools directly

The future UI role is to collect intent and render governed state supplied by controllers or runtime boundaries.
