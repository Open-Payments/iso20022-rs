// Open Payment Message Parsing Library
// https://github.com/Open-Payments/iso20022-rs
//
// This library is designed to parse message formats based on the ISO 20022 standards,
// including but not limited to FedNow messages. It supports various financial message types,
// such as customer credit transfers, payment status reports, administrative notifications, 
// and other ISO 20022 messages, using Serde for efficient serialization and deserialization.
//
// Copyright (c) 2024 Open Payments
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/Open-Payments/iso20022-rs


#![allow(unused_imports)]
use regex::Regex;
use iso20022_common::{common::*, ValidationError};
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

// InformationRequestStatusChangeNotificationV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InformationRequestStatusChangeNotificationV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry") )]
	pub orgnl_biz_qry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnfdtltySts") )]
	pub cnfdtlty_sts: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl InformationRequestStatusChangeNotificationV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_biz_qry.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_biz_qry is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_biz_qry.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_biz_qry exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
