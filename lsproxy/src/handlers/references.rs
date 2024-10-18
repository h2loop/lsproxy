use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use log::{info, error};
use lsp_types::Position;

use crate::api_types::{GetReferencesRequest, ReferencesResponse};
use crate::AppState;
use crate::api_types::ErrorResponse;
use crate::lsp::manager::LspManagerError;

/// Find all references to a symbol
///
/// The input position should point to the identifier of the symbol you want to get the references for.
///
/// Returns a list of locations where the symbol at the given position is referenced.
///
/// The returned positions point to the start of the reference identifier.
///
/// e.g. for `User` on line 0 of `src/main.py`:
/// ```
///  0: class User:
///  input____^^^^
///  1:     def __init__(self, name, age):
///  2:         self.name = name
///  3:         self.age = age
///  4:
///  5: user = User("John", 30)
///  output____^
/// ```
#[utoipa::path(
    post,
    path = "/references",
    request_body = GetReferencesRequest,
    responses(
        (status = 200, description = "References retrieved successfully", body = ReferencesResponse),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn references(data: Data<AppState>, info: Json<GetReferencesRequest>) -> HttpResponse {
    info!(
        "Received references request for file: {}, line: {}, character: {}",
        info.symbol_identifier_position.path,
        info.symbol_identifier_position.position.line,
        info.symbol_identifier_position.position.character
    );
    let lsp_manager = data.lsp_manager.lock().unwrap();
    let result = lsp_manager
        .references(
            &info.symbol_identifier_position.path,
            Position {
                line: info.symbol_identifier_position.position.line,
                character: info.symbol_identifier_position.position.character,
            },
            info.include_declaration,
        )
        .await;
    match result {
        Ok(references) => HttpResponse::Ok().json(ReferencesResponse::from((
            references,
            info.include_raw_response,
        ))),
        Err(e) => {
            error!("Failed to get references: {}", e);
            match e {
                LspManagerError::FileNotFound(path) => {
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: format!("File not found: {}", path),
                    })
                }
                LspManagerError::LspClientNotFound(lang) => HttpResponse::InternalServerError()
                    .body(format!("LSP client not found for {:?}", lang)),
                LspManagerError::InternalError(msg) => {
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: format!("Internal error: {}", msg),
                    })
                }
                LspManagerError::UnsupportedFileType(path) => {
                    HttpResponse::BadRequest().json(ErrorResponse {
                        error: format!("Unsupported file type: {}", path),
                    })
                }
            }
        }
    }
}
