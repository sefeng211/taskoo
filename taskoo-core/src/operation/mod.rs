mod add;
mod agenda;
mod delete;
mod get;
mod modify;
mod view;

pub use add::*;
pub use delete::*;
pub use get::*;
pub use modify::*;
pub use view::*;
pub use agenda::*;

use crate::core::Operation;
use crate::error::CoreError;
pub use crate::db::task_helper::Task;

pub fn execute(op: &mut impl Operation) -> Result<(), CoreError> {
    op.init()?;
    op.do_work().map(|tasks| {
        op.set_result(tasks);
    })?;
    Ok(())
}

// TODO: This needs to be handled better
pub fn execute_agenda(op: &mut Agenda) -> Result<(), CoreError> {
    op.init()?;
    op.do_work_for_agenda().map(|tasks| {
        op.set_result(tasks);
    })?;
    Ok(())
}
