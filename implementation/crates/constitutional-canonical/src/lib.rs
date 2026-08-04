//! Canonical Operational Constitutional Context representation.
//!
//! The selected profile is a readable, UTF-8, line-oriented reference encoding.
//! It is not a universal MACS wire protocol, constitutional meaning, identity,
//! authority, truth, applicability, conformance, or certification mechanism.

use constitutional_contracts::{
    AdmissionReportId, AdmittedSource, AdmittedSourceSet, BaselineRef, BoundaryContextRef,
    ConstitutionalSourceId, ConstitutionalSourceRef, ConstitutionalSourceVersion, ContextId,
    ContractError, EffectiveContextRef, ImplementationProfileRef, ImplementationVersion,
    JurisdictionRef, OperationId, OperationalConstitutionalContext, RepresentationFormatId,
    RepresentationVersion, ScopeRef, SourceAdmissionStatus, SourceContentDigest, SourceFamily,
    SourceIdentity, SourceRootId, SourceSetId, TimePoint,
};
use std::collections::HashSet;
use std::fmt;

pub mod representation;
pub use representation::*;

pub const FORMAT_ID: &str = "macs-reference-canonical-context";
pub const FORMAT_VERSION: &str = "1.0.0";
pub const AUTHORITY_FORMAT_ID: &str = "macs-reference-canonical-authority-evaluation";
pub const AUTHORITY_FORMAT_VERSION: &str = "1.0.0";
pub const IDENTITY_FORMAT_ID: &str = "macs-reference-canonical-identity-evaluation";
pub const PARTICIPATION_FORMAT_ID: &str = "macs-reference-canonical-participation-evaluation";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalAuthorityRepresentation {
    pub fields: Vec<(String, String)>,
}

pub fn authority_canonicalize(
    result: &constitutional_authority::AuthorityEvaluationResult,
) -> CanonicalAuthorityRepresentation {
    let claim = result.claim.as_ref();
    let scope = claim
        .map(|claim| {
            format!(
                "{}|{}|{}|{}|{}",
                claim.scope.subject,
                claim.scope.act,
                claim.scope.domain,
                claim.scope.operational_boundary,
                claim
                    .scope
                    .exclusions
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .unwrap_or_default();
    let basis = result
        .basis
        .as_ref()
        .map(|basis| {
            format!(
                "{}|{}|{}|{}|{}",
                basis.authority,
                basis.source.id,
                basis.source.version,
                basis.source.path,
                basis.assignment_holder
            )
        })
        .unwrap_or_default();
    let findings = result
        .findings
        .iter()
        .map(|finding| {
            format!(
                "{:?}:{}:{}:{}",
                finding.code, finding.subject, finding.detail, finding.fatal
            )
        })
        .collect::<Vec<_>>()
        .join(";");
    CanonicalAuthorityRepresentation {
        fields: vec![
            ("format_id".into(), AUTHORITY_FORMAT_ID.into()),
            (
                "representation_version".into(),
                AUTHORITY_FORMAT_VERSION.into(),
            ),
            ("evaluation_id".into(), result.evaluation_id.to_string()),
            ("operation_id".into(), result.operation_id.to_string()),
            ("context_id".into(), result.context_id.to_string()),
            (
                "source_set_id".into(),
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "authority_profile_version".into(),
                result.authority_profile_version.to_string(),
            ),
            (
                "implementation_version".into(),
                result.implementation_version.to_string(),
            ),
            (
                "determination".into(),
                format!("{:?}", result.determination),
            ),
            (
                "claim".into(),
                claim
                    .map(|claim| {
                        format!(
                            "{}|{}|{}|{}|{}|{}",
                            claim.authority,
                            claim.holder,
                            claim.subject,
                            claim.act,
                            claim.jurisdiction,
                            scope
                        )
                    })
                    .unwrap_or_default(),
            ),
            ("basis".into(), basis),
            (
                "jurisdiction".into(),
                result
                    .jurisdiction
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("findings".into(), findings),
            (
                "execution_performed".into(),
                result.execution_performed.to_string(),
            ),
            ("non_claims".into(), result.non_claims.join(";")),
        ],
    }
}

pub fn authority_encode(
    result: &constitutional_authority::AuthorityEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    let representation = authority_canonicalize(result);
    let mut output = String::new();
    for (key, value) in representation.fields {
        output.push_str(&key);
        output.push('=');
        output.push_str(&authority_escape(&value));
        output.push('\n');
    }
    Ok(output.into_bytes())
}

pub fn authority_decode(
    bytes: &[u8],
) -> Result<CanonicalAuthorityRepresentation, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let expected = [
        "format_id",
        "representation_version",
        "evaluation_id",
        "operation_id",
        "context_id",
        "source_set_id",
        "authority_profile_version",
        "implementation_version",
        "determination",
        "claim",
        "basis",
        "jurisdiction",
        "findings",
        "execution_performed",
        "non_claims",
    ];
    let mut fields = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if index >= expected.len() || key != expected[index] {
            return Err(CanonicalDecodeError::NonCanonicalOrdering);
        }
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.len() != expected.len() {
        return Err(CanonicalDecodeError::MissingRequiredField(
            "authority evaluation field".into(),
        ));
    }
    if fields[0].1 != AUTHORITY_FORMAT_ID {
        return Err(CanonicalDecodeError::UnsupportedFormat(fields[0].1.clone()));
    }
    if fields[1].1 != AUTHORITY_FORMAT_VERSION {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields[1].1.clone(),
        ));
    }
    Ok(CanonicalAuthorityRepresentation { fields })
}

fn authority_escape(value: &str) -> String {
    value
        .replace('%', "%25")
        .replace('\n', "%0A")
        .replace('=', "%3D")
}
fn authority_unescape(value: &str) -> Result<String, CanonicalDecodeError> {
    let mut output = String::new();
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(CanonicalDecodeError::MalformedLine(value.into()));
            }
            let code = &value[index + 1..index + 3];
            match code {
                "25" => output.push('%'),
                "0A" => output.push('\n'),
                "3D" => output.push('='),
                _ => return Err(CanonicalDecodeError::MalformedLine(value.into())),
            }
            index += 3;
        } else {
            output.push(bytes[index] as char);
            index += 1;
        }
    }
    Ok(output)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalIdentityRepresentation {
    pub fields: Vec<(String, String)>,
}

pub fn identity_canonicalize(
    result: &constitutional_identity::IdentityEvaluationResult,
) -> CanonicalIdentityRepresentation {
    let claim = result
        .claim
        .as_ref()
        .map(|claim| {
            format!(
                "{}|{}|{}|{}",
                claim.identity,
                claim.subject,
                claim.distinction.distinction_key,
                claim
                    .representation
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default();
    CanonicalIdentityRepresentation {
        fields: vec![
            ("format_id".into(), IDENTITY_FORMAT_ID.into()),
            (
                "representation_version".into(),
                AUTHORITY_FORMAT_VERSION.into(),
            ),
            ("evaluation_id".into(), result.evaluation_id.to_string()),
            ("operation_id".into(), result.operation_id.to_string()),
            ("context_id".into(), result.context_id.to_string()),
            (
                "source_set_id".into(),
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version".into(),
                result.implementation_version.to_string(),
            ),
            ("profile_version".into(), result.profile_version.to_string()),
            (
                "determination".into(),
                format!("{:?}", result.determination),
            ),
            ("claim".into(), claim),
            (
                "continuity".into(),
                result
                    .continuity
                    .as_ref()
                    .map(|item| {
                        format!(
                            "{}|{:?}|{}",
                            item.prior_identity,
                            item.kind,
                            item.basis
                                .as_ref()
                                .map(ToString::to_string)
                                .unwrap_or_default()
                        )
                    })
                    .unwrap_or_default(),
            ),
            (
                "findings".into(),
                result
                    .findings
                    .iter()
                    .map(|item| {
                        format!(
                            "{:?}:{}:{}:{}",
                            item.code, item.subject, item.detail, item.fatal
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            (
                "authentication_performed".into(),
                result.authentication_performed.to_string(),
            ),
            (
                "identity_created".into(),
                result.identity_created.to_string(),
            ),
            ("non_claims".into(), result.non_claims.join(";")),
        ],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalParticipationRepresentation {
    pub fields: Vec<(String, String)>,
}

pub fn participation_canonicalize(
    result: &constitutional_identity::ParticipationEvaluationResult,
) -> CanonicalParticipationRepresentation {
    let claim = result
        .claim
        .as_ref()
        .map(|claim| {
            format!(
                "{}|{}|{}|{}",
                claim.participation,
                claim.subject,
                claim.identity.evaluation_id,
                claim
                    .scope
                    .as_ref()
                    .map(|scope| format!(
                        "{}|{}|{}",
                        scope.activity, scope.institution, scope.process
                    ))
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default();
    CanonicalParticipationRepresentation {
        fields: vec![
            ("format_id".into(), PARTICIPATION_FORMAT_ID.into()),
            (
                "representation_version".into(),
                AUTHORITY_FORMAT_VERSION.into(),
            ),
            ("evaluation_id".into(), result.evaluation_id.to_string()),
            ("operation_id".into(), result.operation_id.to_string()),
            ("context_id".into(), result.context_id.to_string()),
            (
                "source_set_id".into(),
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version".into(),
                result.implementation_version.to_string(),
            ),
            ("profile_version".into(), result.profile_version.to_string()),
            (
                "determination".into(),
                format!("{:?}", result.determination),
            ),
            ("claim".into(), claim),
            (
                "findings".into(),
                result
                    .findings
                    .iter()
                    .map(|item| {
                        format!(
                            "{:?}:{}:{}:{}",
                            item.code, item.subject, item.detail, item.fatal
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            (
                "execution_performed".into(),
                result.execution_performed.to_string(),
            ),
            (
                "participation_activated".into(),
                result.participation_activated.to_string(),
            ),
            ("non_claims".into(), result.non_claims.join(";")),
        ],
    }
}

pub fn identity_encode(
    result: &constitutional_identity::IdentityEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    encode_identity_fields(identity_canonicalize(result).fields)
}
pub fn participation_encode(
    result: &constitutional_identity::ParticipationEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    encode_identity_fields(participation_canonicalize(result).fields)
}

pub fn identity_decode(
    bytes: &[u8],
) -> Result<CanonicalIdentityRepresentation, CanonicalDecodeError> {
    let fields = decode_identity_fields(bytes, IDENTITY_FORMAT_ID, 15)?;
    Ok(CanonicalIdentityRepresentation { fields })
}
pub fn participation_decode(
    bytes: &[u8],
) -> Result<CanonicalParticipationRepresentation, CanonicalDecodeError> {
    let fields = decode_identity_fields(bytes, PARTICIPATION_FORMAT_ID, 14)?;
    Ok(CanonicalParticipationRepresentation { fields })
}

fn encode_identity_fields(fields: Vec<(String, String)>) -> Result<Vec<u8>, CanonicalDecodeError> {
    let mut output = String::new();
    for (key, value) in fields {
        output.push_str(&key);
        output.push('=');
        output.push_str(&authority_escape(&value));
        output.push('\n');
    }
    Ok(output.into_bytes())
}
fn decode_identity_fields(
    bytes: &[u8],
    format: &str,
    count: usize,
) -> Result<Vec<(String, String)>, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let expected_identity = [
        "format_id",
        "representation_version",
        "evaluation_id",
        "operation_id",
        "context_id",
        "source_set_id",
        "implementation_version",
        "profile_version",
        "determination",
        "claim",
        "continuity",
        "findings",
        "authentication_performed",
        "identity_created",
        "non_claims",
    ];
    let expected_participation = [
        "format_id",
        "representation_version",
        "evaluation_id",
        "operation_id",
        "context_id",
        "source_set_id",
        "implementation_version",
        "profile_version",
        "determination",
        "claim",
        "findings",
        "execution_performed",
        "participation_activated",
        "non_claims",
    ];
    let expected = if count == 15 {
        &expected_identity[..]
    } else {
        &expected_participation[..]
    };
    let mut fields = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if index >= expected.len() || key != expected[index] {
            return Err(CanonicalDecodeError::NonCanonicalOrdering);
        }
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.len() != count {
        return Err(CanonicalDecodeError::MissingRequiredField(
            "identity/participation field".into(),
        ));
    }
    if fields[0].1 != format {
        return Err(CanonicalDecodeError::UnsupportedFormat(fields[0].1.clone()));
    }
    if fields[1].1 != AUTHORITY_FORMAT_VERSION {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields[1].1.clone(),
        ));
    }
    Ok(fields)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalContextRepresentation {
    pub format_id: RepresentationFormatId,
    pub representation_version: RepresentationVersion,
    pub context_id: Option<ContextId>,
    pub admitted_source_set: Option<AdmittedSourceSet>,
    pub operation_id: OperationId,
    pub source_set: Vec<ConstitutionalSourceRef>,
    pub baseline: BaselineRef,
    pub jurisdiction: JurisdictionRef,
    pub scope: ScopeRef,
    pub effective_context: EffectiveContextRef,
    pub boundary_context: BoundaryContextRef,
    pub implementation_profile: Option<ImplementationProfileRef>,
    pub constitutional_effective_time: TimePoint,
    pub observation_time: TimePoint,
    pub processing_time: TimePoint,
    pub implementation_version: ImplementationVersion,
}

impl CanonicalContextRepresentation {
    pub fn from_context(context: &OperationalConstitutionalContext) -> Self {
        let mut source_set = context.source_set.clone();
        source_set.sort_by_key(source_key);
        Self {
            format_id: RepresentationFormatId::new(FORMAT_ID).expect("profile constant is valid"),
            representation_version: RepresentationVersion::new(FORMAT_VERSION)
                .expect("version constant is valid"),
            context_id: None,
            admitted_source_set: context.admitted_source_set.clone(),
            operation_id: context.operation_id.clone(),
            source_set,
            baseline: context.baseline.clone(),
            jurisdiction: context.jurisdiction.clone(),
            scope: context.scope.clone(),
            effective_context: context.effective_context.clone(),
            boundary_context: context.boundary_context.clone(),
            implementation_profile: None,
            constitutional_effective_time: context.constitutional_effective_time.clone(),
            observation_time: context.observation_time.clone(),
            processing_time: context.processing_time.clone(),
            implementation_version: context.implementation_version.clone(),
        }
    }

    pub fn to_context(&self) -> OperationalConstitutionalContext {
        OperationalConstitutionalContext {
            operation_id: self.operation_id.clone(),
            source_set: self.source_set.clone(),
            baseline: self.baseline.clone(),
            jurisdiction: self.jurisdiction.clone(),
            scope: self.scope.clone(),
            effective_context: self.effective_context.clone(),
            boundary_context: self.boundary_context.clone(),
            constitutional_effective_time: self.constitutional_effective_time.clone(),
            observation_time: self.observation_time.clone(),
            processing_time: self.processing_time.clone(),
            implementation_version: self.implementation_version.clone(),
            admitted_source_set: self.admitted_source_set.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalDecodeError {
    InvalidUtf8,
    MalformedLine(String),
    UnknownField(String),
    DuplicateField(String),
    MissingRequiredField(String),
    UnsupportedFormat(String),
    UnsupportedVersion(String),
    InvalidValue { field: String, detail: String },
    DuplicateSourceReference,
    AmbiguousSourceReference(String),
    ConflictingSourceVersion(String),
    NonCanonicalOrdering,
    TrailingData,
    RoundTripMismatch,
}

impl fmt::Display for CanonicalDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn canonicalize(context: &OperationalConstitutionalContext) -> CanonicalContextRepresentation {
    CanonicalContextRepresentation::from_context(context)
}

pub fn validate_representation(
    representation: &CanonicalContextRepresentation,
) -> Result<(), CanonicalDecodeError> {
    if representation.format_id.as_str() != FORMAT_ID {
        return Err(CanonicalDecodeError::UnsupportedFormat(
            representation.format_id.to_string(),
        ));
    }
    if representation.representation_version.as_str() != FORMAT_VERSION {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            representation.representation_version.to_string(),
        ));
    }
    let mut seen = HashSet::new();
    let mut source_identities = std::collections::BTreeMap::new();
    for source in &representation.source_set {
        let key = source_key(source);
        if !seen.insert(key.clone()) {
            return Err(CanonicalDecodeError::DuplicateSourceReference);
        }
        if let Some((previous_version, previous_key)) = source_identities.insert(
            source.id.to_string(),
            (source.version.to_string(), key.clone()),
        ) {
            if previous_version != source.version.to_string() {
                return Err(CanonicalDecodeError::ConflictingSourceVersion(
                    source.id.to_string(),
                ));
            }
            if previous_key != key {
                return Err(CanonicalDecodeError::AmbiguousSourceReference(
                    source.id.to_string(),
                ));
            }
        }
    }
    let mut prior = None;
    for source in &representation.source_set {
        let key = source_key(source);
        if prior.as_ref().is_some_and(|value| value > &key) {
            return Err(CanonicalDecodeError::NonCanonicalOrdering);
        }
        prior = Some(key);
    }
    if let Some(source_set) = &representation.admitted_source_set {
        if source_set
            .sources
            .iter()
            .any(|source| source.admission_status.as_str() == "REJECTED")
        {
            return Err(CanonicalDecodeError::InvalidValue {
                field: "admitted_source_set".into(),
                detail: "rejected source is not representable in an admitted source set".into(),
            });
        }
        let mut prior = None;
        for source in &source_set.sources {
            let key = source_identity_key(&source.descriptor.identity);
            if prior.as_ref().is_some_and(|value| value > &key) {
                return Err(CanonicalDecodeError::NonCanonicalOrdering);
            }
            prior = Some(key);
        }
    }
    Ok(())
}

pub fn encode(context: &OperationalConstitutionalContext) -> Result<Vec<u8>, CanonicalDecodeError> {
    encode_representation(&canonicalize(context))
}

pub fn encode_representation(
    representation: &CanonicalContextRepresentation,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    validate_representation(representation)?;
    let mut fields: Vec<(String, String)> = vec![
        (
            "format_id".into(),
            representation.format_id.as_str().to_string(),
        ),
        (
            "representation_version".into(),
            representation.representation_version.as_str().to_string(),
        ),
        (
            "context_id".into(),
            optional(representation.context_id.as_ref().map(ToString::to_string)),
        ),
        (
            "admitted_source_set".into(),
            optional_admitted_source_set(representation.admitted_source_set.as_ref())?,
        ),
        (
            "operation_id".into(),
            representation.operation_id.to_string(),
        ),
        (
            "source_count".into(),
            representation.source_set.len().to_string(),
        ),
        ("baseline".into(), representation.baseline.to_string()),
        (
            "jurisdiction".into(),
            representation.jurisdiction.to_string(),
        ),
        ("scope".into(), representation.scope.to_string()),
        (
            "effective_context".into(),
            representation.effective_context.to_string(),
        ),
        (
            "boundary_context".into(),
            representation.boundary_context.to_string(),
        ),
        (
            "implementation_profile".into(),
            optional(
                representation
                    .implementation_profile
                    .as_ref()
                    .map(ToString::to_string),
            ),
        ),
        (
            "constitutional_effective_time".into(),
            representation
                .constitutional_effective_time
                .as_str()
                .to_string(),
        ),
        (
            "observation_time".into(),
            representation.observation_time.as_str().to_string(),
        ),
        (
            "processing_time".into(),
            representation.processing_time.as_str().to_string(),
        ),
        (
            "implementation_version".into(),
            representation.implementation_version.to_string(),
        ),
    ];
    for (index, source) in representation.source_set.iter().enumerate() {
        fields.push((format!("source.{index}.id"), source.id.to_string()));
        fields.push((
            format!("source.{index}.version"),
            source.version.to_string(),
        ));
        fields.push((format!("source.{index}.path"), source.path.clone()));
    }
    let mut output = String::new();
    for (key, value) in fields {
        output.push_str(&key);
        output.push('=');
        output.push_str(&escape(&value));
        output.push('\n');
    }
    Ok(output.into_bytes())
}

pub fn decode(bytes: &[u8]) -> Result<CanonicalContextRepresentation, CanonicalDecodeError> {
    if !bytes.ends_with(b"\n") {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let text = String::from_utf8(bytes.to_vec()).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    let mut fields = Vec::new();
    let mut seen = HashSet::new();
    let mut lines: Vec<_> = text.split('\n').collect();
    if lines.pop() != Some("") {
        return Err(CanonicalDecodeError::TrailingData);
    }
    for line in lines {
        if line.is_empty() {
            return Err(CanonicalDecodeError::TrailingData);
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(CanonicalDecodeError::MalformedLine(line.into()));
        };
        if !seen.insert(key.to_string()) {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        if !known_field(key) {
            return Err(CanonicalDecodeError::UnknownField(key.into()));
        }
        fields.push((
            key.to_string(),
            unescape(value).map_err(|detail| CanonicalDecodeError::InvalidValue {
                field: key.into(),
                detail,
            })?,
        ));
    }
    let get = |key: &str| {
        fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
            .ok_or_else(|| CanonicalDecodeError::MissingRequiredField(key.into()))
    };
    let format_id =
        RepresentationFormatId::new(get("format_id")?).map_err(contract("format_id"))?;
    let representation_version = RepresentationVersion::new(get("representation_version")?)
        .map_err(contract("representation_version"))?;
    let source_count: usize =
        get("source_count")?
            .parse()
            .map_err(|_| CanonicalDecodeError::InvalidValue {
                field: "source_count".into(),
                detail: "expected unsigned decimal".into(),
            })?;
    let mut source_set = Vec::new();
    for index in 0..source_count {
        let id = ConstitutionalSourceId::new(get(&format!("source.{index}.id"))?)
            .map_err(contract("source id"))?;
        let version = ConstitutionalSourceVersion::new(get(&format!("source.{index}.version"))?)
            .map_err(contract("source version"))?;
        let path = get(&format!("source.{index}.path"))?;
        source_set.push(
            ConstitutionalSourceRef::new(id, version, path).map_err(contract("source path"))?,
        );
    }
    let representation = CanonicalContextRepresentation {
        format_id,
        representation_version,
        context_id: optional_context(get("context_id")?, "context_id")?,
        admitted_source_set: decode_admitted_source_set(get("admitted_source_set")?)?,
        operation_id: OperationId::new(get("operation_id")?).map_err(contract("operation_id"))?,
        source_set,
        baseline: BaselineRef::new(get("baseline")?).map_err(contract("baseline"))?,
        jurisdiction: JurisdictionRef::new(get("jurisdiction")?)
            .map_err(contract("jurisdiction"))?,
        scope: ScopeRef::new(get("scope")?).map_err(contract("scope"))?,
        effective_context: EffectiveContextRef::new(get("effective_context")?)
            .map_err(contract("effective_context"))?,
        boundary_context: BoundaryContextRef::new(get("boundary_context")?)
            .map_err(contract("boundary_context"))?,
        implementation_profile: optional_profile(
            get("implementation_profile")?,
            "implementation_profile",
        )?,
        constitutional_effective_time: TimePoint::new(get("constitutional_effective_time")?)
            .map_err(contract("constitutional_effective_time"))?,
        observation_time: TimePoint::new(get("observation_time")?)
            .map_err(contract("observation_time"))?,
        processing_time: TimePoint::new(get("processing_time")?)
            .map_err(contract("processing_time"))?,
        implementation_version: ImplementationVersion::new(get("implementation_version")?)
            .map_err(contract("implementation_version"))?,
    };
    if fields.len() != expected_field_count(source_count) {
        return Err(CanonicalDecodeError::UnknownField(
            "unexpected or missing source field".into(),
        ));
    }
    validate_representation(&representation)?;
    if encode_representation(&representation)? != bytes {
        return Err(CanonicalDecodeError::NonCanonicalOrdering);
    }
    Ok(representation)
}

pub fn round_trip(context: &OperationalConstitutionalContext) -> Result<(), CanonicalDecodeError> {
    let bytes = encode(context)?;
    let decoded = decode(&bytes)?;
    if encode_representation(&decoded)? != bytes {
        return Err(CanonicalDecodeError::RoundTripMismatch);
    }
    Ok(())
}

/// Phase 1 compatibility wrapper: stable canonical bytes for the context slice.
pub fn canonical_context(context: &OperationalConstitutionalContext) -> Vec<u8> {
    encode(context).expect("context constructed by the resolver is encodable")
}

/// A small non-cryptographic digest for deterministic fixture comparison only.
pub fn non_authoritative_digest(bytes: &[u8]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

fn source_identity_key(identity: &SourceIdentity) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        identity.identifier,
        identity.family,
        identity.version,
        identity.root_id,
        identity.relative_path,
        identity.content_digest.value
    )
}

fn optional_admitted_source_set(
    source_set: Option<&AdmittedSourceSet>,
) -> Result<String, CanonicalDecodeError> {
    let Some(source_set) = source_set else {
        return Ok("absent".into());
    };
    let roots = source_set
        .root_ids
        .iter()
        .map(|root| escape(root.as_str()))
        .collect::<Vec<_>>()
        .join(",");
    let sources = source_set
        .sources
        .iter()
        .map(|source| {
            let identity = &source.descriptor.identity;
            [
                escape(identity.identifier.as_str()),
                escape(identity.family.as_str()),
                escape(identity.version.as_str()),
                escape(identity.root_id.as_str()),
                escape(&identity.relative_path),
                escape(&identity.content_digest.algorithm),
                escape(&identity.content_digest.value),
                escape(source.admission_status.as_str()),
            ]
            .join("~")
        })
        .collect::<Vec<_>>()
        .join(";");
    Ok(format!(
        "present:{}|{}|{}|{}",
        escape(source_set.source_set_id.as_str()),
        escape(source_set.admission_report_id.as_str()),
        roots,
        sources
    ))
}

fn decode_admitted_source_set(
    value: String,
) -> Result<Option<AdmittedSourceSet>, CanonicalDecodeError> {
    let Some(value) = value.strip_prefix("present:") else {
        return if value == "absent" {
            Ok(None)
        } else {
            Err(CanonicalDecodeError::InvalidValue {
                field: "admitted_source_set".into(),
                detail: "expected absent or present:value".into(),
            })
        };
    };
    let parts: Vec<_> = value.split('|').collect();
    if parts.len() != 4 {
        return Err(CanonicalDecodeError::InvalidValue {
            field: "admitted_source_set".into(),
            detail: "invalid admitted source-set field structure".into(),
        });
    }
    let source_set_id = SourceSetId::new(
        unescape(parts[0]).map_err(|detail| invalid_value("admitted_source_set", detail))?,
    )
    .map_err(contract("source_set_id"))?;
    let admission_report_id = AdmissionReportId::new(
        unescape(parts[1]).map_err(|detail| invalid_value("admitted_source_set", detail))?,
    )
    .map_err(contract("admission_report_id"))?;
    let root_ids = if parts[2].is_empty() {
        Vec::new()
    } else {
        parts[2]
            .split(',')
            .map(|value| {
                SourceRootId::new(
                    unescape(value)
                        .map_err(|detail| invalid_value("admitted_source_set", detail))?,
                )
                .map_err(contract("source_root_id"))
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    let mut sources = Vec::new();
    if !parts[3].is_empty() {
        for encoded in parts[3].split(';') {
            let fields: Vec<_> = encoded.split('~').collect();
            if fields.len() != 8 {
                return Err(CanonicalDecodeError::InvalidValue {
                    field: "admitted_source_set".into(),
                    detail: "invalid admitted source identity".into(),
                });
            }
            let value = |index: usize| {
                unescape(fields[index])
                    .map_err(|detail| invalid_value("admitted_source_set", detail))
            };
            let identifier =
                ConstitutionalSourceId::new(value(0)?).map_err(contract("source identifier"))?;
            let family = SourceFamily::new(value(1)?).map_err(contract("source family"))?;
            let version =
                ConstitutionalSourceVersion::new(value(2)?).map_err(contract("source version"))?;
            let root_id = SourceRootId::new(value(3)?).map_err(contract("source root"))?;
            let relative_path = value(4)?;
            let content_digest = SourceContentDigest {
                algorithm: value(5)?,
                value: value(6)?,
            };
            let admission_status =
                SourceAdmissionStatus::new(value(7)?).map_err(contract("admission status"))?;
            let source_ref = ConstitutionalSourceRef::new(
                identifier.clone(),
                version.clone(),
                format!("{}/{}", root_id, relative_path),
            )
            .map_err(contract("source reference"))?;
            let identity = SourceIdentity {
                identifier,
                family,
                version,
                root_id,
                relative_path,
                content_digest,
            };
            sources.push(AdmittedSource {
                descriptor: constitutional_contracts::SourceDescriptor {
                    source_ref,
                    identity,
                    title: None,
                    normative_status: None,
                    classification: None,
                    revision_type: None,
                    dependencies: Vec::new(),
                },
                admission_status,
            });
        }
    }
    Ok(Some(AdmittedSourceSet {
        source_set_id,
        admission_report_id,
        root_ids,
        sources,
    }))
}

fn invalid_value(field: &str, detail: String) -> CanonicalDecodeError {
    CanonicalDecodeError::InvalidValue {
        field: field.into(),
        detail,
    }
}

fn source_key(source: &ConstitutionalSourceRef) -> String {
    format!("{}\u{1f}{}\u{1f}{}", source.id, source.version, source.path)
}
fn optional(value: Option<String>) -> String {
    value
        .map(|value| format!("present:{value}"))
        .unwrap_or_else(|| "absent".into())
}
fn optional_context(value: String, field: &str) -> Result<Option<ContextId>, CanonicalDecodeError> {
    match value.strip_prefix("present:") {
        Some(value) => ContextId::new(value).map(Some).map_err(contract(field)),
        None if value == "absent" => Ok(None),
        None => Err(CanonicalDecodeError::InvalidValue {
            field: field.into(),
            detail: "expected absent or present:value".into(),
        }),
    }
}
fn optional_profile(
    value: String,
    field: &str,
) -> Result<Option<ImplementationProfileRef>, CanonicalDecodeError> {
    match value.strip_prefix("present:") {
        Some(value) => ImplementationProfileRef::new(value)
            .map(Some)
            .map_err(contract(field)),
        None if value == "absent" => Ok(None),
        None => Err(CanonicalDecodeError::InvalidValue {
            field: field.into(),
            detail: "expected absent or present:value".into(),
        }),
    }
}
fn expected_field_count(source_count: usize) -> usize {
    16 + source_count * 3
}
fn known_field(key: &str) -> bool {
    if matches!(
        key,
        "format_id"
            | "representation_version"
            | "context_id"
            | "admitted_source_set"
            | "operation_id"
            | "source_count"
            | "baseline"
            | "jurisdiction"
            | "scope"
            | "effective_context"
            | "boundary_context"
            | "implementation_profile"
            | "constitutional_effective_time"
            | "observation_time"
            | "processing_time"
            | "implementation_version"
    ) {
        return true;
    }
    let mut parts = key.split('.');
    matches!((parts.next(), parts.next(), parts.next(), parts.next()), (Some("source"), Some(index), Some("id" | "version" | "path"), None) if index.parse::<usize>().is_ok())
}
fn contract(field: &str) -> impl FnOnce(ContractError) -> CanonicalDecodeError {
    let field = field.to_string();
    move |_| CanonicalDecodeError::InvalidValue {
        field,
        detail: "invalid typed value".into(),
    }
}
fn escape(value: &str) -> String {
    value
        .as_bytes()
        .iter()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                (*byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}
fn unescape(value: &str) -> Result<String, String> {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err("truncated percent escape".into());
            }
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                .map_err(|_| "invalid percent escape")?;
            output.push(u8::from_str_radix(hex, 16).map_err(|_| "invalid percent escape")?);
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(output).map_err(|_| "decoded value is not UTF-8".into())
}

pub fn state_identity_encode(
    identity: &constitutional_state::StateIdentity,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-state-identity",
        vec![
            ("state_id", identity.id.to_string()),
            ("subject", identity.subject.to_string()),
            ("family", identity.family.to_string()),
            ("dimension", identity.dimension.to_string()),
            ("domain", identity.domain.to_string()),
            ("scope", identity.scope.to_string()),
            ("context_id", identity.context.to_string()),
            (
                "source_set_id",
                identity
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("version", identity.version.to_string()),
        ],
    )
}
pub fn transition_encode(
    transition: &constitutional_state::ProposedTransition,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-transition",
        vec![
            ("transition_id", transition.id.to_string()),
            ("subject", transition.subject.to_string()),
            ("source_state", transition.source_state.id.to_string()),
            ("target_state", transition.target_state.id.to_string()),
            ("kind", format!("{:?}", transition.kind)),
            (
                "effects",
                transition
                    .effects
                    .iter()
                    .map(|effect| format!("{:?}", effect))
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            (
                "authority",
                transition
                    .authority
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("context_id", transition.context.to_string()),
            (
                "source_set_id",
                transition
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version",
                transition.implementation_version.to_string(),
            ),
        ],
    )
}
pub fn state_evaluation_encode(
    result: &constitutional_state::StateEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-state-evaluation",
        vec![
            ("evaluation_id", result.evaluation_id.to_string()),
            ("operation_id", result.operation_id.to_string()),
            ("context_id", result.context_id.to_string()),
            (
                "source_set_id",
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version",
                result.implementation_version.to_string(),
            ),
            ("profile_version", result.profile_version.to_string()),
            (
                "state_determination",
                format!("{:?}", result.state_determination),
            ),
            ("admission", format!("{:?}", result.admission)),
            ("support", format!("{:?}", result.support)),
            ("preconditions", format!("{:?}", result.preconditions)),
            ("invariants", format!("{:?}", result.invariants)),
            ("projection", format!("{:?}", result.projection)),
            (
                "execution_performed",
                result.execution_performed.to_string(),
            ),
            ("mutation_performed", result.mutation_performed.to_string()),
            (
                "findings",
                result
                    .findings
                    .iter()
                    .map(|item| {
                        format!(
                            "{:?}:{}:{}:{}",
                            item.code, item.subject, item.detail, item.fatal
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            ("non_claims", result.non_claims.join(";")),
        ],
    )
}
pub fn state_decode(
    bytes: &[u8],
    expected_format: &str,
) -> Result<Vec<(String, String)>, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let mut fields = Vec::new();
    for line in text.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.first().map(|field| field.1.as_str()) != Some(expected_format) {
        return Err(CanonicalDecodeError::UnsupportedFormat(
            expected_format.into(),
        ));
    }
    if fields.get(1).map(|field| field.1.as_str()) != Some("1.0.0") {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields
                .get(1)
                .map(|field| field.1.clone())
                .unwrap_or_default(),
        ));
    }
    Ok(fields)
}
fn state_encode(
    format: &str,
    fields: Vec<(&str, String)>,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    let mut output = format!("format_id={format}\nrepresentation_version=1.0.0\n");
    for (key, value) in fields {
        output.push_str(key);
        output.push('=');
        output.push_str(&authority_escape(&value));
        output.push('\n');
    }
    Ok(output.into_bytes())
}

pub fn state_claim_encode(
    claim: &constitutional_state::StateClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-state-claim",
        vec![
            ("state_id", claim.identity.id.to_string()),
            ("basis", claim.basis.to_string()),
            ("category", format!("{:?}", claim.category)),
            (
                "source",
                format!(
                    "{}|{}|{}",
                    claim.source.id, claim.source.version, claim.source.path
                ),
            ),
            (
                "values",
                claim
                    .values
                    .iter()
                    .map(|value| format!("{}:{}", value.dimension, value.value))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ],
    )
}
pub fn transition_effect_encode(
    effect: &constitutional_state::TransitionEffect,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-transition-effect",
        vec![("effect", format!("{:?}", effect))],
    )
}
pub fn projected_state_encode(
    state: &constitutional_state::ProjectedState,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    state_encode(
        "macs-reference-canonical-projected-state",
        vec![
            ("state_id", state.identity.id.to_string()),
            (
                "values",
                state
                    .values
                    .iter()
                    .map(|value| format!("{}:{}", value.dimension, value.value))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            ("projected", state.projected.to_string()),
            ("execution_performed", state.execution_performed.to_string()),
        ],
    )
}

pub fn artifact_identity_encode(
    identity: &constitutional_artifacts::ArtifactIdentity,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-identity",
        vec![
            ("artifact_id", identity.id.to_string()),
            ("kind", format!("{:?}", identity.kind)),
            (
                "represented_subject",
                identity
                    .represented_subject
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("revision", identity.revision.to_string()),
            (
                "roles",
                identity
                    .roles
                    .iter()
                    .map(|role| format!("{:?}", role))
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ],
    )
}
pub fn artifact_instance_encode(
    instance: &constitutional_artifacts::ArtifactInstanceIdentity,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-instance",
        vec![
            ("instance_id", instance.id.to_string()),
            ("artifact_id", instance.artifact.to_string()),
            ("context", instance.context.to_string()),
            ("serialization", instance.serialization.to_string()),
        ],
    )
}
pub fn artifact_recognition_encode(
    basis: &constitutional_artifacts::RecognitionBasis,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-recognition",
        vec![
            (
                "source",
                format!(
                    "{}|{}|{}",
                    basis.source.id, basis.source.version, basis.source.path
                ),
            ),
            ("role", format!("{:?}", basis.role)),
            (
                "expressly_recognized",
                basis.expressly_recognized.to_string(),
            ),
            (
                "authority_reference",
                basis
                    .authority_reference
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "identity_reference",
                basis
                    .identity_reference
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("provenance", basis.provenance.to_string()),
        ],
    )
}
pub fn artifact_provenance_encode(
    provenance: &constitutional_artifacts::ArtifactProvenance,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-provenance",
        vec![
            (
                "origin",
                provenance
                    .origin
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "authority",
                provenance
                    .authority
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "identity",
                provenance
                    .identity
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "transformation",
                provenance
                    .transformation
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "version",
                provenance
                    .version
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "custody",
                provenance
                    .custody
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "publication",
                provenance
                    .publication
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "lineage",
                provenance
                    .lineage
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            ),
        ],
    )
}
pub fn artifact_relationship_encode(
    relationship: &constitutional_artifacts::ArtifactRelationship,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-relationship",
        vec![
            ("from", relationship.from.to_string()),
            ("kind", format!("{:?}", relationship.kind)),
            ("to", relationship.to.to_string()),
        ],
    )
}
pub fn artifact_integrity_encode(
    integrity: &constitutional_artifacts::ArtifactIntegrityClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-integrity",
        vec![
            (
                "representational",
                format!("{:?}", integrity.representational),
            ),
            ("identity", format!("{:?}", integrity.identity)),
            ("instance", format!("{:?}", integrity.instance)),
            ("lineage", format!("{:?}", integrity.lineage)),
            ("transformation", format!("{:?}", integrity.transformation)),
            ("version", format!("{:?}", integrity.version)),
            ("custody", format!("{:?}", integrity.custody)),
            ("publication", format!("{:?}", integrity.publication)),
        ],
    )
}
pub fn artifact_evaluation_encode(
    result: &constitutional_artifacts::ArtifactEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-artifact-evaluation",
        vec![
            ("evaluation_id", result.evaluation_id.to_string()),
            ("operation_id", result.operation_id.to_string()),
            ("context_id", result.context_id.to_string()),
            (
                "source_set_id",
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version",
                result.implementation_version.to_string(),
            ),
            ("profile_version", result.profile_version.to_string()),
            ("recognition", format!("{:?}", result.recognition)),
            ("instance", format!("{:?}", result.instance)),
            (
                "identity_preservation",
                format!("{:?}", result.identity_preservation),
            ),
            ("provenance", format!("{:?}", result.provenance)),
            ("integrity", format!("{:?}", result.integrity)),
            (
                "findings",
                result
                    .findings
                    .iter()
                    .map(|item| {
                        format!(
                            "{:?}:{}:{}:{}",
                            item.code, item.subject, item.detail, item.fatal
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(";"),
            ),
            ("non_claims", result.non_claims.join(";")),
        ],
    )
}
pub fn artifact_decode(
    bytes: &[u8],
    expected_format: &str,
) -> Result<Vec<(String, String)>, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let allowed: &[&str] = match expected_format {
        "macs-reference-canonical-artifact-identity" => &[
            "format_id",
            "representation_version",
            "artifact_id",
            "kind",
            "represented_subject",
            "revision",
            "roles",
        ],
        "macs-reference-canonical-artifact-instance" => &[
            "format_id",
            "representation_version",
            "instance_id",
            "artifact_id",
            "context",
            "serialization",
        ],
        "macs-reference-canonical-artifact-recognition" => &[
            "format_id",
            "representation_version",
            "source",
            "role",
            "expressly_recognized",
            "authority_reference",
            "identity_reference",
            "provenance",
        ],
        "macs-reference-canonical-artifact-provenance" => &[
            "format_id",
            "representation_version",
            "origin",
            "authority",
            "identity",
            "transformation",
            "version",
            "custody",
            "publication",
            "lineage",
        ],
        "macs-reference-canonical-artifact-relationship" => {
            &["format_id", "representation_version", "from", "kind", "to"]
        }
        "macs-reference-canonical-artifact-integrity" => &[
            "format_id",
            "representation_version",
            "representational",
            "identity",
            "instance",
            "lineage",
            "transformation",
            "version",
            "custody",
            "publication",
        ],
        "macs-reference-canonical-artifact-evaluation" => &[
            "format_id",
            "representation_version",
            "evaluation_id",
            "operation_id",
            "context_id",
            "source_set_id",
            "implementation_version",
            "profile_version",
            "recognition",
            "instance",
            "identity_preservation",
            "provenance",
            "integrity",
            "findings",
            "non_claims",
        ],
        _ => {
            return Err(CanonicalDecodeError::UnsupportedFormat(
                expected_format.into(),
            ));
        }
    };
    let mut fields = Vec::new();
    for line in text.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if !allowed.contains(&key) {
            return Err(CanonicalDecodeError::UnknownField(key.into()));
        }
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.first().map(|field| field.1.as_str()) != Some(expected_format) {
        return Err(CanonicalDecodeError::UnsupportedFormat(
            expected_format.into(),
        ));
    }
    if fields.get(1).map(|field| field.1.as_str()) != Some("1.0.0") {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields
                .get(1)
                .map(|field| field.1.clone())
                .unwrap_or_default(),
        ));
    }
    Ok(fields)
}
fn artifact_encode(
    format: &str,
    fields: Vec<(&str, String)>,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    let mut output = String::new();
    let has_header = fields
        .first()
        .map(|(key, _)| *key == "format_id")
        .unwrap_or(false);
    if !has_header {
        output.push_str("format_id=");
        output.push_str(format);
        output.push('\n');
        output.push_str("representation_version=1.0.0\n");
    }
    for (key, value) in fields {
        output.push_str(key);
        output.push('=');
        output.push_str(&authority_escape(&value));
        output.push('\n');
    }
    Ok(output.into_bytes())
}

pub const INTERACTION_FORMAT_VERSION: &str = "1.0.0";

fn interaction_fields(
    kind: &str,
    identity: &constitutional_interaction::InteractionIdentity,
    extra: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("format_id", kind.to_owned()),
        (
            "representation_version",
            INTERACTION_FORMAT_VERSION.to_owned(),
        ),
        ("interaction_id", identity.id.to_string()),
        ("subject", identity.subject.to_string()),
        ("interaction_kind", format!("{:?}", identity.kind)),
        ("purpose", identity.purpose.to_string()),
        ("scope", identity.scope.to_string()),
        ("context_id", identity.context.to_string()),
        (
            "source_set_id",
            identity
                .source_set_id
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
        ),
        ("implementation_version", identity.version.to_string()),
        ("extra", extra.to_owned()),
    ]
}

pub fn interaction_identity_encode(
    identity: &constitutional_interaction::InteractionIdentity,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-interaction-identity",
        interaction_fields(
            "macs-reference-canonical-interaction-identity",
            identity,
            "",
        ),
    )
}
pub fn interaction_claim_encode(
    claim: &constitutional_interaction::InteractionClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-interaction-claim",
        interaction_fields(
            "macs-reference-canonical-interaction-claim",
            &claim.identity,
            &format!(
                "basis={};source={};projected={};historical={}",
                claim.basis, claim.source.id, claim.projected, claim.historical
            ),
        ),
    )
}
pub fn constitutional_interaction_encode(
    claim: &constitutional_interaction::InteractionClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    interaction_claim_encode(claim)
}
pub fn boundary_identity_encode(
    boundary: &constitutional_interaction::ConstitutionalBoundary,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-boundary-identity",
        vec![
            (
                "format_id",
                "macs-reference-canonical-boundary-identity".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("boundary_id", boundary.identity.to_string()),
            ("boundary_kind", format!("{:?}", boundary.kind)),
            ("domain", boundary.domain.to_string()),
            ("scope", boundary.scope.to_string()),
            ("context_id", boundary.context.to_string()),
            ("source", boundary.source.id.to_string()),
            ("implementation_version", boundary.version.to_string()),
            (
                "extra",
                format!("{}>{}", boundary.source_side, boundary.destination_side),
            ),
        ],
    )
}
pub fn constitutional_boundary_encode(
    boundary: &constitutional_interaction::ConstitutionalBoundary,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    boundary_identity_encode(boundary)
}
pub fn proposed_boundary_crossing_encode(
    crossing: &constitutional_interaction::ProposedBoundaryCrossing,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-boundary-crossing",
        vec![
            (
                "format_id",
                "macs-reference-canonical-boundary-crossing".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("crossing_id", crossing.id.to_string()),
            ("interaction_id", crossing.interaction.to_string()),
            ("boundary_id", crossing.boundary.to_string()),
            ("source_side", crossing.source_side.to_string()),
            ("destination_side", crossing.destination_side.to_string()),
            ("initiator", crossing.initiator.to_string()),
            ("recipient", crossing.recipient.to_string()),
            ("scope", crossing.scope.to_string()),
            (
                "implementation_version",
                crossing.implementation_version.to_string(),
            ),
            (
                "extra",
                format!(
                    "{:?};{}",
                    crossing.kind,
                    crossing
                        .authority
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default()
                ),
            ),
        ],
    )
}
pub fn handoff_claim_encode(
    claim: &constitutional_interaction::HandoffClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-handoff-claim",
        vec![
            ("format_id", "macs-reference-canonical-handoff-claim".into()),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("handoff_id", claim.id.to_string()),
            ("interaction_id", claim.interaction.to_string()),
            ("from", claim.from.to_string()),
            ("to", claim.to.to_string()),
            ("subject", format!("{:?}", claim.subject)),
            ("scope", claim.scope.to_string()),
            ("extra", claim.authority_transfer_claimed.to_string()),
        ],
    )
}
pub fn response_claim_encode(
    claim: &constitutional_interaction::ResponseClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-response-claim",
        vec![
            (
                "format_id",
                "macs-reference-canonical-response-claim".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("response_id", claim.id.to_string()),
            ("interaction_id", claim.interaction.to_string()),
            ("kind", format!("{:?}", claim.kind)),
            ("responder", claim.responder.to_string()),
            (
                "basis",
                claim
                    .basis
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "condition",
                claim
                    .condition
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "extra",
                format!("{};{}", claim.received_claimed, claim.delivered_claimed),
            ),
        ],
    )
}
pub fn propagation_claim_encode(
    claim: &constitutional_interaction::PropagationClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-propagation-claim",
        vec![
            (
                "format_id",
                "macs-reference-canonical-propagation-claim".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("propagation_id", claim.id.to_string()),
            ("upstream", claim.upstream.to_string()),
            ("downstream", claim.downstream.to_string()),
            ("scope", claim.scope.to_string()),
            (
                "limit",
                claim
                    .limit
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("lineage", {
                let mut x = claim
                    .lineage
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                x.sort();
                x.join(",")
            }),
            ("extra", claim.prohibited.to_string()),
        ],
    )
}
pub fn containment_claim_encode(
    claim: &constitutional_interaction::ContainmentClaim,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-containment-claim",
        vec![
            (
                "format_id",
                "macs-reference-canonical-containment-claim".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("containment_id", claim.id.to_string()),
            ("container", claim.container.to_string()),
            ("components", {
                let mut x = claim
                    .components
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                x.sort();
                x.join(",")
            }),
            ("scope", claim.scope.to_string()),
        ],
    )
}
pub fn interaction_evaluation_encode(
    result: &constitutional_interaction::InteractionEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-interaction-evaluation",
        vec![
            (
                "format_id",
                "macs-reference-canonical-interaction-evaluation".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("evaluation_id", result.evaluation_id.to_string()),
            ("operation_id", result.operation_id.to_string()),
            ("context_id", result.context_id.to_string()),
            (
                "source_set_id",
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version",
                result.implementation_version.to_string(),
            ),
            (
                "determinations",
                format!(
                    "{:?}|{:?}|{:?}|{:?}|{:?}",
                    result.recognition,
                    result.admission,
                    result.support,
                    result.crossing,
                    result.projection
                ),
            ),
            (
                "flags",
                format!(
                    "{};{};{};{}",
                    result.execution_performed,
                    result.transmission_performed,
                    result.receipt_established,
                    result.effect_established
                ),
            ),
            ("findings", {
                let mut x = result
                    .findings
                    .iter()
                    .map(|f| format!("{:?}:{}", f.code, f.subject))
                    .collect::<Vec<_>>();
                x.sort();
                x.join(",")
            }),
            ("non_claims", result.non_claims.join(";")),
        ],
    )
}
pub fn interaction_admission_request_encode(
    req: &constitutional_interaction::InteractionEvaluationRequest,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    req.interaction
        .as_ref()
        .map(|x| {
            interaction_fields(
                "macs-reference-canonical-interaction-admission-request",
                &x.identity,
                &req.evaluation_id.to_string(),
            )
        })
        .map(|f| artifact_encode("macs-reference-canonical-interaction-admission-request", f))
        .unwrap_or_else(|| {
            Err(CanonicalDecodeError::MissingRequiredField(
                "interaction".into(),
            ))
        })
}
pub fn interaction_support_result_encode(
    result: &constitutional_interaction::InteractionEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    interaction_evaluation_encode(result)
}
pub fn boundary_crossing_evaluation_encode(
    result: &constitutional_interaction::InteractionEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    interaction_evaluation_encode(result)
}
pub fn interaction_result_projection_encode(
    result: &constitutional_interaction::ProjectedInteractionResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    artifact_encode(
        "macs-reference-canonical-interaction-projection",
        vec![
            (
                "format_id",
                "macs-reference-canonical-interaction-projection".into(),
            ),
            ("representation_version", INTERACTION_FORMAT_VERSION.into()),
            ("interaction_id", result.interaction.to_string()),
            (
                "receiving_scope",
                result
                    .receiving_scope
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            ("response_required", result.response_required.to_string()),
            ("projected", result.projected.to_string()),
            ("non_executed", result.non_executed.to_string()),
            ("non_transmitted", result.non_transmitted.to_string()),
            ("non_received", result.non_received.to_string()),
            ("non_accepted", result.non_accepted.to_string()),
            ("non_effective", result.non_effective.to_string()),
        ],
    )
}
pub fn interaction_decode(
    bytes: &[u8],
    expected_format: &str,
) -> Result<Vec<(String, String)>, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let expected: &[&str] = match expected_format {
        "macs-reference-canonical-interaction-identity"
        | "macs-reference-canonical-interaction-claim"
        | "macs-reference-canonical-interaction-admission-request" => &[
            "format_id",
            "representation_version",
            "interaction_id",
            "subject",
            "interaction_kind",
            "purpose",
            "scope",
            "context_id",
            "source_set_id",
            "implementation_version",
            "extra",
        ],
        "macs-reference-canonical-boundary-identity" => &[
            "format_id",
            "representation_version",
            "boundary_id",
            "boundary_kind",
            "domain",
            "scope",
            "context_id",
            "source",
            "implementation_version",
            "extra",
        ],
        "macs-reference-canonical-boundary-crossing" => &[
            "format_id",
            "representation_version",
            "crossing_id",
            "interaction_id",
            "boundary_id",
            "source_side",
            "destination_side",
            "initiator",
            "recipient",
            "scope",
            "implementation_version",
            "extra",
        ],
        "macs-reference-canonical-handoff-claim" => &[
            "format_id",
            "representation_version",
            "handoff_id",
            "interaction_id",
            "from",
            "to",
            "subject",
            "scope",
            "extra",
        ],
        "macs-reference-canonical-response-claim" => &[
            "format_id",
            "representation_version",
            "response_id",
            "interaction_id",
            "kind",
            "responder",
            "basis",
            "condition",
            "extra",
        ],
        "macs-reference-canonical-propagation-claim" => &[
            "format_id",
            "representation_version",
            "propagation_id",
            "upstream",
            "downstream",
            "scope",
            "limit",
            "lineage",
            "extra",
        ],
        "macs-reference-canonical-containment-claim" => &[
            "format_id",
            "representation_version",
            "containment_id",
            "container",
            "components",
            "scope",
        ],
        "macs-reference-canonical-interaction-evaluation" => &[
            "format_id",
            "representation_version",
            "evaluation_id",
            "operation_id",
            "context_id",
            "source_set_id",
            "implementation_version",
            "determinations",
            "flags",
            "findings",
            "non_claims",
        ],
        "macs-reference-canonical-interaction-projection" => &[
            "format_id",
            "representation_version",
            "interaction_id",
            "receiving_scope",
            "response_required",
            "projected",
            "non_executed",
            "non_transmitted",
            "non_received",
            "non_accepted",
            "non_effective",
        ],
        _ => {
            return Err(CanonicalDecodeError::UnsupportedFormat(
                expected_format.into(),
            ));
        }
    };
    let mut fields = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if index >= expected.len() || key != expected[index] {
            if !expected.contains(&key) {
                return Err(CanonicalDecodeError::UnknownField(key.into()));
            }
            return Err(CanonicalDecodeError::NonCanonicalOrdering);
        }
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.first().map(|x| x.1.as_str()) != Some(expected_format) {
        return Err(CanonicalDecodeError::UnsupportedFormat(
            expected_format.into(),
        ));
    }
    if fields.get(1).map(|x| x.1.as_str()) != Some(INTERACTION_FORMAT_VERSION) {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields.get(1).map(|x| x.1.clone()).unwrap_or_default(),
        ));
    }
    if fields.len() != expected.len() {
        return Err(CanonicalDecodeError::MissingRequiredField(
            "interaction representation field".into(),
        ));
    }
    Ok(fields)
}

pub fn core_evaluation_encode(
    result: &constitutional_core::CoreEvaluationResult,
) -> Result<Vec<u8>, CanonicalDecodeError> {
    let mut domains = result
        .domain_results
        .references
        .iter()
        .map(|r| format!("{:?}:{}", r.domain, r.result_id))
        .collect::<Vec<_>>();
    domains.sort();
    artifact_encode(
        "macs-reference-canonical-core-evaluation",
        vec![
            (
                "format_id",
                "macs-reference-canonical-core-evaluation".into(),
            ),
            ("representation_version", "1.0.0".into()),
            ("evaluation_id", result.evaluation_id.to_string()),
            ("operation_id", result.operation_id.to_string()),
            ("context_id", result.context_id.to_string()),
            (
                "source_set_id",
                result
                    .source_set_id
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            ),
            (
                "implementation_version",
                result.implementation_version.to_string(),
            ),
            ("integration", format!("{:?}", result.integration)),
            ("dependencies", format!("{:?}", result.dependencies)),
            ("invariants", format!("{:?}", result.invariants)),
            ("bindings", format!("{:?}", result.bindings)),
            ("domains", domains.join(",")),
            ("canonical_identity", result.canonical_identity.to_string()),
            (
                "flags",
                format!(
                    "{};{};{}",
                    result.execution_performed,
                    result.mutation_performed,
                    result.effect_established
                ),
            ),
            ("findings", {
                let mut x = result
                    .findings
                    .iter()
                    .map(|f| format!("{:?}:{}", f.code, f.subject))
                    .collect::<Vec<_>>();
                x.sort();
                x.join(",")
            }),
            ("evidence", {
                let mut x = result
                    .evidence
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>();
                x.sort();
                x.join(",")
            }),
            ("non_claims", result.non_claims.join(";")),
        ],
    )
}

pub fn core_evaluation_decode(bytes: &[u8]) -> Result<Vec<(String, String)>, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    let expected = [
        "format_id",
        "representation_version",
        "evaluation_id",
        "operation_id",
        "context_id",
        "source_set_id",
        "implementation_version",
        "integration",
        "dependencies",
        "invariants",
        "bindings",
        "domains",
        "canonical_identity",
        "flags",
        "findings",
        "evidence",
        "non_claims",
    ];
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let mut fields = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if index >= expected.len() || key != expected[index] {
            if !expected.contains(&key) {
                return Err(CanonicalDecodeError::UnknownField(key.into()));
            }
            return Err(CanonicalDecodeError::NonCanonicalOrdering);
        }
        if fields
            .iter()
            .any(|(known, _): &(String, String)| known == key)
        {
            return Err(CanonicalDecodeError::DuplicateField(key.into()));
        }
        fields.push((key.into(), authority_unescape(value)?));
    }
    if fields.len() != expected.len() {
        return Err(CanonicalDecodeError::MissingRequiredField(
            "core evaluation field".into(),
        ));
    }
    if fields[0].1 != "macs-reference-canonical-core-evaluation" {
        return Err(CanonicalDecodeError::UnsupportedFormat(fields[0].1.clone()));
    }
    if fields[1].1 != "1.0.0" {
        return Err(CanonicalDecodeError::UnsupportedVersion(
            fields[1].1.clone(),
        ));
    }
    Ok(fields)
}
