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

// ResolutionOfInvestigationV13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ResolutionOfInvestigationV13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnmt") )]
	pub assgnmt: CaseAssignment6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: InvestigationStatus6Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlDtls", skip_serializing_if = "Option::is_none") )]
	pub cxl_dtls: Option<Vec<UnderlyingTransaction32>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModDtls", skip_serializing_if = "Option::is_none") )]
	pub mod_dtls: Option<PaymentTransaction157>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClmNonRctDtls", skip_serializing_if = "Option::is_none") )]
	pub clm_non_rct_dtls: Option<ClaimNonReceipt3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtDtls", skip_serializing_if = "Option::is_none") )]
	pub stmt_dtls: Option<StatementResolutionEntry5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CrrctnTx", skip_serializing_if = "Option::is_none") )]
	pub crrctn_tx: Option<CorrectiveTransaction5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none") )]
	pub rsltn_rltd_inf: Option<ResolutionData5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ResolutionOfInvestigationV13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.assgnmt.validate()?;
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		self.sts.validate()?;
		if let Some(ref vec) = self.cxl_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.mod_dtls { val.validate()? }
		if let Some(ref val) = self.clm_non_rct_dtls { val.validate()? }
		if let Some(ref val) = self.stmt_dtls { val.validate()? }
		if let Some(ref val) = self.crrctn_tx { val.validate()? }
		if let Some(ref val) = self.rsltn_rltd_inf { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
