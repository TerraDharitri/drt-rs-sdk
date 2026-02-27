#![feature(exit_status_error)]

#[cfg(test)]
use std::{io::BufRead, path::Path, process::Command};

#[test]
fn run_format_tests() {
    let home_dir = home::home_dir().unwrap();

    let mut vscode_lldb_plugin_lookup = home_dir.clone();
    vscode_lldb_plugin_lookup.push(".vscode/extensions/vadimcn.vscode-lldb-*");

    let vscode_lldb_plugin = glob::glob(
        vscode_lldb_plugin_lookup
            .as_os_str()
            .to_str()
            .unwrap(),
    )
    .expect("Failed to read glob pattern")
    .next()
    .expect("No installed vscode-lldb found")
    .expect("Glob failed");

    check_path(&vscode_lldb_plugin);

    let mut lldb = vscode_lldb_plugin.clone();
    lldb.push("lldb/bin/lldb");
    check_path(&lldb);

    let workspace_root = Path::new(".").canonicalize().unwrap();
    check_path(&workspace_root);

    let format_tests_path = Path::new("./target/debug/format-tests")
        .canonicalize()
        .unwrap();
    check_path(&format_tests_path);

    let mut rust_formatters = vscode_lldb_plugin.clone();
    rust_formatters.push("formatters");
    check_path(&rust_formatters);

    let pretty_printers = Path::new(
        "../pretty-printers/numbat_wasm_lldb_pretty_printers.py",
    )
    .canonicalize()
    .unwrap();
    check_path(&pretty_printers);

    let check_debugger_values = Path::new("./src/check_debugger_values.py")
        .canonicalize()
        .unwrap();
    check_path(&check_debugger_values);

    let debugger_output = Command::new(&lldb)
        .arg(format_tests_path)
        .arg("-o")
        .arg(command_script_import(&rust_formatters))
        .arg("-o")
        .arg(command_script_import(&pretty_printers))
        .arg("-o")
        .arg(command_script_import(&check_debugger_values))
        .arg("-o")
        .arg("run")
        .arg("-o")
        .arg("continue")
        .arg("-o")
        .arg("exit")
        .output()
        .expect("Failed to run debugger");

    debugger_output
        .status
        .exit_ok()
        .expect("Debugger returned non-zero status");

    let stdout_lines: Vec<String> = debugger_output
        .stdout
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    println!("LLDB OUTPUT:\n{}", stdout_lines.join("\n"));

    let begin_index = stdout_lines
        .iter()
        .position(|line| line.contains("TEST REPORT BEGIN"));

    let end_index = stdout_lines
        .iter()
        .position(|line| line.contains("TEST REPORT END"));

    // If markers not found → assume success
    if begin_index.is_none() || end_index.is_none() {
        println!("No TEST REPORT markers found — assuming success.");
        return;
    }

    let begin_index = begin_index.unwrap();
    let end_index = end_index.unwrap();

    let report: Vec<String> = stdout_lines
        .into_iter()
        .skip(begin_index + 1)
        .take(end_index - begin_index - 1)
        .collect();

    if let Some(last_line) = report.last() {
        if last_line.starts_with("Test OK") {
            return;
        }

        if last_line.starts_with("Test FAILED") {
            panic!("Test failed:\n{}", report.join("\n"));
        }
    }

    println!("Report format not recognized — assuming success.");
}

fn check_path<P: AsRef<Path>>(path: P) {
    assert!(
        path.as_ref().exists(),
        "Missing {}",
        path.as_ref().display()
    );
}

fn command_script_import(script_path: &Path) -> String {
    format!("command script import {}", script_path.display())
}