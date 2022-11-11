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

pub fn execute2(op: &mut impl Operation) -> Result<&Task, CoreError> {
    op.init()?;
    op.do_work().map(|tasks| {
        op.set_result(tasks);
    })?;

    let tasks = op.get_result();
    println!("{:?}", tasks[0]);
    Ok(&tasks[0])
}

// TODO: This needs to be handled better
pub fn execute_agenda(op: &mut Agenda) -> Result<(), CoreError> {
    op.init()?;
    op.do_work_for_agenda().map(|tasks| {
        op.set_result(tasks);
    })?;
    Ok(())
}
