//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

use std::{
    error::Error,
    fmt::{
        Debug as StdDebug,
        Display,
        Formatter,
        Result
    }
};

crate struct CommandError(crate String);

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl StdDebug for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Command Error: {}", self.0)
    }
}

impl Error for CommandError {}