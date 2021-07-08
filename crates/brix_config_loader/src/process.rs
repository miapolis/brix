use lazy_static::lazy_static;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

use brix_commands::{CopyCommand, SearchReplaceCommand, TemplateCommand};
use brix_errors::BrixError;

use crate::context::{cli_config_to_map, ContextMap};
use crate::ConfigLoader;
use crate::{Command, CommandList, RawConfig};
use crate::{ProcessedCommandParams, RawCommandParams};

lazy_static! {
    static ref SUPPORTED_COMMANDS: Vec<&'static str> = vec!["copy", "search_replace"];
}

impl<'a> ConfigLoader<'a> {
    pub fn process(&self, config: &RawConfig) -> Result<CommandList, BrixError> {
        let mut list = CommandList::new();

        for command in config.commands.iter() {
            let key = command.keys().next().unwrap();
            let value = command.values().next().unwrap();
            let command: Box<dyn Command> = match key.to_lowercase().as_str() {
                "copy" => Box::new(CopyCommand::new()),
                "search_replace" => Box::new(SearchReplaceCommand::new()),
                "template" => Box::new(TemplateCommand::new()),
                _ => {
                    let matches =
                        difflib::get_close_matches(key, SUPPORTED_COMMANDS.to_vec(), 1, 0.6);
                    if let Some(closest) = matches.get(0) {
                        return Err(BrixError::with(&format!(
                            "command '{}' not found... did you mean '{}'?",
                            key, closest
                        )));
                    } else {
                        return Err(BrixError::with(&format!("command '{}' not found", key)));
                    }
                }
            };

            // Serialize the data into json
            let json = json!(value);
            // Read context
            let local_context = value.context.clone().unwrap_or(HashMap::new());
            // Create context map and populate accordingly
            let context_map = ContextMap {
                cli_positional: cli_config_to_map(self.cli_config),
                config_global: config.context.clone().unwrap_or(HashMap::new()),
                command_local: local_context,
            };
            // Merge contexts together
            let context = context_map.do_merge();

            let processor_context = brix_processor::create_context(context);
            let res = brix_processor::process(json.to_string(), processor_context)?;
            let raw_args: RawCommandParams = serde_json::from_str(&res).unwrap();
            let args = self.create_processed_args(&raw_args)?;

            list.push((command, args));
        }

        Ok(list)
    }

    fn create_processed_args(
        &self,
        raw: &RawCommandParams,
    ) -> Result<ProcessedCommandParams, BrixError> {
        let config = self.config_dir.as_ref().unwrap();

        let mut source = None;
        let mut destination = None;
        let mut overwrite = None;
        let mut search = None;
        let mut replace = None;
        let mut context = None;

        if let Some(raw_source) = &raw.source {
            source = Some(config.join(raw_source)); // Source is relative to config
        }
        if let Some(raw_destination) = &raw.destination {
            destination = Some(PathBuf::from(raw_destination)); // Dest is absolute path
        }
        if let Some(raw_overwrite) = raw.overwrite {
            overwrite = Some(raw_overwrite);
        }
        if let Some(raw_search) = &raw.search {
            search = Some(raw_search.clone());
        }
        if let Some(raw_replace) = &raw.replace {
            replace = Some(raw_replace.clone());
        }
        if let Some(raw_context) = &raw.context {
            context = Some(raw_context.clone());
        }

        Ok(ProcessedCommandParams {
            source,
            destination,
            overwrite,
            search,
            replace,
            context,
        })
    }
}
