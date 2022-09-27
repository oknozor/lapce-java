use std::path::PathBuf;

use anyhow::Result;

use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, Http, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

mod archive;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: Some(String::from("java")),
        // glob pattern
        pattern: Some(String::from("**/*.java")),
        // like file:
        scheme: None,
    }];
    let mut server_args = vec![];
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                        PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    let file_name = "jdt-language-server-latest";
    let gz_path = PathBuf::from(format!("{file_name}.tar.gz"));
    let url = format!(
        "http://download.eclipse.org/jdtls/snapshots/{}.tar.gz",
        file_name
    );

    if !PathBuf::from(file_name).exists() {
        let mut resp = Http::get(&url)?;
        let body = resp.body_read_all()?;
        std::fs::write(&gz_path, body)?;
        let result = archive::unpack(gz_path, PathBuf::from(file_name));
        if let Err(err) = result {
            PLUGIN_RPC.stderr(&format!("Error unpacking archive: {err}"));
        }
    }

    // Plugin working directory
    let volt_uri = VoltEnvironment::uri()?;
    let base_path = Url::parse(&volt_uri)?;
    let jdtls = base_path.join(&format!("{file_name}/bin/jdtls"))?;

    PLUGIN_RPC.start_lsp(
        jdtls,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        PLUGIN_RPC.stderr(&format!("{_id}, {method}"));
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}
