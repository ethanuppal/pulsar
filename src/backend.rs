use crate::ir::generator::GeneratedTopLevel;

pub mod calyx_backend;

// This interface hasn't been finalized yet, so it is quite sloppy as written

pub trait PulsarBackend {
    type ExtraInput;
    type Error;

    fn new() -> Self;
    fn run(
        &mut self, code: Vec<GeneratedTopLevel>, input: Self::ExtraInput
    ) -> Result<(), Self::Error>;
}
