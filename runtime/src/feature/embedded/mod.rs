mod manifest;
mod logging;
mod reader;
mod seeder;

pub use reader::EmbeddedReader;
pub(crate) use seeder::{Embedded, SeedError};
