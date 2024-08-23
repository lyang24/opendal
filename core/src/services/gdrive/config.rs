// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// [GoogleDrive](https://drive.google.com/) configuration.
#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(default)]
#[non_exhaustive]
pub struct GdriveConfig {
    /// The root for gdrive
    pub root: Option<String>,
    /// Access token for gdrive.
    pub access_token: Option<String>,
    /// Refresh token for gdrive.
    pub refresh_token: Option<String>,
    /// Client id for gdrive.
    pub client_id: Option<String>,
    /// Client secret for gdrive.
    pub client_secret: Option<String>,
}

impl Debug for GdriveConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GdriveConfig")
            .field("root", &self.root)
            .finish_non_exhaustive()
    }
}
