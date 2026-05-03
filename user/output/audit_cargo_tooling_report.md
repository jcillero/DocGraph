# Cargo Tooling Audit Report

- Generated at: `2026-05-02 13:10:08`
- Workspace root: `.`
- Report path: `user/output/audit_cargo_tooling_report.md`
- Deep mode: `no`

---

## cargo tree

- Status: `OK`
- Command: `cargo tree --workspace`
- Exit code: `0`

### stdout

```text
action_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\action_core)
Ôö£ÔöÇÔöÇ serde v1.0.228
Ôöé   Ôö£ÔöÇÔöÇ serde_core v1.0.228
Ôöé   ÔööÔöÇÔöÇ serde_derive v1.0.228 (proc-macro)
Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106
Ôöé       Ôöé   ÔööÔöÇÔöÇ unicode-ident v1.0.24
Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45
Ôöé       Ôöé   ÔööÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé       ÔööÔöÇÔöÇ syn v2.0.117
Ôöé           Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé           Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé           ÔööÔöÇÔöÇ unicode-ident v1.0.24
ÔööÔöÇÔöÇ serde_json v1.0.149
    Ôö£ÔöÇÔöÇ itoa v1.0.18
    Ôö£ÔöÇÔöÇ memchr v2.8.0
    Ôö£ÔöÇÔöÇ serde_core v1.0.228
    ÔööÔöÇÔöÇ zmij v1.0.21

app_services v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\app_services)
Ôö£ÔöÇÔöÇ project_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\project_runtime)
Ôöé   Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ sha2 v0.10.9
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ cpufeatures v0.2.17
Ôöé   Ôöé       ÔööÔöÇÔöÇ digest v0.10.7
Ôöé   Ôöé           Ôö£ÔöÇÔöÇ block-buffer v0.10.4
Ôöé   Ôöé           Ôöé   ÔööÔöÇÔöÇ generic-array v0.14.7
Ôöé   Ôöé           Ôöé       ÔööÔöÇÔöÇ typenum v1.20.0
Ôöé   Ôöé           Ôöé       [build-dependencies]
Ôöé   Ôöé           Ôöé       ÔööÔöÇÔöÇ version_check v0.9.5
Ôöé   Ôöé           ÔööÔöÇÔöÇ crypto-common v0.1.7
Ôöé   Ôöé               Ôö£ÔöÇÔöÇ generic-array v0.14.7 (*)
Ôöé   Ôöé               ÔööÔöÇÔöÇ typenum v1.20.0
Ôöé   Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
Ôöé   Ôö£ÔöÇÔöÇ spec_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\spec_runtime)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core)
Ôöé   Ôöé       ÔööÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôöé   ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)
ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)

cli_app v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\cli_app)
Ôö£ÔöÇÔöÇ app_services v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\app_services) (*)
Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôö£ÔöÇÔöÇ tool_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\tool_runtime)
Ôöé   Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
Ôöé   ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)
ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)

core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)

document_text_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\document_text_runtime)
Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôö£ÔöÇÔöÇ lopdf v0.32.0
Ôöé   Ôö£ÔöÇÔöÇ chrono v0.4.44
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ libm v0.2.16
Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ autocfg v1.5.0
Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôö£ÔöÇÔöÇ encoding_rs v0.8.35
Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôö£ÔöÇÔöÇ flate2 v1.1.9
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ crc32fast v1.5.0
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   ÔööÔöÇÔöÇ miniz_oxide v0.8.9
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ adler2 v2.0.1
Ôöé   Ôöé       ÔööÔöÇÔöÇ simd-adler32 v0.3.9
Ôöé   Ôö£ÔöÇÔöÇ itoa v1.0.18
Ôöé   Ôö£ÔöÇÔöÇ linked-hash-map v0.5.6
Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôö£ÔöÇÔöÇ md5 v0.7.0
Ôöé   Ôö£ÔöÇÔöÇ nom v7.1.3
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ memchr v2.8.0
Ôöé   Ôöé   ÔööÔöÇÔöÇ minimal-lexical v0.2.1
Ôöé   Ôö£ÔöÇÔöÇ rayon v1.12.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ either v1.15.0
Ôöé   Ôöé   ÔööÔöÇÔöÇ rayon-core v1.13.0
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ crossbeam-deque v0.8.6
Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ crossbeam-epoch v0.9.18
Ôöé   Ôöé       Ôöé   Ôöé   ÔööÔöÇÔöÇ crossbeam-utils v0.8.21
Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ crossbeam-utils v0.8.21
Ôöé   Ôöé       ÔööÔöÇÔöÇ crossbeam-utils v0.8.21
Ôöé   Ôö£ÔöÇÔöÇ time v0.3.47
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ deranged v0.5.8
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ powerfmt v0.2.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ itoa v1.0.18
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-conv v0.2.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ powerfmt v0.2.0
Ôöé   Ôöé   ÔööÔöÇÔöÇ time-core v0.1.8
Ôöé   ÔööÔöÇÔöÇ weezl v0.1.12
Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
ÔööÔöÇÔöÇ zip v0.6.6
    Ôö£ÔöÇÔöÇ byteorder v1.5.0
    Ôö£ÔöÇÔöÇ crc32fast v1.5.0 (*)
    ÔööÔöÇÔöÇ flate2 v1.1.9 (*)

io_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\io_runtime)
Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)

llm_cloud v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\llm_cloud)
Ôö£ÔöÇÔöÇ llm_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\llm_core)
Ôöé   ÔööÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
Ôö£ÔöÇÔöÇ reqwest v0.12.28
Ôöé   Ôö£ÔöÇÔöÇ base64 v0.22.1
Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôö£ÔöÇÔöÇ futures-channel v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôöé   ÔööÔöÇÔöÇ futures-sink v0.3.32
Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôö£ÔöÇÔöÇ futures-util v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-io v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-sink v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-task v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ memchr v2.8.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   ÔööÔöÇÔöÇ slab v0.4.12
Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   ÔööÔöÇÔöÇ itoa v1.0.18
Ôöé   Ôö£ÔöÇÔöÇ http-body v1.0.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   ÔööÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ http-body-util v0.1.3
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http-body v1.0.1 (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôö£ÔöÇÔöÇ hyper v1.9.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ atomic-waker v1.1.2
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-channel v0.3.32 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http-body v1.0.1 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ httparse v1.10.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ itoa v1.0.18
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tokio v1.52.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ mio v1.2.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.61.2
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ socket2 v0.6.3
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.61.2 (*)
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.61.2 (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ want v0.3.1
Ôöé   Ôöé       ÔööÔöÇÔöÇ try-lock v0.2.5
Ôöé   Ôö£ÔöÇÔöÇ hyper-rustls v0.27.9
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hyper v1.9.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hyper-util v0.1.20
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ base64 v0.22.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-channel v0.3.32 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-util v0.3.32 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http-body v1.0.1 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hyper v1.9.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ipnet v2.12.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ libc v0.2.185
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ percent-encoding v2.3.2
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ socket2 v0.6.3 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tokio v1.52.1 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tower-service v0.3.3
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ tracing v0.1.44
Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ tracing-core v0.1.36
Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ once_cell v1.21.4
Ôöé   Ôöé   Ôöé               Ôö£ÔöÇÔöÇ critical-section v1.2.0
Ôöé   Ôöé   Ôöé               ÔööÔöÇÔöÇ portable-atomic v1.13.1
Ôöé   Ôöé   Ôöé                   ÔööÔöÇÔöÇ critical-section v1.2.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustls v0.23.38
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ once_cell v1.21.4 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ring v0.17.14
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ getrandom v0.2.17
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ untrusted v0.9.0
Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cc v1.2.60
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ find-msvc-tools v0.1.9
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ shlex v1.3.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustls-pki-types v1.14.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zeroize v1.8.2
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustls-webpki v0.103.13
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ring v0.17.14 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustls-pki-types v1.14.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ untrusted v0.9.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ subtle v2.6.1
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zeroize v1.8.2
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tokio v1.52.1 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tokio-rustls v0.26.4
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustls v0.23.38 (*)
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ tokio v1.52.1 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tower-service v0.3.3
Ôöé   Ôöé   ÔööÔöÇÔöÇ webpki-roots v1.0.7
Ôöé   Ôöé       ÔööÔöÇÔöÇ rustls-pki-types v1.14.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ hyper-util v0.1.20 (*)
Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôö£ÔöÇÔöÇ percent-encoding v2.3.2
Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôö£ÔöÇÔöÇ rustls v0.23.38 (*)
Ôöé   Ôö£ÔöÇÔöÇ rustls-pki-types v1.14.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
Ôöé   Ôö£ÔöÇÔöÇ serde_urlencoded v0.7.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ form_urlencoded v1.2.2
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ percent-encoding v2.3.2
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ itoa v1.0.18
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ryu v1.0.23
Ôöé   Ôöé   ÔööÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôö£ÔöÇÔöÇ sync_wrapper v1.0.2
Ôöé   Ôöé   ÔööÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôö£ÔöÇÔöÇ tokio v1.52.1 (*)
Ôöé   Ôö£ÔöÇÔöÇ tokio-rustls v0.26.4 (*)
Ôöé   Ôö£ÔöÇÔöÇ tower v0.5.3
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-core v0.3.32
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-util v0.3.32 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ sync_wrapper v1.0.2 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tokio v1.52.1 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tower-layer v0.3.3
Ôöé   Ôöé   ÔööÔöÇÔöÇ tower-service v0.3.3
Ôöé   Ôö£ÔöÇÔöÇ tower-http v0.6.8
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ serde_core v1.0.228
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytes v1.11.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ futures-util v0.3.32 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http v1.4.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ http-body v1.0.1 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ iri-string v0.7.12
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project-lite v0.2.17
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tower v0.5.3 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tower-layer v0.3.3
Ôöé   Ôöé   ÔööÔöÇÔöÇ tower-service v0.3.3
Ôöé   Ôö£ÔöÇÔöÇ tower-service v0.3.3
Ôöé   Ôö£ÔöÇÔöÇ url v2.5.8
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ form_urlencoded v1.2.2 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ idna v1.1.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ idna_adapter v1.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_normalizer v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_collections v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ displaydoc v0.2.5 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ potential_utf v0.1.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ writeable v0.6.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ yoke v0.8.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ stable_deref_trait v1.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ yoke-derive v0.8.2 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé   Ôö£ÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé   ÔööÔöÇÔöÇ synstructure v0.13.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôöé       ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ zerofrom v0.1.7
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé       ÔööÔöÇÔöÇ zerofrom-derive v0.1.7 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé           Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé           Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé           Ôö£ÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé           ÔööÔöÇÔöÇ synstructure v0.13.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ zerofrom v0.1.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ zerovec-derive v0.11.3 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ utf8_iter v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ yoke v0.8.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ zerofrom v0.1.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_normalizer_data v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_provider v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ displaydoc v0.2.5 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_locale_core v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ displaydoc v0.2.5 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ litemap v0.8.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tinystr v0.8.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ displaydoc v0.2.5 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ writeable v0.6.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ stable_deref_trait v1.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ writeable v0.6.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ yoke v0.8.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ zerofrom v0.1.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ zerotrie v0.2.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ displaydoc v0.2.5 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ yoke v0.8.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerofrom v0.1.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ icu_properties v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ icu_collections v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ icu_locale_core v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ icu_properties_data v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ icu_provider v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ zerotrie v0.2.4 (*)
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ utf8_iter v1.0.4
Ôöé   Ôöé   ÔööÔöÇÔöÇ percent-encoding v2.3.2
Ôöé   ÔööÔöÇÔöÇ webpki-roots v1.0.7 (*)
Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
ÔööÔöÇÔöÇ serde_json v1.0.149 (*)

llm_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\llm_core) (*)

llm_local v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\llm_local)
ÔööÔöÇÔöÇ llm_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\llm_core) (*)

project_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\project_runtime) (*)

spec_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\spec_runtime) (*)

tool_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\tool_runtime) (*)

ui_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_core)
Ôö£ÔöÇÔöÇ core_domain v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\core_domain) (*)
ÔööÔöÇÔöÇ io_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\io_runtime) (*)

ui_i18n v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_i18n)
ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)

ui_slint v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_slint)
Ôö£ÔöÇÔöÇ app_services v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\app_services) (*)
Ôö£ÔöÇÔöÇ clipboard-win v5.4.1
Ôöé   ÔööÔöÇÔöÇ error-code v3.3.2
Ôö£ÔöÇÔöÇ document_text_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\document_text_runtime) (*)
Ôö£ÔöÇÔöÇ io_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\io_runtime) (*)
Ôö£ÔöÇÔöÇ project_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\project_runtime) (*)
Ôö£ÔöÇÔöÇ slint v1.16.0
Ôöé   Ôö£ÔöÇÔöÇ const-field-offset v0.2.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ const-field-offset-macro v0.2.0 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ field-offset v0.3.6
Ôöé   Ôöé       ÔööÔöÇÔöÇ memoffset v0.9.1
Ôöé   Ôöé           [build-dependencies]
Ôöé   Ôöé           ÔööÔöÇÔöÇ autocfg v1.5.0
Ôöé   Ôöé       [build-dependencies]
Ôöé   Ôöé       ÔööÔöÇÔöÇ rustc_version v0.4.1
Ôöé   Ôöé           ÔööÔöÇÔöÇ semver v1.0.28
Ôöé   Ôö£ÔöÇÔöÇ i-slint-backend-selector v1.16.0
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-backend-winit v1.16.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit v0.22.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit_winit v0.30.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit v0.22.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit_windows v0.30.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit v0.22.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit_consumer v0.32.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ accesskit v0.22.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ hashbrown v0.16.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ allocator-api2 v0.2.21
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ equivalent v1.0.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ foldhash v0.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hashbrown v0.16.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ static_assertions v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows v0.61.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-collections v0.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-core v0.61.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-implement v0.60.2 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-interface v0.59.3 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-result v0.3.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-strings v0.4.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-core v0.61.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-future v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-core v0.61.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-threading v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-numerics v0.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-core v0.61.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-link v0.1.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-core v0.61.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ winit v0.30.13
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ cursor-icon v1.2.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ dpi v0.1.2
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ smol_str v0.2.2
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ tracing v0.1.44 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-sys v0.52.0
Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ windows-targets v0.52.6
Ôöé   Ôöé   Ôöé   Ôöé               ÔööÔöÇÔöÇ windows_x86_64_msvc v0.52.6
Ôöé   Ôöé   Ôöé   Ôöé       [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ cfg_aliases v0.2.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ bytemuck_derive v1.10.2 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ copypasta v0.10.2
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ clipboard-win v5.4.1 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ derive_more-impl v2.1.1 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ convert_case v0.10.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ unicode-xid v0.2.6
Ôöé   Ôöé   Ôöé   Ôöé       [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ rustc_version v0.4.1 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glutin v0.32.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glutin_egl_sys v0.7.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.52.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ gl_generator v0.14.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ khronos_api v3.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ xml-rs v0.8.28
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glutin_wgl_sys v0.6.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ gl_generator v0.14.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ libloading v0.8.9
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ once_cell v1.21.4 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.52.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg_aliases v0.2.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glutin-winit v0.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glutin v0.32.3 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ winit v0.30.13 (*)
Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg_aliases v0.2.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ fontique v0.8.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hashbrown v0.16.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ linebender_resource_handle v0.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ memmap2 v0.9.10
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ parlance v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ read-fonts v0.37.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ core_maths v0.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ libm v0.2.16
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ font-types v0.11.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows v0.62.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-collections v0.3.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-core v0.62.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-implement v0.60.2 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-interface v0.59.3 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-result v0.4.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-strings v0.5.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-core v0.62.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-future v0.3.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-core v0.62.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-threading v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-numerics v0.3.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ windows-core v0.62.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-link v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-core v0.62.2 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ htmlparser v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pulldown-cmark v0.13.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ getopts v0.2.24
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ unicode-width v0.2.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ memchr v2.8.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pulldown-cmark-escape v0.11.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ unicase v2.9.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ skrifa v0.40.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ read-fonts v0.37.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core v1.16.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ auto_enums v0.8.8 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_utils v0.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ chrono v0.4.44 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ clru v0.6.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ hashbrown v0.16.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ const-field-offset v0.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ euclid v0.22.14
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core-macros v1.16.0 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ serde_json v1.0.149 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_normalizer v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ image v0.25.10
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ byteorder-lite v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ moxcms v0.8.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ pxfm v0.1.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ png v0.18.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ crc32fast v1.5.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ fdeflate v0.3.7
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ simd-adler32 v0.3.9
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ flate2 v1.1.9 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ miniz_oxide v0.8.9 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ zune-core v0.5.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zune-jpeg v0.5.15
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ zune-core v0.5.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_algorithms v1.0.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_geom v1.0.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayvec v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ euclid v0.22.14 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_extra v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ thiserror v2.0.18
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ thiserror-impl v2.0.18 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_geom v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ once_cell v1.21.4 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ parley v0.8.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ fontique v0.8.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ harfrust v0.5.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ core_maths v0.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ read-fonts v0.37.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hashbrown v0.16.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_normalizer v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_properties v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_segmenter v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_collections v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_locale v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_collections v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_locale_core v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_locale_data v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_provider v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ potential_utf v0.1.5 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tinystr v0.8.3 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_provider v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_segmenter_data v2.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ potential_utf v0.1.5 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ utf8_iter v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zerovec v0.11.6 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ linebender_resource_handle v0.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ parlance v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ parley_data v0.8.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ icu_properties v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ skrifa v0.40.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-project v1.1.11
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ pin-project-internal v1.1.11 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-weak v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ portable-atomic v1.13.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ resvg v0.47.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ gif v0.14.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ color_quant v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ weezl v0.1.12
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ image-webp v0.2.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ byteorder-lite v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ quick-error v2.0.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pico-args v0.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rgb v0.8.53
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ svgtypes v0.16.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ kurbo v0.13.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayvec v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ siphasher v1.0.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tiny-skia v0.12.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayref v0.3.9
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayvec v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ png v0.18.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ tiny-skia-path v0.12.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ arrayref v0.3.9
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ strict-num v0.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ float-cmp v0.9.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ usvg v0.47.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ base64 v0.22.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ data-url v0.3.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ flate2 v1.1.9 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ fontdb v0.23.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ slotmap v1.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ version_check v0.9.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tinyvec v1.11.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ tinyvec_macros v0.1.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ ttf-parser v0.25.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ core_maths v0.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ imagesize v0.14.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ kurbo v0.13.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pico-args v0.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ roxmltree v0.21.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ memchr v2.8.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustybuzz v0.20.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ core_maths v0.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smallvec v1.15.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ttf-parser v0.25.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-bidi-mirroring v0.4.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-ccc v0.4.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-properties v0.1.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ unicode-script v0.5.8
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ simplecss v0.2.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ siphasher v1.0.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ strict-num v0.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ svgtypes v0.16.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tiny-skia-path v0.12.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ ttf-parser v0.25.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-bidi v0.3.18
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-script v0.5.8
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-vo v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ xmlwriter v0.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zune-jpeg v0.5.15 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rgb v0.8.53 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ scoped-tls-hkt v0.1.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ scopeguard v1.2.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ skrifa v0.40.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ slab v0.4.12
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ strum v0.28.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ strum_macros v0.28.0 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ heck v0.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ swash v0.2.7
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ skrifa v0.40.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ yazi v0.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zeno v0.3.3
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ sys-locale v0.3.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ taffy v0.9.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayvec v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ slotmap v1.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-linebreak v0.1.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-script v0.5.8
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ vtable v0.4.0
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ const-field-offset v0.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ portable-atomic v1.13.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ stable_deref_trait v1.2.1
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ vtable-macro v0.4.0 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé           Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core-macros v1.16.0 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-renderer-femtovg v1.16.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ cfg-if v1.0.4
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ const-field-offset v0.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ femtovg v0.23.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ fnv v1.0.7
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glow v0.17.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ imgref v1.12.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ itertools v0.14.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ either v1.15.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rgb v0.8.53 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ slotmap v1.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ swash v0.2.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ glow v0.17.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core-macros v1.16.0 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ imgref v1.12.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-weak v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ rgb v0.8.53 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-renderer-software v1.16.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bytemuck v1.25.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ clru v0.6.3 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ euclid v0.22.14 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ integer-sqrt v0.1.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ skrifa v0.40.0 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ swash v0.2.7 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ zeno v0.3.3
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ imgref v1.12.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ muda v0.18.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ crossbeam-channel v0.5.15
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ crossbeam-utils v0.8.21
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ dpi v0.1.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ keyboard-types v0.7.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ bitflags v2.11.1 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ once_cell v1.21.4 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ thiserror v2.0.18 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.60.2
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ windows-targets v0.53.5
Ôöé   Ôöé   Ôöé   Ôöé           ÔööÔöÇÔöÇ windows_x86_64_msvc v0.53.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ pin-weak v1.1.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rgb v0.8.53 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ scoped-tls-hkt v0.1.5
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ scopeguard v1.2.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ softbuffer v0.4.8
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ raw-window-handle v0.6.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ tracing v0.1.44 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ windows-sys v0.61.2 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ strum v0.28.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ vtable v0.4.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ webbrowser v1.2.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ log v0.4.29
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ url v2.5.8 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ windows v0.62.2 (*)
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ winit v0.30.13 (*)
Ôöé   Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ cfg_aliases v0.2.1
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core v1.16.0 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-core-macros v1.16.0 (proc-macro) (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ i-slint-renderer-femtovg v1.16.0 (*)
Ôöé   Ôöé   [build-dependencies]
Ôöé   Ôöé   ÔööÔöÇÔöÇ i-slint-common v1.16.0
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé       Ôö£ÔöÇÔöÇ htmlparser v0.2.1
Ôöé   Ôöé       ÔööÔöÇÔöÇ pulldown-cmark v0.13.3 (*)
Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ i-slint-core v1.16.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ i-slint-core-macros v1.16.0 (proc-macro) (*)
Ôöé   Ôö£ÔöÇÔöÇ i-slint-renderer-femtovg v1.16.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ i-slint-renderer-software v1.16.0 (*)
Ôöé   Ôö£ÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôö£ÔöÇÔöÇ once_cell v1.21.4 (*)
Ôöé   Ôö£ÔöÇÔöÇ pin-weak v1.1.0
Ôöé   Ôö£ÔöÇÔöÇ slint-macros v1.16.0 (proc-macro)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-compiler v1.16.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ annotate-snippets v0.12.15
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ anstyle v1.0.14
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ unicode-width v0.2.2
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ by_address v1.2.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ data-url v0.3.2
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ derive_more v2.1.1 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ i-slint-common v1.16.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ icu_normalizer v2.2.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ itertools v0.14.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ linked_hash_set v0.1.6
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ linked-hash-map v0.5.6
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_extra v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_geom v1.0.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ arrayvec v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ euclid v0.22.14 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ libm v0.2.16
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       [build-dependencies]
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ autocfg v1.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ num-traits v0.2.19 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ thiserror v2.0.18
Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ thiserror-impl v2.0.18 (proc-macro) (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ lyon_path v1.0.19 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num_enum v0.7.6
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ num_enum_derive v0.7.6 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro-crate v3.5.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ toml_edit v0.25.11+spec-1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ indexmap v2.14.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   Ôö£ÔöÇÔöÇ equivalent v1.0.2
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ hashbrown v0.17.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ toml_datetime v1.1.1+spec-1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôö£ÔöÇÔöÇ toml_parser v1.1.2+spec-1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       Ôöé   ÔööÔöÇÔöÇ winnow v1.0.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôöé       ÔööÔöÇÔöÇ winnow v1.0.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ syn v2.0.117 (*)
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ rustversion v1.0.22 (proc-macro)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rowan v0.16.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ countme v3.0.1
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ hashbrown v0.14.5
Ôöé   Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ rustc-hash v1.1.0
Ôöé   Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ text-size v1.1.1
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ smol_str v0.3.6
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ strum v0.28.0 (*)
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ typed-index-collections v3.5.0
Ôöé   Ôöé   Ôöé   Ôö£ÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   Ôöé   Ôöé   ÔööÔöÇÔöÇ url v2.5.8 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ proc-macro2 v1.0.106 (*)
Ôöé   Ôöé   Ôö£ÔöÇÔöÇ quote v1.0.45 (*)
Ôöé   Ôöé   ÔööÔöÇÔöÇ spin_on v0.1.1
Ôöé   Ôöé       ÔööÔöÇÔöÇ pin-utils v0.1.0
Ôöé   Ôö£ÔöÇÔöÇ unicode-segmentation v1.13.2
Ôöé   ÔööÔöÇÔöÇ vtable v0.4.0 (*)
Ôö£ÔöÇÔöÇ spec_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\spec_runtime) (*)
Ôö£ÔöÇÔöÇ tool_runtime v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\tool_runtime) (*)
Ôö£ÔöÇÔöÇ ui_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_core) (*)
Ôö£ÔöÇÔöÇ ui_i18n v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_i18n) (*)
Ôö£ÔöÇÔöÇ ui_theme v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_theme)
Ôöé   ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)
ÔööÔöÇÔöÇ workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)

ui_theme v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\ui_theme) (*)

verify_progress v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\verify_progress)
Ôö£ÔöÇÔöÇ serde v1.0.228 (*)
ÔööÔöÇÔöÇ serde_json v1.0.149 (*)

workspace_core v0.1.0 (C:\MAQUETAS SOFTWARE\codex_sandbox_app_docgraph_B\rust_portable_app\crates\workspace_core) (*)
```

## cargo metadata

- Status: `OK`
- Command: `cargo metadata --format-version 1 --no-deps`
- Exit code: `0`

### stdout

```text
{"packages":[{"name":"action_core","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/action_core#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"action_core","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\action_core\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\action_core\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"core_domain","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/core_domain#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"sha2","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.10","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"core_domain","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"workspace_core","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/workspace_core#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"workspace_core","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"io_runtime","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/io_runtime#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"io_runtime","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\io_runtime\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\io_runtime\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"spec_runtime","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/spec_runtime#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"spec_runtime","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\spec_runtime\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\spec_runtime\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"project_runtime","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/project_runtime#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"spec_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\spec_runtime"},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"project_runtime","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\project_runtime\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\project_runtime\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"document_text_runtime","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/document_text_runtime#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"lopdf","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.32","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"zip","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.6","kind":null,"rename":null,"optional":false,"uses_default_features":false,"features":["deflate"],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"document_text_runtime","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\document_text_runtime\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\document_text_runtime\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"llm_core","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_core#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"llm_core","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_core\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_core\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"llm_local","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_local#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"llm_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"llm_local","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_local\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_local\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"llm_cloud","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_cloud#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"llm_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_core"},{"name":"reqwest","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^0.12","kind":null,"rename":null,"optional":false,"uses_default_features":false,"features":["blocking","json","rustls-tls"],"target":null,"registry":null},{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"llm_cloud","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_cloud\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\llm_cloud\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"tool_runtime","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/tool_runtime#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"tool_runtime","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\tool_runtime\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\tool_runtime\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"ui_core","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_core#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"io_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\io_runtime"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"ui_core","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_core\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_core\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"ui_i18n","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_i18n#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"ui_i18n","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_i18n\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_i18n\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"ui_theme","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_theme#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"ui_theme","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_theme\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_theme\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"ui_slint","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_slint#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"app_services","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\app_services"},{"name":"clipboard-win","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^5.4.1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null},{"name":"document_text_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\document_text_runtime"},{"name":"io_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\io_runtime"},{"name":"project_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\project_runtime"},{"name":"slint","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1.8","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["std"],"target":null,"registry":null},{"name":"spec_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\spec_runtime"},{"name":"tool_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\tool_runtime"},{"name":"ui_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_core"},{"name":"ui_i18n","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_i18n"},{"name":"ui_theme","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_theme"},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"ui_slint","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_slint\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\ui_slint\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"app_services","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/app_services#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"project_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\project_runtime"},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"app_services","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\app_services\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\app_services\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"cli_app","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/cli_app#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"app_services","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\app_services"},{"name":"core_domain","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\core_domain"},{"name":"tool_runtime","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\tool_runtime"},{"name":"workspace_core","source":null,"req":"*","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null,"path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\workspace_core"}],"targets":[{"kind":["bin"],"crate_types":["bin"],"name":"cli_app","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\cli_app\\src\\main.rs","edition":"2021","doc":true,"doctest":false,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\cli_app\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null},{"name":"verify_progress","version":"0.1.0","id":"path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/verify_progress#0.1.0","license":null,"license_file":null,"description":null,"source":null,"dependencies":[{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":["derive"],"target":null,"registry":null},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","req":"^1","kind":null,"rename":null,"optional":false,"uses_default_features":true,"features":[],"target":null,"registry":null}],"targets":[{"kind":["lib"],"crate_types":["lib"],"name":"verify_progress","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\verify_progress\\src\\lib.rs","edition":"2021","doc":true,"doctest":true,"test":true},{"kind":["bin"],"crate_types":["bin"],"name":"verify_progress","src_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\verify_progress\\src\\main.rs","edition":"2021","doc":true,"doctest":false,"test":true}],"features":{},"manifest_path":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\crates\\verify_progress\\Cargo.toml","metadata":null,"publish":null,"authors":[],"categories":[],"keywords":[],"readme":null,"repository":null,"homepage":null,"documentation":null,"edition":"2021","links":null,"default_run":null,"rust_version":null}],"workspace_members":["path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/action_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/core_domain#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/workspace_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/io_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/spec_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/project_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/document_text_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_local#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_cloud#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/tool_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_i18n#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_theme#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_slint#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/app_services#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/cli_app#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/verify_progress#0.1.0"],"workspace_default_members":["path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/action_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/core_domain#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/workspace_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/io_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/spec_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/project_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/document_text_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_local#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/llm_cloud#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/tool_runtime#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_core#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_i18n#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_theme#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/ui_slint#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/app_services#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/cli_app#0.1.0","path+file:///C:/MAQUETAS%20SOFTWARE/codex_sandbox_app_docgraph_B/rust_portable_app/crates/verify_progress#0.1.0"],"resolve":null,"target_directory":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\target","build_directory":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app\\target","version":1,"workspace_root":"C:\\MAQUETAS SOFTWARE\\codex_sandbox_app_docgraph_B\\rust_portable_app","metadata":null}
```

## cargo clippy

- Status: `SKIPPED_HEAVY_BY_DEFAULT`
- Command: `cargo clippy --workspace --all-targets -- --no-deps`

Skipped by default. Re-run with `--deep` to execute this tool.

## cargo udeps

- Status: `SKIPPED_HEAVY_BY_DEFAULT`
- Command: `cargo-udeps --workspace`

Skipped by default. Re-run with `--deep` to execute this tool.

## cargo bloat

- Status: `SKIPPED_HEAVY_BY_DEFAULT`
- Command: `cargo-bloat --workspace --crates`

Skipped by default. Re-run with `--deep` to execute this tool.

## cargo deny

- Status: `NOT_AVAILABLE`
- Command: `cargo-deny check`

No output captured.

## Summary

- `cargo_tree`: `OK`
- `cargo_metadata`: `OK`
- `cargo_clippy`: `SKIPPED_HEAVY_BY_DEFAULT`
- `cargo_udeps`: `SKIPPED_HEAVY_BY_DEFAULT`
- `cargo_bloat`: `SKIPPED_HEAVY_BY_DEFAULT`
- `cargo_deny`: `NOT_AVAILABLE`
