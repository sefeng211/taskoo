mod add;
mod delete;
mod get;
mod modify;
mod view;

pub use add::*;
pub use delete::*;
pub use get::*;
pub use modify::*;
pub use view::*;

use crate::core::Operation;
use crate::error::TaskooError;

pub fn execute(op: &mut impl Operation) -> Result<(), TaskooError> {
    op.init();
    op.do_work().map(|tasks| {
        op.set_result(tasks);
    })?;
    Ok(())
}