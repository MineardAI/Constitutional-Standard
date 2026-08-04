//! Structural/contextual validation. Validation findings do not create authority.

use constitutional_contracts::*;

pub mod verification;
pub use verification::*;

pub fn validate_context(context: &OperationalConstitutionalContext) -> ValidationResult {
    let mut result = ValidationResult::success();
    if context.constitutional_effective_time == context.processing_time {
        result.findings.push(ValidationFinding { code: FindingCode::ConflictingInput, severity: FindingSeverity::Warning, subject: context.operation_id.to_string(), detail: "processing time must remain distinguishable from constitutional effective time when they are different observations".into() });
    }
    result
}
