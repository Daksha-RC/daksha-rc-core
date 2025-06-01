use chrono::{DateTime, Utc};
use disintegrate::{Event, StateMutate, StateQuery};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use thiserror::Error;
use utoipa::ToSchema;

pub type ClaimId = i64;
#[derive(Debug, Clone, PartialEq, Eq, Event, Serialize, Deserialize)]
#[stream(ClaimStateEvent, [ClaimSubmitted, ClaimVerified,ClaimVerificationFailed,ClaimApproved,ClaimRejected])]
pub enum ClaimEvent {
    ClaimSubmitted {
        #[id]
        claim_id: ClaimId,
        claimant_id: String,
        claim_data: Option<String>,
        timestamp: DateTime<Utc>,
    },
    ClaimVerified {
        #[id]
        claim_id: ClaimId,
        verifier_id: String,
        verified_at: DateTime<Utc>,
    },
    ClaimVerificationFailed {
        #[id]
        claim_id: ClaimId,
        verifier_id: String,
        reason: String,
        failed_at: DateTime<Utc>,
    },
    ClaimApproved {
        #[id]
        claim_id: ClaimId,
        approver_id: String,
        approved_at: DateTime<Utc>,
    },
    ClaimRejected {
        #[id]
        claim_id: ClaimId,
        rejector_id: String,
        reason: String,
        rejected_at: DateTime<Utc>,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ClaimError {
    #[error("Claim not found {0}")]
    ClaimNotFound(String),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Display, EnumString, ToSchema,
)]
pub enum ClaimStatus {
    #[default]
    None,
    Submitted,
    Approved,
    Rejected,
    Verified,
    Invalid,
    ValidationFailed,
}

#[derive(Default, StateQuery, Clone, Debug, Serialize, Deserialize)]
#[state_query(AccountBalanceEvent)]
pub struct Claim {
    #[id]
    claim_id: ClaimId,
    entity: String,
    entity_id: String,
    status: ClaimStatus,
    claim_data: Option<String>,
    attestation_id: Option<String>,
    attestation_name: Option<String>,
    verifier_id: Option<String>,
    timestamp: DateTime<Utc>,
}
impl Claim {
    pub fn new(claim_id: ClaimId) -> Self {
        Self {
            claim_id,
            ..Default::default()
        }
    }
}

impl StateMutate for Claim {
    fn mutate(&mut self, event: Self::Event) {
        match event {
            ClaimEvent::ClaimSubmitted {
                claim_id,
                claimant_id,
                claim_data,
                ..
            } => {
                self.status = ClaimStatus::Submitted;
                self.claim_id = claim_id;
                self.entity_id = claimant_id;
                self.claim_data = claim_data;
            }
            ClaimEvent::ClaimVerified { .. } => {
                self.status = ClaimStatus::Verified;
            }
            ClaimEvent::ClaimVerificationFailed { .. } => {
                self.status = ClaimStatus::Invalid;
            }
            ClaimEvent::ClaimApproved { .. } => {
                self.status = ClaimStatus::Approved;
            }
            ClaimEvent::ClaimRejected { .. } => {
                self.status = ClaimStatus::Rejected;
            }
        }
    }
}

// Commands
pub struct SubmitClaim {
    claim_id: String,
    entity: String,
    entity_id: String,
    claim_data: Option<String>,
}

impl SubmitClaim {}

pub struct VerifyClaim {
    claim_id: String,
    entity: String,
    entity_id: String,
}
impl VerifyClaim {}

pub struct ApproveClaim {
    claim_id: String,
}
pub struct RejectClaim {
    claim_id: String,
}
