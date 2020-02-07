// Load before other modules which depend on macros in here.
#[macro_use]
mod util;
mod data;
pub mod build;
pub mod deploy;
pub mod test;

const DOCUMENT_ID_BYTES: usize = 4;

type Term = Vec<u8>;
type TermId = usize;