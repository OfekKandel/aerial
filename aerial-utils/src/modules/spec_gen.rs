use std::collections::HashMap;
use clap::{Command, CommandFactory};
use serde::Serialize;
use crate::AerialUtilsArgs;


pub fn print_subcommand_specs() {
    let mut cmd = AerialUtilsArgs::command();
    cmd.build();
    print_chatgpt_subcommands(&cmd);
}

fn print_chatgpt_subcommands(cmd: &Command) {
    let subcommands = get_chatgpt_subcommands(cmd, "");
    println!("{}", serde_json::to_string_pretty(&subcommands).unwrap());
}

fn get_chatgpt_subcommands(cmd: &Command, prefix: &str) -> Vec<ChatGPTCommand> {
    let children_prefix = match cmd.get_name() {
        "aerial-utils" => "".into(),
        _ => format!("{}{}_", prefix, cmd.get_name())
    };
    let mut subcommands = Vec::new();
    for subcommand in cmd.get_subcommands().filter(|s| s.get_name() != "help") {
        if subcommand.has_subcommands() {
            subcommands.extend(get_chatgpt_subcommands(subcommand, &children_prefix).into_iter());
        } else {
            subcommands.push(ChatGPTCommand::from_cmd(subcommand, children_prefix.clone()));
        }
    }
    subcommands

}
#[derive(Serialize)]
pub struct ChatGPTCommand {
    #[serde(rename = "type")]
    cmd_type: String,
    function: ChatGPTFunction
}

impl ChatGPTCommand {
    pub fn from_cmd(cmd: &Command, path: String) -> Self {
        Self {
            cmd_type: "function".into(),
            function: ChatGPTFunction {
                name: format!("{}{}", path, cmd.get_name()),
                description: cmd.get_about().map(|txt| txt.to_string()).unwrap_or("No Description".into()),
                parameters: ChatGPTFunctionParams {
                    param_type: "object".into(),
                    properties: HashMap::new(),
                    required: Vec::new(),
                }
            }
        }
    }
}

#[derive(Serialize)]
pub struct ChatGPTFunction {
    name: String,
    description: String,
    parameters: ChatGPTFunctionParams
}

#[derive(Serialize)]
pub struct ChatGPTFunctionParams {
    #[serde(rename = "type")]
    param_type: String,
    properties: HashMap<String, ChatGPTFunctionProperty>,
    required: Vec<String>,
}

#[derive(Serialize)]
pub struct ChatGPTFunctionProperty {
    #[serde(rename = "type")]
    property_type: String,
    description: String,
}