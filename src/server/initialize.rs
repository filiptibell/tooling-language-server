use itertools::Itertools;
use tracing::{info, trace};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::server::*;

impl Server {
    pub async fn respond_to_initalize(&self, params: InitializeParams) -> Result<InitializeResult> {
        trace!("Initializing server with params: {params:#?}");

        // Negotiate with client for settings, encodings, try to prefer using utf-8
        log_client_info(&params);
        let (position_encoding, offset_encoding) = negotiate_position_and_offset_encoding(&params);

        // Create registration parameters combined for all known tools
        let diagnostic_registration_options = DiagnosticRegistrationOptions {
            text_document_registration_options: TextDocumentRegistrationOptions {
                document_selector: Some(
                    Tools::file_globs()
                        .iter()
                        .map(|&glob| DocumentFilter {
                            scheme: Some(String::from("file")),
                            pattern: Some(String::from(glob)),
                            language: None,
                        })
                        .collect(),
                ),
            },
            diagnostic_options: DiagnosticOptions {
                inter_file_dependencies: false,
                workspace_diagnostics: false,
                ..Default::default()
            },
            ..Default::default()
        };

        // Respond with negotiated encoding, server info, capabilities
        Ok(InitializeResult {
            offset_encoding: Some(offset_encoding),
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(position_encoding),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        change: Some(TextDocumentSyncKind::FULL),
                        open_close: Some(true),
                        ..Default::default()
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::RegistrationOptions(
                    diagnostic_registration_options,
                )),
                ..ServerCapabilities::default()
            },
        })
    }
}

fn log_client_info(params: &InitializeParams) {
    let num_folders = params
        .workspace_folders
        .as_deref()
        .map(|f| f.len())
        .unwrap_or(0);

    if let Some(info) = &params.client_info {
        if let Some(version) = &info.version {
            info!(
                "Client connected - {} v{} - {} workspace folder{}",
                info.name,
                version,
                num_folders,
                if num_folders != 1 { "s" } else { "" }
            );
        } else {
            info!(
                "Client connected - {} - {} workspace folder{}",
                info.name,
                num_folders,
                if num_folders != 1 { "s" } else { "" }
            );
        }
    }
}

fn negotiate_position_and_offset_encoding(
    params: &InitializeParams,
) -> (PositionEncodingKind, String) {
    let encoding_capabilities = match &params.capabilities.general {
        Some(general) => general.position_encodings.as_ref(),
        None => None,
    };

    let position_encodings_str;
    let position_encoding = match encoding_capabilities {
        Some(encodings) => {
            position_encodings_str = encodings
                .iter()
                .map(|e| format!("\"{}\"", e.as_str()))
                .join(", ");
            if encodings.contains(&PositionEncodingKind::UTF8) {
                PositionEncodingKind::UTF8
            } else {
                PositionEncodingKind::UTF16
            }
        }
        None => {
            position_encodings_str = String::from("N/A");
            PositionEncodingKind::UTF16
        }
    };

    let offset_encodings_str;
    let offset_encoding = match &params.capabilities.offset_encoding {
        Some(encodings) => {
            offset_encodings_str = encodings
                .iter()
                .map(|e| format!("\"{}\"", e.as_str()))
                .join(", ");
            if encodings.contains(&String::from("utf-8"))
                || encodings.contains(&String::from("utf8"))
            {
                "utf-8"
            } else {
                "utf-16"
            }
        }
        None => {
            offset_encodings_str = String::from("N/A");
            "utf-16"
        }
    };

    info!(
        "Client encoding support\n\tPosition: {}\n\tOffset: {}",
        position_encodings_str, offset_encodings_str
    );
    info!(
        "Negotiated encodings with client\n\tPosition: \"{}\"\n\tOffset: \"{}\"",
        position_encoding.as_str(),
        offset_encoding,
    );

    (position_encoding, offset_encoding.to_string())
}