use crate::checker;
use crate::registry;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckPackageParams {
    #[schemars(description = "Package name to check")]
    pub name: String,
    #[schemars(
        description = "Optional comma-separated registry IDs to check (defaults to popular registries)"
    )]
    pub registries: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckPackagesParams {
    #[schemars(description = "List of package names to check")]
    pub names: Vec<String>,
    #[schemars(
        description = "Optional comma-separated registry IDs to check (defaults to popular registries)"
    )]
    pub registries: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListRegistriesParams {}

pub struct StakedMcp {
    tool_router: ToolRouter<Self>,
}

impl Default for StakedMcp {
    fn default() -> Self {
        Self::new()
    }
}

fn resolve_registries(registries: &Option<String>) -> Vec<&'static registry::Registry> {
    match registries {
        Some(ids) => {
            let ids: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
            registry::registries_by_ids(&ids)
        }
        None => registry::popular_registries(),
    }
}

#[tool_router]
impl StakedMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool(
        description = "Check if a package name is available across package registries. Returns availability status for each registry checked."
    )]
    async fn check_package(
        &self,
        Parameters(params): Parameters<CheckPackageParams>,
    ) -> Result<CallToolResult, McpError> {
        let registries = resolve_registries(&params.registries);
        if registries.is_empty() {
            return Err(McpError::invalid_params(
                "No valid registries specified",
                None,
            ));
        }
        let result = checker::check_package(&params.name, &registries).await;
        let json = serde_json::to_string_pretty(&result)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[rmcp::tool(
        description = "Check multiple package names for availability across registries. Runs all checks concurrently."
    )]
    async fn check_packages(
        &self,
        Parameters(params): Parameters<CheckPackagesParams>,
    ) -> Result<CallToolResult, McpError> {
        if params.names.is_empty() {
            return Err(McpError::invalid_params("names list cannot be empty", None));
        }
        if params.names.len() > 50 {
            return Err(McpError::invalid_params(
                "Maximum 50 names per request",
                None,
            ));
        }
        let registries = resolve_registries(&params.registries);
        if registries.is_empty() {
            return Err(McpError::invalid_params(
                "No valid registries specified",
                None,
            ));
        }
        let results = checker::check_packages(&params.names, &registries).await;
        let json = serde_json::to_string_pretty(&results)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[rmcp::tool(
        description = "List all available package registries with their IDs, names, ecosystems, and supported languages."
    )]
    async fn list_registries(
        &self,
        Parameters(_params): Parameters<ListRegistriesParams>,
    ) -> Result<CallToolResult, McpError> {
        let infos: Vec<_> = registry::all_registries()
            .iter()
            .map(registry::registry_info)
            .collect();
        let json = serde_json::to_string_pretty(&infos)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {e}"), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl ServerHandler for StakedMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Package registry name availability checker. Use check_package for a single name, \
                 check_packages for bulk lookups, or list_registries to see available registries."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            ..Default::default()
        }
    }
}
