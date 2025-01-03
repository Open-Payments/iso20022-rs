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

// IntraBalanceMovementModificationRequestStatusAdviceV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IntraBalanceMovementModificationRequestStatusAdviceV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<DocumentIdentification51>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqRef") )]
	pub req_ref: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
	pub csh_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_ownr: Option<SystemPartyIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqDtls", skip_serializing_if = "Option::is_none") )]
	pub req_dtls: Option<RequestDetails22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgSts") )]
	pub prcg_sts: ProcessingStatus71Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygIntraBal", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_intra_bal: Option<IntraBalance5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl IntraBalanceMovementModificationRequestStatusAdviceV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if self.req_ref.chars().count() < 1 {
			return Err(ValidationError::new(1001, "req_ref is shorter than the minimum length of 1".to_string()));
		}
		if self.req_ref.chars().count() > 35 {
			return Err(ValidationError::new(1002, "req_ref exceeds the maximum length of 35".to_string()));
		}
		self.csh_acct.validate()?;
		if let Some(ref val) = self.csh_acct_ownr { val.validate()? }
		if let Some(ref val) = self.csh_acct_svcr { val.validate()? }
		if let Some(ref val) = self.req_dtls { val.validate()? }
		self.prcg_sts.validate()?;
		if let Some(ref val) = self.undrlyg_intra_bal { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}
