use crate::commands::args::ArgSpec;
use crate::commands::context::CommandContext;
use crate::commands::error::CommandError;

pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn args(&self) -> Vec<ArgSpec> {
        Vec::new()
    }

    fn overloads(&self) -> Vec<Vec<ArgSpec>> {
        vec![self.args()]
    }

    fn execute(&self, ctx: &mut CommandContext<'_>) -> Result<(), CommandError>;
}
