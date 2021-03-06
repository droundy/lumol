// Lumol, an extensible molecular simulation engine
// Copyright (C) Lumol's contributors — BSD license

//! Parallelism related utilities

pub mod prelude;

mod shortcuts;
pub use self::shortcuts::ParallelShortcuts;

mod thread_local_store;
pub use self::thread_local_store::ThreadLocalStore;
