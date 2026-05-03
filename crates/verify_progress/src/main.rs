use std::env;
use std::path::PathBuf;

use verify_progress::{
    ai_specs_report_to_pretty_json, report_to_markdown, report_to_pretty_json, validate_ai_specs,
    verify_progress, write_ai_specs_report, write_text_output, AiSpecsValidationStatus,
    OverallStatus, VerificationMode, WrapperScriptRunner,
};

fn main() {
    let options = match parse_args(env::args().skip(1).collect()) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    match options.command {
        CliCommand::VerifyProgress {
            mode,
            json_out,
            md_out,
        } => run_progress_verification(options.workspace_root, mode, json_out, md_out),
        CliCommand::ValidateAiSpecs => run_ai_specs_validation(options.workspace_root),
    }
}

fn run_progress_verification(
    workspace_root: PathBuf,
    mode: VerificationMode,
    json_out: Option<PathBuf>,
    md_out: Option<PathBuf>,
) {
    let runner = WrapperScriptRunner;
    let report = match verify_progress(&workspace_root, mode, &runner) {
        Ok(report) => report,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    let json = match report_to_pretty_json(&report) {
        Ok(json) => json,
        Err(message) => {
            eprintln!("failed to serialize report: {message}");
            std::process::exit(1);
        }
    };

    println!("{json}");

    if let Some(path) = &json_out {
        if let Err(message) = write_text_output(path, &json) {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }

    if let Some(path) = &md_out {
        if let Err(message) = write_text_output(path, &report_to_markdown(&report)) {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }

    if report.overall_status == OverallStatus::Fail {
        std::process::exit(1);
    }
}

fn run_ai_specs_validation(workspace_root: PathBuf) {
    let report = match validate_ai_specs(&workspace_root) {
        Ok(report) => report,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    };

    let json = match ai_specs_report_to_pretty_json(&report) {
        Ok(json) => json,
        Err(message) => {
            eprintln!("failed to serialize report: {message}");
            std::process::exit(1);
        }
    };

    println!("json_checked_count={}", report.json_checked_count);
    println!(
        "prompt_specs_checked_count={}",
        report.prompt_specs_checked_count
    );
    println!(
        "semantic_specs_checked_count={}",
        report.semantic_specs_checked_count
    );
    println!("policies_checked_count={}", report.policies_checked_count);
    println!("schemas_checked_count={}", report.schemas_checked_count);
    println!("error_count={}", report.error_count);
    println!("warning_count={}", report.warning_count);
    println!(
        "report={}",
        workspace_root
            .join("user/output/validate_ai_specs_report.json")
            .display()
    );

    if let Err(message) = write_ai_specs_report(&workspace_root, &report) {
        eprintln!("{message}");
        std::process::exit(1);
    }

    if report.status == AiSpecsValidationStatus::Error {
        for error in &report.errors {
            println!("[ERROR] {}: {} - {}", error.file, error.code, error.message);
        }
        std::process::exit(1);
    }

    println!("{json}");
    println!("[OK] validate_ai_specs=true");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliOptions {
    workspace_root: PathBuf,
    command: CliCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CliCommand {
    VerifyProgress {
        mode: VerificationMode,
        json_out: Option<PathBuf>,
        md_out: Option<PathBuf>,
    },
    ValidateAiSpecs,
}

fn parse_args(args: Vec<String>) -> Result<CliOptions, String> {
    let mut workspace_root = None;
    let mut mode = None;
    let mut json_out = None;
    let mut md_out = None;
    let mut validate_ai_specs_mode = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--validate-ai-specs" => {
                validate_ai_specs_mode = true;
            }
            "--workspace-root" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--workspace-root requires a value".to_owned())?;
                workspace_root = Some(PathBuf::from(value));
            }
            "--mode" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--mode requires a value".to_owned())?;
                mode = VerificationMode::parse(value);
                if mode.is_none() {
                    return Err(format!("unsupported mode: {value}"));
                }
            }
            "--json-out" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--json-out requires a value".to_owned())?;
                json_out = Some(PathBuf::from(value));
            }
            "--md-out" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| "--md-out requires a value".to_owned())?;
                md_out = Some(PathBuf::from(value));
            }
            flag => return Err(format!("unsupported argument: {flag}")),
        }

        index += 1;
    }

    let workspace_root =
        workspace_root.ok_or_else(|| "missing required argument: --workspace-root".to_owned())?;
    if validate_ai_specs_mode {
        return Ok(CliOptions {
            workspace_root,
            command: CliCommand::ValidateAiSpecs,
        });
    }

    Ok(CliOptions {
        workspace_root,
        command: CliCommand::VerifyProgress {
            mode: mode.ok_or_else(|| "missing required argument: --mode".to_owned())?,
            json_out,
            md_out,
        },
    })
}
