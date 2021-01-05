// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use core::fmt;

/// Errors for Dusk PKI
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    /// Invalid Compressed Point"
    InvalidPoint,
    /// Invalid Parameters
    InvalidParameters,
    /// Bad Length
    BadLength {
        /// The found length
        found: usize,
        /// The expected length
        expected: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dusk PKI Error: {:?}", &self)
    }
}
