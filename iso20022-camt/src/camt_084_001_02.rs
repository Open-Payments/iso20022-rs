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

// IntraBalanceMovementPostingReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IntraBalanceMovementPostingReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<DocumentIdentification51>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pgntn") )]
	pub pgntn: Pagination1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptGnlDtls") )]
	pub rpt_gnl_dtls: IntraBalanceReport6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
	pub csh_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_ownr: Option<SystemPartyIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubBal", skip_serializing_if = "Option::is_none") )]
	pub sub_bal: Option<Vec<IntraBalancePosting5>>,
}

impl IntraBalanceMovementPostingReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		self.pgntn.validate()?;
		self.rpt_gnl_dtls.validate()?;
		self.csh_acct.validate()?;
		if let Some(ref val) = self.csh_acct_ownr { val.validate()? }
		if let Some(ref val) = self.csh_acct_svcr { val.validate()? }
		if let Some(ref vec) = self.sub_bal { for item in vec { item.validate()? } }
		Ok(())
	}
}