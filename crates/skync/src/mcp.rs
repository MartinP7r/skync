use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt, handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, schemars, tool, tool_handler, tool_router,
    transport::stdio,
};

use crate::config::Config;
use crate::discover;

#[derive(Debug, Clone)]
pub struct SkyncServer {
    config: Config,
    tool_router: ToolRouter<Self>,
}

impl SkyncServer {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tool_router: Self::tool_router(),
        }
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReadSkillRequest {
    #[schemars(description = "The skill name (directory name) to read")]
    pub name: String,
}

#[tool_router]
impl SkyncServer {
    #[tool(description = "List all skills available in the skync library")]
    fn list_skills(&self) -> Result<CallToolResult, McpError> {
        let skills = discover::discover_all(&self.config)
            .map_err(|e| McpError::internal_error(format!("discovery failed: {e}"), None))?;

        if skills.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No skills found. Run `skync init` to configure sources.",
            )]));
        }

        let mut lines = Vec::with_capacity(skills.len() + 1);
        lines.push(format!("{} skill(s) found:\n", skills.len()));
        for skill in &skills {
            lines.push(format!(
                "- {} (source: {}, path: {})",
                skill.name,
                skill.source_name,
                skill.path.display()
            ));
        }

        Ok(CallToolResult::success(vec![Content::text(
            lines.join("\n"),
        )]))
    }

    #[tool(description = "Read the SKILL.md content of a skill by name")]
    fn read_skill(
        &self,
        Parameters(ReadSkillRequest { name }): Parameters<ReadSkillRequest>,
    ) -> Result<CallToolResult, McpError> {
        let skills = discover::discover_all(&self.config)
            .map_err(|e| McpError::internal_error(format!("discovery failed: {e}"), None))?;

        let skill = skills.iter().find(|s| s.name == name);

        match skill {
            Some(skill) => {
                let skill_md = skill.path.join("SKILL.md");
                let content = std::fs::read_to_string(&skill_md).map_err(|e| {
                    McpError::internal_error(
                        format!("failed to read {}: {e}", skill_md.display()),
                        None,
                    )
                })?;
                Ok(CallToolResult::success(vec![Content::text(content)]))
            }
            None => Ok(CallToolResult::error(vec![Content::text(format!(
                "Skill '{}' not found. Use list_skills to see available skills.",
                name
            ))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for SkyncServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Skync MCP server — exposes discovered AI coding skills for reading".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "skync-mcp".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

/// Start the MCP server on stdio.
pub async fn serve(config: Config) -> anyhow::Result<()> {
    let server = SkyncServer::new(config);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
