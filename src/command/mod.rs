mod echo;
mod exit;
mod r#type;

use std::collections::{BTreeMap, BTreeSet};

use crate::command::{echo::Echo, exit::Exit, r#type::Type};

pub trait Command {
    fn execute(&self) -> Result<(), ()>;
}

pub trait TryWithArgsAndContext: Sized {
    fn try_with_args_and_context(args: &str, context: Context) -> Result<Self, ()>;
}

#[derive(Clone)]
pub struct Context {
    recognized_commands: BTreeSet<String>,
}

type CreationClosure<'a> = Box<dyn Fn(&'a str, Context) -> Result<Box<dyn Command>, ()>>;

pub(crate) struct CommandParser<'a> {
    command_creators: BTreeMap<String, CreationClosure<'a>>,
}

impl<'a> CommandParser<'a> {
    pub fn new() -> Self {
        let commands = [
            (
                "exit".to_string(),
                Box::new(|s: &'a str, context: Context| {
                    Exit::try_with_args_and_context(s, context)
                        .map(|ok| Box::new(ok) as Box<dyn Command>)
                }) as Box<dyn Fn(&'a str, Context) -> Result<Box<dyn Command>, ()>>,
            ),
            (
                "echo".to_string(),
                Box::new(|s: &'a str, context: Context| {
                    Echo::try_with_args_and_context(s, context)
                        .map(|ok| Box::new(ok) as Box<dyn Command>)
                }) as Box<dyn Fn(&'a str, Context) -> Result<Box<dyn Command>, ()>>,
            ),
            (
                "type".to_string(),
                Box::new(|s: &'a str, context: Context| {
                    Type::try_with_args_and_context(s, context)
                        .map(|ok| Box::new(ok) as Box<dyn Command>)
                }) as Box<dyn Fn(&'a str, Context) -> Result<Box<dyn Command>, ()>>,
            ),
        ];
        Self {
            command_creators: BTreeMap::from_iter(commands.into_iter()),
        }
    }
}

impl<'a> CommandParser<'a> {
    pub(crate) fn parse(&self, raw_command: &'a str) -> Result<Box<dyn Command>, String> {
        let context = Context {
            recognized_commands: BTreeSet::from_iter(self.command_creators.keys().cloned()),
        };

        let (input_command, input_args) = raw_command
            .split_once(' ')
            .map(|(command, args)| (command.trim(), args.trim()))
            .unwrap_or((raw_command.trim(), ""));
        let command_creator = self
            .command_creators
            .get(input_command)
            .ok_or(format!("{input_command}: command not found"))?;

        let command = command_creator(input_args, context).unwrap();
        Ok(command)
    }
}
