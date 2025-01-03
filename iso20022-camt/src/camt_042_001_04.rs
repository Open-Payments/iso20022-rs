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

// FundDetailedEstimatedCashForecastReportV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundDetailedEstimatedCashForecastReportV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolRef", skip_serializing_if = "Option::is_none") )]
	pub pool_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn") )]
	pub msg_pgntn: Pagination,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndOrSubFndDtls", skip_serializing_if = "Option::is_none") )]
	pub fnd_or_sub_fnd_dtls: Option<Fund3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EstmtdFndCshFcstDtls") )]
	pub estmtd_fnd_csh_fcst_dtls: Vec<EstimatedFundCashForecast5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsltdNetCshFcst", skip_serializing_if = "Option::is_none") )]
	pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl FundDetailedEstimatedCashForecastReportV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.pool_ref { val.validate()? }
		if let Some(ref vec) = self.prvs_ref { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rltd_ref { for item in vec { item.validate()? } }
		self.msg_pgntn.validate()?;
		if let Some(ref val) = self.fnd_or_sub_fnd_dtls { val.validate()? }
		for item in &self.estmtd_fnd_csh_fcst_dtls { item.validate()? }
		if let Some(ref val) = self.cnsltd_net_csh_fcst { val.validate()? }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}
