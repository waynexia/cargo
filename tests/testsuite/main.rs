// See src/cargo/lib.rs for notes on these lint settings.
#![warn(rust_2018_idioms)]
#![allow(clippy::all)]

#[macro_use]
extern crate cargo_test_macro;

mod advanced_env;
mod alt_registry;
mod artifact_dep;
mod bad_config;
mod bad_manifest_path;
mod bench;
mod binary_name;
mod build;
mod build_plan;
mod build_script;
mod build_script_env;
mod build_script_extra_link_arg;
mod cache_messages;
mod cargo;
mod cargo_add;
mod cargo_alias_config;
mod cargo_bench;
mod cargo_build;
mod cargo_check;
mod cargo_clean;
mod cargo_command;
mod cargo_config;
mod cargo_doc;
mod cargo_env_config;
mod cargo_features;
mod cargo_fetch;
mod cargo_fix;
mod cargo_generate_lockfile;
mod cargo_git_checkout;
mod cargo_help;
mod cargo_init;
mod cargo_install;
mod cargo_locate_project;
mod cargo_login;
mod cargo_logout;
mod cargo_metadata;
mod cargo_new;
mod cargo_owner;
mod cargo_package;
mod cargo_pkgid;
mod cargo_publish;
mod cargo_read_manifest;
mod cargo_remove;
mod cargo_report;
mod cargo_run;
mod cargo_rustc;
mod cargo_rustdoc;
mod cargo_search;
mod cargo_targets;
mod cargo_test;
mod cargo_tree;
mod cargo_uninstall;
mod cargo_update;
mod cargo_vendor;
mod cargo_verify_project;
mod cargo_version;
mod cargo_yank;
mod cfg;
mod check;
mod check_cfg;
mod clean;
mod collisions;
mod concurrent;
mod config;
mod config_cli;
mod config_include;
mod corrupt_git;
mod credential_process;
mod cross_compile;
mod cross_publish;
mod custom_target;
mod death;
mod dep_info;
mod direct_minimal_versions;
mod directory;
mod doc;
mod docscrape;
mod edition;
mod error;
mod features;
mod features2;
mod features_namespaced;
mod fetch;
mod fix;
mod freshness;
mod future_incompat_report;
mod generate_lockfile;
mod git;
mod git_auth;
mod git_gc;
mod git_shallow;
mod glob_targets;
mod help;
mod https;
mod inheritable_workspace_fields;
mod install;
mod install_upgrade;
mod jobserver;
mod lints;
mod list_availables;
mod local_registry;
mod locate_project;
mod lockfile_compat;
mod login;
mod logout;
mod lto;
mod member_discovery;
mod member_errors;
mod message_format;
mod messages;
mod metabuild;
mod metadata;
mod minimal_versions;
mod multitarget;
mod net_config;
mod new;
mod offline;
mod old_cargos;
mod out_dir;
mod owner;
mod package;
mod package_features;
mod patch;
mod path;
mod paths;
mod pkgid;
mod proc_macro;
mod profile_config;
mod profile_custom;
mod profile_overrides;
mod profile_targets;
mod profiles;
mod progress;
mod pub_priv;
mod publish;
mod publish_lockfile;
mod read_manifest;
mod registry;
mod registry_auth;
mod rename_deps;
mod replace;
mod required_features;
mod run;
mod rust_version;
mod rustc;
mod rustc_info_cache;
mod rustdoc;
mod rustdoc_extern_html;
mod rustdocflags;
mod rustflags;
mod rustup;
mod script;
mod search;
mod shell_quoting;
mod source_replacement;
mod ssh;
mod standard_lib;
mod test;
mod timings;
mod tool_paths;
mod tree;
mod tree_graph_features;
mod unit_graph;
mod update;
mod vendor;
mod verify_project;
mod version;
mod warn_on_failure;
mod weak_dep_features;
mod workspaces;
mod yank;

#[cargo_test]
fn aaa_trigger_cross_compile_disabled_check() {
    // This triggers the cross compile disabled check to run ASAP, see #5141
    cargo_test_support::cross_compile::disabled();
}
