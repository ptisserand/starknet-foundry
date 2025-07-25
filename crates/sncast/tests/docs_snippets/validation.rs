use std::fs;

use crate::helpers::constants::URL;
use crate::helpers::runner::runner;
use camino::Utf8PathBuf;
use docs::snippet::{Snippet, SnippetType};
use docs::utils::{
    get_nth_ancestor, print_ignored_snippet_message, print_snippets_validation_summary,
    update_scarb_toml_dependencies,
};
use docs::validation::{extract_snippets_from_directory, extract_snippets_from_file};
use shared::test_utils::output_assert::assert_stdout_contains;
use tempfile::TempDir;

#[test]
fn test_docs_snippets() {
    let root_dir_path = get_nth_ancestor(2);
    let docs_dir_path = root_dir_path.join("docs/src");
    let sncast_readme_path = root_dir_path.join("crates/sncast/README.md");

    let snippet_type = SnippetType::sncast();

    let docs_snippets = extract_snippets_from_directory(&docs_dir_path, &snippet_type)
        .expect("Failed to extract command snippets");

    let readme_snippets = extract_snippets_from_file(&sncast_readme_path, &snippet_type)
        .expect("Failed to extract command snippets");

    let snippets = docs_snippets
        .into_iter()
        .chain(readme_snippets)
        .collect::<Vec<Snippet>>();

    let hello_sncast_dir =
        Utf8PathBuf::from_path_buf(root_dir_path.join("docs/listings/hello_sncast"))
            .expect("Invalid UTF-8 path");

    let dirs_to_copy = [
        "crates/sncast/tests/data/files",
        "docs/listings/hello_sncast",
    ];

    let tempdir = TempDir::new().expect("Unable to create a temporary directory");

    let target_path = tempdir.path();

    for dir in &dirs_to_copy {
        let source_path = root_dir_path.join(dir);

        fs_extra::dir::copy(
            source_path.as_path(),
            target_path,
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        )
        .expect("Failed to copy the directory");
    }

    let source_accouns_json_path = hello_sncast_dir.join("accounts.json");
    let target_accounts_json_path = tempdir.path().join("accounts.json");

    fs::copy(&source_accouns_json_path, &target_accounts_json_path)
        .expect("Failed to copy accounts.json");
    update_scarb_toml_dependencies(&tempdir).unwrap();

    for snippet in &snippets {
        if snippet.config.ignored {
            print_ignored_snippet_message(snippet);
            continue;
        }

        let args = snippet.to_command_args();
        let mut args: Vec<&str> = args.iter().map(String::as_str).collect();

        // remove "sncast" from the args
        args.remove(0);

        args.insert(0, "--accounts-file");
        args.insert(1, target_accounts_json_path.to_str().unwrap());

        if snippet.config.replace_network {
            let network_pos = args.iter().position(|arg| *arg == "--network");
            if let Some(network_pos) = network_pos {
                args[network_pos] = "--url";
                args[network_pos + 1] = URL;
            }
        }

        let snapbox = runner(&args).current_dir(tempdir.path());
        let output = snapbox.assert().success();

        if snippet.output.is_some() && !snippet.config.ignored_output {
            assert_stdout_contains(output, snippet.output.as_ref().unwrap());
        }
    }

    print_snippets_validation_summary(&snippets, snippet_type.as_str());
}
