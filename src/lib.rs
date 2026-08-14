pub mod board;
pub mod generate;
pub mod known;
pub mod rules;
pub mod solve;
pub mod ui;
pub mod utils;

pub use board::*;
pub use generate::*;
pub use known::*;
pub use rules::*;
pub use solve::*;
pub use ui::*;
pub use utils::*;

#[cfg(feature="mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature="mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
