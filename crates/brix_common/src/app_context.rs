// Copyright (c) 2021 Ethan Lerner, Caleb Cushing, and the Brix contributors
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use brix_cli::Config;
use brix_processor::ProcessorCore;

/// Struct containing common properties that require access during command execution.
pub struct AppContext<'a> {
    pub processor: ProcessorCore<'a>,
    pub config: &'a Config,
}
