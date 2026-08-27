use anyhow::Context;
use clap::{Args, ValueEnum};
use inquire::{Select, Text, validator::Validation};
use plumbum::manifest::{Manifest, ToolManager};
use std::{io::IsTerminal, path::Path};

#[derive(Debug, Args)]
pub struct InitCommand {
    /// Creates a project without dialogue
    #[arg(long, requires = "scope")]
    no_input: bool,

    /// Project name (defaults to the current directory name)
    name: Option<String>,

    /// pesde scope/namespace
    #[arg(long)]
    scope: Option<String>,

    /// Tool manager to use (defaults to rokit)
    #[arg(long)]
    tool_manager: Option<ToolManager>,
}

impl InitCommand {
    pub fn run(self, cwd: &Path) -> anyhow::Result<()> {
        let manifest_path = cwd.join("plumbum.toml");

        if manifest_path.exists() {
            anyhow::bail!("this directory is already a Plumbum project (plumbum.toml exists)");
        }

        let (name, scope, tool_manager) = self.resolve(cwd)?;

        for dir in [
            ".plumbum/configs/places",
            ".plumbum/configs/packages",
            ".plumbum/configs/scripts",
            "places",
            "packages",
            "scripts",
        ] {
            std::fs::create_dir_all(cwd.join(dir))
                .with_context(|| format!("creating dir {}", dir))?;
        }

        let manifest = Manifest {
            name,
            scope,
            version: "0.1.0".to_string(),
            tool_manager,
        };
        let toml = toml::to_string(&manifest).context("serialize manifest")?;
        std::fs::write(manifest_path, toml).context("write plumbum.toml")?;

        Ok(())
    }

    fn resolve(self, cwd: &Path) -> anyhow::Result<(String, String, ToolManager)> {
        let Self {
            no_input,
            name,
            scope,
            tool_manager,
        } = self;

        let no_input = no_input || !std::io::stdin().is_terminal();

        let name = match name {
            Some(name) => name,
            None => {
                let derived = cwd.file_name().and_then(|n| n.to_str()).map(str::to_owned);

                if no_input {
                    derived.context(
                        "cannot derive a project name from the current directory; pass <name> explicitly",
                    )?
                } else {
                    let mut prompt = Text::new("what is the name of the project?");
                    if let Some(derived) = &derived {
                        prompt = prompt.with_default(derived);
                    }
                    prompt.prompt()?
                }
            }
        };

        let scope = match scope {
            Some(scope) => scope,
            None if no_input => anyhow::bail!(
                "--scope is required when there is no interactive terminal (or with --no-input)"
            ),
            None => Text::new("what is the name of the pesde scope?")
                .with_validator(|input: &str| {
                    Ok(match validate_scope(input) {
                        Ok(()) => Validation::Valid,
                        Err(e) => Validation::Invalid(e.into()),
                    })
                })
                .prompt()?,
        };
        validate_scope(&scope).map_err(anyhow::Error::msg)?;

        let tool_manager = match tool_manager {
            Some(tool_manager) => tool_manager,
            None if no_input => ToolManager::Rokit,
            None => Select::new(
                "which tool manager are you going to use?",
                ToolManager::value_variants().to_vec(),
            )
            .prompt()?,
        };

        Ok((name, scope, tool_manager))
    }
}

fn validate_scope(input: &str) -> Result<(), &'static str> {
    if !input
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err("scope may only contain lowercase letters, digits, and underscores");
    }
    if input.chars().all(|c| c.is_ascii_digit()) {
        return Err("scope must include non-digit characters");
    }
    if !(3..=32).contains(&input.len()) {
        return Err("scope must be 3-32 characters long");
    }
    if input.starts_with('_') || input.ends_with('_') {
        return Err("scope must not start or end with an underscore");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_scope() {
        assert!(validate_scope("word30210").is_ok());
    }

    #[test]
    fn rejects_too_short() {
        assert!(validate_scope("ab").is_err());
    }

    #[test]
    fn rejects_leading_or_trailing_underscore() {
        assert!(validate_scope("_abcde").is_err());
        assert!(validate_scope("abcde_").is_err());
    }

    #[test]
    fn rejects_digits_only() {
        assert!(validate_scope("01234").is_err());
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(validate_scope("Hello, world!").is_err());
    }
}
