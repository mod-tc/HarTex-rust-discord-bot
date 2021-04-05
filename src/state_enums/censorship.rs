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

use crate::{
    system::{
        state_machine::StateEnum
    }
};

#[derive(Debug, Clone)]
crate enum CensorshipProcess {
    Initialized,

    ZalgoFiltered,

    InvitesFiltered,

    DomainsFiltered,

    BlockedWordsOrTokensFiltered,

    BlockedMentionsFiltered,

    ConsecutiveCapitalLettersFiltered,

    Completed
}

impl StateEnum for CensorshipProcess {}

impl PartialEq for CensorshipProcess {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone)]
crate enum NicknameCensorshipProcess {
    Initialized,

    ZalgoNicknamesFiltered,

    BlockedNicknamesFiltered,

    BlockedNicknamePrefixesFiltered,

    Completed
}

impl StateEnum for NicknameCensorshipProcess {}

impl PartialEq for CensorshipProcess {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
