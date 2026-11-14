mod logging;
mod manifest;
mod reader;
mod seeder;

pub use reader::EmbeddedReader;
pub(crate) use seeder::{Embedded, SeedError};
