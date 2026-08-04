//! Shared IMP-005 canonical representation mechanics.
//!
//! This module owns representation structure and a bounded, deterministic UTF-8
//! line binding. It does not own the meaning or validity of any constitutional
//! domain object, and the binding is neither transport, persistence, admission,
//! execution, evidence, nor authority.

use std::fmt;

use crate::CanonicalDecodeError;

pub const REPRESENTATION_MODEL_ID: &str = "macs-reference-canonical-representation";
pub const REPRESENTATION_MODEL_VERSION: &str = "0.1.0";
pub const SERIALIZATION_BINDING_ID: &str = "macs-reference-canonical-lines";
pub const SERIALIZATION_BINDING_VERSION: &str = "1.0.0";

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalRepresentationId(String);

impl CanonicalRepresentationId {
    pub fn new(value: impl Into<String>) -> Result<Self, CanonicalModelError> {
        let value = value.into();
        if !valid_identifier(&value) {
            return Err(CanonicalModelError::MalformedIdentifier(value));
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CanonicalRepresentationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanonicalRepresentationKind {
    SourceReference,
    Context,
    AuthorityJurisdiction,
    IdentityParticipation,
    ArtifactProvenance,
    StateTransition,
    InteractionBoundary,
    CoreEvaluation,
    TraceabilityRecord,
    Finding,
    Determination,
    VerificationReference,
    EvidenceReference,
    ReleaseReference,
    ActivationReference,
    Extension(String),
}

impl CanonicalRepresentationKind {
    fn as_str(&self) -> String {
        match self {
            Self::SourceReference => "source-reference",
            Self::Context => "context",
            Self::AuthorityJurisdiction => "authority-jurisdiction",
            Self::IdentityParticipation => "identity-participation",
            Self::ArtifactProvenance => "artifact-provenance",
            Self::StateTransition => "state-transition",
            Self::InteractionBoundary => "interaction-boundary",
            Self::CoreEvaluation => "core-evaluation",
            Self::TraceabilityRecord => "traceability-record",
            Self::Finding => "finding",
            Self::Determination => "determination",
            Self::VerificationReference => "verification-reference",
            Self::EvidenceReference => "evidence-reference",
            Self::ReleaseReference => "release-reference",
            Self::ActivationReference => "activation-reference",
            Self::Extension(value) => return format!("extension:{value}"),
        }
        .into()
    }
    fn parse(value: &str) -> Result<Self, CanonicalDecodeError> {
        let result = match value {
            "source-reference" => Self::SourceReference,
            "context" => Self::Context,
            "authority-jurisdiction" => Self::AuthorityJurisdiction,
            "identity-participation" => Self::IdentityParticipation,
            "artifact-provenance" => Self::ArtifactProvenance,
            "state-transition" => Self::StateTransition,
            "interaction-boundary" => Self::InteractionBoundary,
            "core-evaluation" => Self::CoreEvaluation,
            "traceability-record" => Self::TraceabilityRecord,
            "finding" => Self::Finding,
            "determination" => Self::Determination,
            "verification-reference" => Self::VerificationReference,
            "evidence-reference" => Self::EvidenceReference,
            "release-reference" => Self::ReleaseReference,
            "activation-reference" => Self::ActivationReference,
            value if value.starts_with("extension:") && valid_identifier(&value[10..]) => {
                Self::Extension(value[10..].into())
            }
            _ => return Err(CanonicalDecodeError::UnsupportedFormat(value.into())),
        };
        Ok(result)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CanonicalReferenceKind {
    Source,
    Context,
    Requirement,
    Component,
    Artifact,
    State,
    Interaction,
    Trace,
    Finding,
    Verification,
    Evidence,
    Release,
    Activation,
    External,
}
impl CanonicalReferenceKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Context => "context",
            Self::Requirement => "requirement",
            Self::Component => "component",
            Self::Artifact => "artifact",
            Self::State => "state",
            Self::Interaction => "interaction",
            Self::Trace => "trace",
            Self::Finding => "finding",
            Self::Verification => "verification",
            Self::Evidence => "evidence",
            Self::Release => "release",
            Self::Activation => "activation",
            Self::External => "external",
        }
    }
    fn parse(value: &str) -> Result<Self, CanonicalDecodeError> {
        Ok(match value {
            "source" => Self::Source,
            "context" => Self::Context,
            "requirement" => Self::Requirement,
            "component" => Self::Component,
            "artifact" => Self::Artifact,
            "state" => Self::State,
            "interaction" => Self::Interaction,
            "trace" => Self::Trace,
            "finding" => Self::Finding,
            "verification" => Self::Verification,
            "evidence" => Self::Evidence,
            "release" => Self::Release,
            "activation" => Self::Activation,
            "external" => Self::External,
            _ => {
                return Err(CanonicalDecodeError::InvalidValue {
                    field: "reference".into(),
                    detail: "invalid reference kind".into(),
                });
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CanonicalReference {
    pub kind: CanonicalReferenceKind,
    pub identifier: String,
    pub version: Option<String>,
}
impl CanonicalReference {
    pub fn new(
        kind: CanonicalReferenceKind,
        identifier: impl Into<String>,
        version: Option<String>,
    ) -> Result<Self, CanonicalModelError> {
        let identifier = identifier.into();
        if !valid_identifier(&identifier) {
            return Err(CanonicalModelError::MalformedReference(identifier));
        }
        if version
            .as_deref()
            .is_some_and(|value| !valid_identifier(value))
        {
            return Err(CanonicalModelError::MalformedReference(
                version.unwrap_or_default(),
            ));
        }
        Ok(Self {
            kind,
            identifier,
            version,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalValue {
    Text(String),
    Boolean(bool),
    Unsigned(u64),
    Signed(i64),
    Null,
    Reference(CanonicalReference),
    Collection(CanonicalCollection),
    Object(Box<CanonicalObject>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalCollection {
    pub ordered: bool,
    pub items: Vec<CanonicalValue>,
}
impl CanonicalCollection {
    pub fn ordered(items: Vec<CanonicalValue>) -> Self {
        Self {
            ordered: true,
            items,
        }
    }
    pub fn unordered(mut items: Vec<CanonicalValue>) -> Self {
        items.sort_by_key(value_sort_key);
        Self {
            ordered: false,
            items,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalField {
    pub name: String,
    pub value: CanonicalValue,
    pub required: bool,
    pub identity_participates: bool,
    pub extension: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalObject {
    pub representation_id: CanonicalRepresentationId,
    pub kind: CanonicalRepresentationKind,
    pub representation_version: String,
    pub schema_id: String,
    pub source_binding: Option<constitutional_contracts::ConstitutionalSourceRef>,
    pub implementation_version: Option<constitutional_contracts::ImplementationVersion>,
    pub required_fields: Vec<String>,
    pub extension_namespaces: Vec<String>,
    pub fields: Vec<CanonicalField>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRecord {
    pub record_id: CanonicalRepresentationId,
    pub object: CanonicalObject,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum CanonicalValidationFindingCode {
    MissingRepresentationIdentifier,
    DuplicateRepresentationIdentifier,
    MissingRepresentationKind,
    UnsupportedRepresentationKind,
    MissingRepresentationVersion,
    UnsupportedRepresentationVersion,
    MalformedSourceBinding,
    MissingSourceVersion,
    MalformedImplementationVersionBinding,
    ImplementationVersionMismatch,
    DuplicateCanonicalField,
    UnknownCanonicalField,
    MissingRequiredCanonicalField,
    InvalidCanonicalFieldOrder,
    InvalidCollectionOrder,
    InvalidDiscriminator,
    IncompatibleSchemaProfile,
    InvalidCanonicalReference,
    UnresolvedMandatoryReference,
    MalformedCanonicalValue,
    NonCanonicalEncoding,
    RoundTripMismatch,
    IdentityParticipationMismatch,
    ProhibitedTransportOrPersistenceField,
    ProhibitedAuthorityBearingField,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalValidationFinding {
    pub code: CanonicalValidationFindingCode,
    pub subject: String,
    pub detail: String,
    pub fatal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalValidationResult {
    pub representation_id: Option<CanonicalRepresentationId>,
    pub valid: bool,
    pub findings: Vec<CanonicalValidationFinding>,
    pub authority_created: bool,
    pub source_admitted: bool,
    pub persisted: bool,
    pub transmitted: bool,
    pub execution_performed: bool,
    pub domain_validity_established: bool,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalEncodingResult {
    pub bytes: Vec<u8>,
    pub representation_id: CanonicalRepresentationId,
    pub identity_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalDecodingResult {
    pub object: CanonicalObject,
    pub validation: CanonicalValidationResult,
    pub identity_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepresentationCompatibility {
    ExactRepresentationVersionMatch,
    SupportedCompatibleVersion,
    UnsupportedVersion,
    IncompatibleRepresentationKind,
    IncompatibleSchemaProfile,
    Indeterminate,
    MalformedRepresentation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalModelError {
    MalformedIdentifier(String),
    MalformedReference(String),
}
impl fmt::Display for CanonicalModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn validate_canonical_object(object: &CanonicalObject) -> CanonicalValidationResult {
    let mut findings = Vec::new();
    if !valid_identifier(object.representation_version.as_str()) {
        findings.push(finding(
            CanonicalValidationFindingCode::MissingRepresentationVersion,
            "representation_version",
            "representation version is missing or malformed",
        ));
    } else if version_parts(&object.representation_version).is_none() {
        findings.push(finding(
            CanonicalValidationFindingCode::UnsupportedRepresentationVersion,
            "representation_version",
            "representation version is not a supported numeric version",
        ));
    }
    if !valid_identifier(&object.schema_id) {
        findings.push(finding(
            CanonicalValidationFindingCode::IncompatibleSchemaProfile,
            "schema_id",
            "schema/profile identifier is malformed",
        ));
    }
    if let Some(source) = &object.source_binding {
        if source.path.is_empty() || source.path.chars().any(char::is_control) {
            findings.push(finding(
                CanonicalValidationFindingCode::MalformedSourceBinding,
                "source_binding",
                "source path is malformed",
            ));
        }
        if source.version.as_str().is_empty() {
            findings.push(finding(
                CanonicalValidationFindingCode::MissingSourceVersion,
                "source_binding",
                "source version is missing",
            ));
        }
    }
    if let Some(version) = &object.implementation_version
        && !valid_identifier(version.as_str())
    {
        findings.push(finding(
            CanonicalValidationFindingCode::MalformedImplementationVersionBinding,
            "implementation_version",
            "implementation version is malformed",
        ));
    }
    let mut names = Vec::new();
    for field in &object.fields {
        if !valid_field_name(&field.name) {
            findings.push(finding(
                CanonicalValidationFindingCode::UnknownCanonicalField,
                field.name.clone(),
                "canonical field name is invalid",
            ));
        }
        if names.iter().any(|known: &String| known == &field.name) {
            findings.push(finding(
                CanonicalValidationFindingCode::DuplicateCanonicalField,
                field.name.clone(),
                "canonical field is duplicated",
            ));
        }
        names.push(field.name.clone());
        if field.extension
            && !object
                .extension_namespaces
                .iter()
                .any(|namespace| field.name.starts_with(namespace))
        {
            findings.push(finding(
                CanonicalValidationFindingCode::UnknownCanonicalField,
                field.name.clone(),
                "extension field is outside a declared namespace",
            ));
        }
        validate_value(&field.value, &field.name, &mut findings);
    }
    if names.windows(2).any(|pair| pair[0] >= pair[1]) {
        findings.push(finding(
            CanonicalValidationFindingCode::InvalidCanonicalFieldOrder,
            "fields",
            "fields must be strictly ordered by canonical name",
        ));
    }
    for required in &object.required_fields {
        if !object.fields.iter().any(|field| &field.name == required) {
            findings.push(finding(
                CanonicalValidationFindingCode::MissingRequiredCanonicalField,
                required.clone(),
                "required field is absent",
            ));
        }
    }
    if object
        .fields
        .iter()
        .any(|field| field.name.starts_with("transport.") || field.name.starts_with("persistence."))
    {
        findings.push(finding(
            CanonicalValidationFindingCode::ProhibitedTransportOrPersistenceField,
            "fields",
            "transport and persistence fields are outside IMP-005 representation scope",
        ));
    }
    if object
        .fields
        .iter()
        .any(|field| field.name.starts_with("authority.") && field.name != "authority.reference")
    {
        findings.push(finding(
            CanonicalValidationFindingCode::ProhibitedAuthorityBearingField,
            "fields",
            "representation cannot create authority-bearing fields",
        ));
    }
    findings.sort_by(|left, right| {
        left.subject
            .cmp(&right.subject)
            .then_with(|| left.code.cmp(&right.code))
    });
    let valid = findings.iter().all(|finding| !finding.fatal);
    CanonicalValidationResult { representation_id: Some(object.representation_id.clone()), valid, findings, authority_created: false, source_admitted: false, persisted: false, transmitted: false, execution_performed: false, domain_validity_established: false, non_claims: vec!["canonical representation is not constitutional validity, source admission, persistence, transport, execution, evidence, assurance, conformance, certification, release authorization, activation, or operational recognition".into()] }
}

pub fn canonical_validate_for_implementation(
    object: &CanonicalObject,
    expected: &constitutional_contracts::ImplementationVersion,
) -> CanonicalValidationResult {
    let mut result = validate_canonical_object(object);
    if object.implementation_version.as_ref() != Some(expected) {
        result.findings.push(finding(
            CanonicalValidationFindingCode::ImplementationVersionMismatch,
            "implementation_version",
            "representation implementation version does not match the expected version",
        ));
        result.findings.sort_by(|left, right| {
            left.subject
                .cmp(&right.subject)
                .then_with(|| left.code.cmp(&right.code))
        });
        result.valid = false;
    }
    result
}

pub fn canonical_encode(
    object: &CanonicalObject,
) -> Result<CanonicalEncodingResult, CanonicalDecodeError> {
    let validation = validate_canonical_object(object);
    if !validation.valid {
        return Err(CanonicalDecodeError::InvalidValue {
            field: "canonical_object".into(),
            detail: format!(
                "{} canonical validation finding(s)",
                validation.findings.len()
            ),
        });
    }
    let bytes = encode_object_unchecked(object)?;
    let identity_digest = canonical_identity_digest(object)?;
    Ok(CanonicalEncodingResult {
        bytes,
        representation_id: object.representation_id.clone(),
        identity_digest,
    })
}

pub fn canonical_decode(bytes: &[u8]) -> Result<CanonicalDecodingResult, CanonicalDecodeError> {
    let object = decode_object(bytes)?;
    let validation = validate_canonical_object(&object);
    if !validation.valid {
        return Err(CanonicalDecodeError::InvalidValue {
            field: "canonical_object".into(),
            detail: "decoded object failed canonical validation".into(),
        });
    }
    let identity_digest = canonical_identity_digest(&object)?;
    Ok(CanonicalDecodingResult {
        object,
        validation,
        identity_digest,
    })
}

pub fn canonical_validate_bytes(bytes: &[u8]) -> CanonicalValidationResult {
    if !bytes.starts_with(b"representation_id=") {
        return CanonicalValidationResult {
            representation_id: None,
            valid: false,
            findings: vec![finding(
                CanonicalValidationFindingCode::MissingRepresentationIdentifier,
                "representation_id",
                "representation identifier is missing or is not the first canonical field",
            )],
            authority_created: false,
            source_admitted: false,
            persisted: false,
            transmitted: false,
            execution_performed: false,
            domain_validity_established: false,
            non_claims: vec!["serialized validation does not create identity, authority, validity, admission, persistence, transport, execution, or recognition".into()],
        };
    }
    match canonical_decode(bytes) {
        Ok(result) => result.validation,
        Err(error) => {
            let code = match &error {
                CanonicalDecodeError::MissingRequiredField(field)
                    if field == "representation_id" =>
                {
                    CanonicalValidationFindingCode::MissingRepresentationIdentifier
                }
                CanonicalDecodeError::MissingRequiredField(field) if field == "kind" => {
                    CanonicalValidationFindingCode::MissingRepresentationKind
                }
                CanonicalDecodeError::MissingRequiredField(field)
                    if field == "representation_version" =>
                {
                    CanonicalValidationFindingCode::MissingRepresentationVersion
                }
                CanonicalDecodeError::MissingRequiredField(_field) => {
                    CanonicalValidationFindingCode::MissingRequiredCanonicalField
                }
                CanonicalDecodeError::DuplicateField(_) => {
                    CanonicalValidationFindingCode::DuplicateCanonicalField
                }
                CanonicalDecodeError::UnknownField(_) => {
                    CanonicalValidationFindingCode::UnknownCanonicalField
                }
                CanonicalDecodeError::UnsupportedFormat(_) => {
                    CanonicalValidationFindingCode::UnsupportedRepresentationKind
                }
                CanonicalDecodeError::UnsupportedVersion(_) => {
                    CanonicalValidationFindingCode::UnsupportedRepresentationVersion
                }
                CanonicalDecodeError::NonCanonicalOrdering => {
                    CanonicalValidationFindingCode::NonCanonicalEncoding
                }
                CanonicalDecodeError::InvalidValue { field, .. } if field == "source_binding" => {
                    CanonicalValidationFindingCode::MalformedSourceBinding
                }
                CanonicalDecodeError::InvalidValue { field, .. }
                    if field == "implementation_version" =>
                {
                    CanonicalValidationFindingCode::MalformedImplementationVersionBinding
                }
                CanonicalDecodeError::InvalidValue { field, .. } if field == "value" => {
                    CanonicalValidationFindingCode::MalformedCanonicalValue
                }
                CanonicalDecodeError::TrailingData => {
                    CanonicalValidationFindingCode::NonCanonicalEncoding
                }
                _ => CanonicalValidationFindingCode::MalformedCanonicalValue,
            };
            CanonicalValidationResult { representation_id: None, valid: false, findings: vec![finding(code, "serialized_representation", error.to_string())], authority_created: false, source_admitted: false, persisted: false, transmitted: false, execution_performed: false, domain_validity_established: false, non_claims: vec!["serialized validation does not admit, validate, execute, persist, transmit, authorize, verify, certify, activate, or recognize the represented object".into()] }
        }
    }
}

pub fn canonical_compatibility(
    left: &CanonicalObject,
    right: &CanonicalObject,
) -> RepresentationCompatibility {
    if !validate_canonical_object(left).valid || !validate_canonical_object(right).valid {
        return RepresentationCompatibility::MalformedRepresentation;
    }
    if left.kind != right.kind {
        return RepresentationCompatibility::IncompatibleRepresentationKind;
    }
    if left.schema_id != right.schema_id {
        return RepresentationCompatibility::IncompatibleSchemaProfile;
    }
    if left.representation_version == right.representation_version {
        return RepresentationCompatibility::ExactRepresentationVersionMatch;
    }
    match (
        version_parts(&left.representation_version),
        version_parts(&right.representation_version),
    ) {
        (Some((left_major, left_minor)), Some((right_major, right_minor)))
            if left_major == right_major && left_minor <= right_minor =>
        {
            RepresentationCompatibility::SupportedCompatibleVersion
        }
        (Some(_), Some(_)) => RepresentationCompatibility::UnsupportedVersion,
        _ => RepresentationCompatibility::Indeterminate,
    }
}

pub fn canonical_identity_digest(object: &CanonicalObject) -> Result<String, CanonicalDecodeError> {
    let mut identity = object.clone();
    identity.fields.retain(|field| field.identity_participates);
    let bytes = encode_object_unchecked(&identity)?;
    // This is the existing deterministic FNV-1a fixture digest, not a cryptographic claim.
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

fn encode_object_unchecked(object: &CanonicalObject) -> Result<Vec<u8>, CanonicalDecodeError> {
    let mut fields = vec![
        (
            "representation_id".into(),
            escape(object.representation_id.as_str()),
        ),
        ("kind".into(), escape(&object.kind.as_str())),
        (
            "representation_version".into(),
            escape(&object.representation_version),
        ),
        ("schema_id".into(), escape(&object.schema_id)),
        (
            "source_binding".into(),
            encode_source_binding(object.source_binding.as_ref())?,
        ),
        (
            "implementation_version".into(),
            optional_string(object.implementation_version.as_ref().map(|v| v.as_str())),
        ),
        (
            "required_fields".into(),
            encode_strings(&object.required_fields),
        ),
        (
            "extension_namespaces".into(),
            encode_strings(&object.extension_namespaces),
        ),
        ("field_count".into(), object.fields.len().to_string()),
    ];
    for (index, field) in object.fields.iter().enumerate() {
        fields.push((format!("field.{index}.name"), escape(&field.name)));
        fields.push((
            format!("field.{index}.required"),
            field.required.to_string(),
        ));
        fields.push((
            format!("field.{index}.identity"),
            field.identity_participates.to_string(),
        ));
        fields.push((
            format!("field.{index}.extension"),
            field.extension.to_string(),
        ));
        fields.push((format!("field.{index}.value"), encode_value(&field.value)?));
    }
    let mut output = String::new();
    for (key, value) in fields {
        output.push_str(&key);
        output.push('=');
        output.push_str(&value);
        output.push('\n');
    }
    Ok(output.into_bytes())
}

fn decode_object(bytes: &[u8]) -> Result<CanonicalObject, CanonicalDecodeError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CanonicalDecodeError::InvalidUtf8)?;
    if text.is_empty() || !text.ends_with('\n') {
        return Err(CanonicalDecodeError::TrailingData);
    }
    let lines: Vec<&str> = text.strip_suffix('\n').unwrap().split('\n').collect();
    let mut cursor = 0usize;
    let mut next_raw = |expected: &str| -> Result<String, CanonicalDecodeError> {
        let line = *lines
            .get(cursor)
            .ok_or_else(|| CanonicalDecodeError::MissingRequiredField(expected.into()))?;
        cursor += 1;
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| CanonicalDecodeError::MalformedLine(line.into()))?;
        if key != expected {
            if key.starts_with("field.")
                || [
                    "representation_id",
                    "kind",
                    "representation_version",
                    "schema_id",
                    "source_binding",
                    "implementation_version",
                    "required_fields",
                    "extension_namespaces",
                    "field_count",
                ]
                .contains(&key)
            {
                return Err(CanonicalDecodeError::NonCanonicalOrdering);
            }
            return Err(CanonicalDecodeError::UnknownField(key.into()));
        }
        Ok(value.into())
    };
    let mut next = |expected: &str| -> Result<String, CanonicalDecodeError> {
        let value = next_raw(expected)?;
        unescape(&value).map_err(|detail| CanonicalDecodeError::InvalidValue {
            field: expected.into(),
            detail,
        })
    };
    let representation_id =
        CanonicalRepresentationId::new(next("representation_id")?).map_err(|error| {
            CanonicalDecodeError::InvalidValue {
                field: "representation_id".into(),
                detail: error.to_string(),
            }
        })?;
    let kind = CanonicalRepresentationKind::parse(&next("kind")?)?;
    let representation_version = next("representation_version")?;
    let schema_id = next("schema_id")?;
    #[allow(clippy::drop_non_drop)]
    drop(next);
    let source_binding = decode_source_binding(&next_raw("source_binding")?)?;
    let mut next = |expected: &str| -> Result<String, CanonicalDecodeError> {
        let value = next_raw(expected)?;
        unescape(&value).map_err(|detail| CanonicalDecodeError::InvalidValue {
            field: expected.into(),
            detail,
        })
    };
    let implementation_version = match next("implementation_version")?.as_str() {
        "absent" => None,
        value => Some(
            constitutional_contracts::ImplementationVersion::new(value).map_err(|_| {
                CanonicalDecodeError::InvalidValue {
                    field: "implementation_version".into(),
                    detail: "malformed implementation version".into(),
                }
            })?,
        ),
    };
    let required_fields = decode_strings(&next("required_fields")?)?;
    let extension_namespaces = decode_strings(&next("extension_namespaces")?)?;
    let field_count: usize =
        next("field_count")?
            .parse()
            .map_err(|_| CanonicalDecodeError::InvalidValue {
                field: "field_count".into(),
                detail: "expected unsigned decimal".into(),
            })?;
    let mut fields = Vec::new();
    for index in 0..field_count {
        let name = next(&format!("field.{index}.name"))?;
        let required = parse_bool(&next(&format!("field.{index}.required"))?, "required")?;
        let identity_participates =
            parse_bool(&next(&format!("field.{index}.identity"))?, "identity")?;
        let extension = parse_bool(&next(&format!("field.{index}.extension"))?, "extension")?;
        let value = decode_value(&next(&format!("field.{index}.value"))?)?;
        fields.push(CanonicalField {
            name,
            value,
            required,
            identity_participates,
            extension,
        });
    }
    if cursor != lines.len() {
        return Err(CanonicalDecodeError::TrailingData);
    }
    Ok(CanonicalObject {
        representation_id,
        kind,
        representation_version,
        schema_id,
        source_binding,
        implementation_version,
        required_fields,
        extension_namespaces,
        fields,
    })
}

fn validate_value(
    value: &CanonicalValue,
    subject: &str,
    findings: &mut Vec<CanonicalValidationFinding>,
) {
    match value {
        CanonicalValue::Reference(reference) => {
            if !valid_identifier(&reference.identifier) {
                findings.push(finding(
                    CanonicalValidationFindingCode::InvalidCanonicalReference,
                    subject,
                    "reference identifier is malformed",
                ));
            }
        }
        CanonicalValue::Collection(collection) => {
            if !collection.ordered
                && collection
                    .items
                    .windows(2)
                    .any(|pair| value_sort_key(&pair[0]) >= value_sort_key(&pair[1]))
            {
                findings.push(finding(
                    CanonicalValidationFindingCode::InvalidCollectionOrder,
                    subject,
                    "unordered collection is not in canonical order",
                ));
            }
            for item in &collection.items {
                validate_value(item, subject, findings);
            }
        }
        CanonicalValue::Object(object) => {
            let nested = validate_canonical_object(object);
            findings.extend(nested.findings);
        }
        _ => {}
    }
}
fn finding(
    code: CanonicalValidationFindingCode,
    subject: impl Into<String>,
    detail: impl Into<String>,
) -> CanonicalValidationFinding {
    CanonicalValidationFinding {
        code,
        subject: subject.into(),
        detail: detail.into(),
        fatal: true,
    }
}
fn valid_identifier(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|c| !c.is_whitespace() && !c.is_control())
}
fn valid_field_name(value: &str) -> bool {
    valid_identifier(value) && !value.contains('=')
}
fn version_parts(value: &str) -> Option<(u64, u64)> {
    let mut parts = value.split('.');
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}
fn value_sort_key(value: &CanonicalValue) -> String {
    encode_value(value).unwrap_or_default()
}
fn optional_string(value: Option<&str>) -> String {
    value.map(escape).unwrap_or_else(|| "absent".into())
}
fn encode_strings(values: &[String]) -> String {
    values
        .iter()
        .map(|value| escape(value))
        .collect::<Vec<_>>()
        .join(",")
}
fn decode_strings(value: &str) -> Result<Vec<String>, CanonicalDecodeError> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    value
        .split(',')
        .map(|item| {
            unescape(item).map_err(|detail| CanonicalDecodeError::InvalidValue {
                field: "strings".into(),
                detail,
            })
        })
        .collect()
}
fn parse_bool(value: &str, field: &str) -> Result<bool, CanonicalDecodeError> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(CanonicalDecodeError::InvalidValue {
            field: field.into(),
            detail: "expected true or false".into(),
        }),
    }
}
fn encode_source_binding(
    value: Option<&constitutional_contracts::ConstitutionalSourceRef>,
) -> Result<String, CanonicalDecodeError> {
    Ok(match value {
        None => "absent".into(),
        Some(source) => [
            escape(source.id.as_str()),
            escape(source.version.as_str()),
            escape(&source.path),
        ]
        .join(","),
    })
}
fn decode_source_binding(
    value: &str,
) -> Result<Option<constitutional_contracts::ConstitutionalSourceRef>, CanonicalDecodeError> {
    if value == "absent" {
        return Ok(None);
    }
    let parts: Vec<_> = value.split(',').collect();
    if parts.len() != 3 {
        return Err(CanonicalDecodeError::InvalidValue {
            field: "source_binding".into(),
            detail: "expected source id, version, and path".into(),
        });
    }
    let id = constitutional_contracts::ConstitutionalSourceId::new(unescape(parts[0]).map_err(
        |detail| CanonicalDecodeError::InvalidValue {
            field: "source_binding".into(),
            detail,
        },
    )?)
    .map_err(|_| CanonicalDecodeError::InvalidValue {
        field: "source_binding".into(),
        detail: "malformed source id".into(),
    })?;
    let version = constitutional_contracts::ConstitutionalSourceVersion::new(
        unescape(parts[1]).map_err(|detail| CanonicalDecodeError::InvalidValue {
            field: "source_binding".into(),
            detail,
        })?,
    )
    .map_err(|_| CanonicalDecodeError::InvalidValue {
        field: "source_binding".into(),
        detail: "malformed source version".into(),
    })?;
    let path = unescape(parts[2]).map_err(|detail| CanonicalDecodeError::InvalidValue {
        field: "source_binding".into(),
        detail,
    })?;
    Ok(Some(
        constitutional_contracts::ConstitutionalSourceRef::new(id, version, path).map_err(
            |_| CanonicalDecodeError::InvalidValue {
                field: "source_binding".into(),
                detail: "malformed source reference".into(),
            },
        )?,
    ))
}
fn encode_value(value: &CanonicalValue) -> Result<String, CanonicalDecodeError> {
    Ok(match value {
        CanonicalValue::Text(value) => format!("text:{}", escape(value)),
        CanonicalValue::Boolean(value) => format!("boolean:{value}"),
        CanonicalValue::Unsigned(value) => format!("unsigned:{value}"),
        CanonicalValue::Signed(value) => format!("signed:{value}"),
        CanonicalValue::Null => "null".into(),
        CanonicalValue::Reference(reference) => format!(
            "reference:{},{},{}",
            reference.kind.as_str(),
            escape(&reference.identifier),
            optional_string(reference.version.as_deref())
        ),
        CanonicalValue::Collection(collection) => format!(
            "collection:{}:{}",
            collection.ordered,
            collection
                .items
                .iter()
                .map(encode_value)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|item| escape(item.as_str()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        CanonicalValue::Object(object) => format!(
            "object:{}",
            escape(
                std::str::from_utf8(&encode_object_unchecked(object)?)
                    .map_err(|_| CanonicalDecodeError::InvalidUtf8)?
            )
        ),
    })
}
fn decode_value(value: &str) -> Result<CanonicalValue, CanonicalDecodeError> {
    if value == "null" {
        return Ok(CanonicalValue::Null);
    }
    let (kind, payload) =
        value
            .split_once(':')
            .ok_or_else(|| CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail: "missing value discriminator".into(),
            })?;
    Ok(match kind {
        "text" => CanonicalValue::Text(unescape(payload).map_err(|detail| {
            CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail,
            }
        })?),
        "boolean" => CanonicalValue::Boolean(parse_bool(payload, "value")?),
        "unsigned" => CanonicalValue::Unsigned(payload.parse().map_err(|_| {
            CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail: "malformed unsigned value".into(),
            }
        })?),
        "signed" => CanonicalValue::Signed(payload.parse().map_err(|_| {
            CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail: "malformed signed value".into(),
            }
        })?),
        "reference" => {
            let parts: Vec<_> = payload.split(',').collect();
            if parts.len() != 3 {
                return Err(CanonicalDecodeError::InvalidValue {
                    field: "value".into(),
                    detail: "malformed reference".into(),
                });
            }
            CanonicalValue::Reference(
                CanonicalReference::new(
                    CanonicalReferenceKind::parse(parts[0])?,
                    unescape(parts[1]).map_err(|detail| CanonicalDecodeError::InvalidValue {
                        field: "value".into(),
                        detail,
                    })?,
                    if parts[2] == "absent" {
                        None
                    } else {
                        Some(unescape(parts[2]).map_err(|detail| {
                            CanonicalDecodeError::InvalidValue {
                                field: "value".into(),
                                detail,
                            }
                        })?)
                    },
                )
                .map_err(|_| CanonicalDecodeError::InvalidValue {
                    field: "value".into(),
                    detail: "malformed reference".into(),
                })?,
            )
        }
        "collection" => {
            let (ordered, items) =
                payload
                    .split_once(':')
                    .ok_or_else(|| CanonicalDecodeError::InvalidValue {
                        field: "value".into(),
                        detail: "malformed collection".into(),
                    })?;
            let ordered = parse_bool(ordered, "collection")?;
            let items = if items.is_empty() {
                Vec::new()
            } else {
                items
                    .split(',')
                    .map(|item| {
                        decode_value(&unescape(item).map_err(|detail| {
                            CanonicalDecodeError::InvalidValue {
                                field: "value".into(),
                                detail,
                            }
                        })?)
                    })
                    .collect::<Result<Vec<_>, _>>()?
            };
            CanonicalValue::Collection(CanonicalCollection { ordered, items })
        }
        "object" => {
            let inner = unescape(payload).map_err(|detail| CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail,
            })?;
            CanonicalValue::Object(Box::new(decode_object(inner.as_bytes())?))
        }
        _ => {
            return Err(CanonicalDecodeError::InvalidValue {
                field: "value".into(),
                detail: "unknown canonical value discriminator".into(),
            });
        }
    })
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
