//! Operation-scoped context resolution against a deliberately small fixture catalog.

use constitutional_contracts::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextResolutionRequest {
    pub operation_id: OperationId,
    pub required_sources: Vec<ConstitutionalSourceId>,
    pub baseline: Option<BaselineRef>,
    pub jurisdiction: Option<JurisdictionRef>,
    pub scope: Option<ScopeRef>,
    pub effective_context: Option<EffectiveContextRef>,
    pub boundary_context: Option<BoundaryContextRef>,
    pub constitutional_effective_time: Option<TimePoint>,
    pub observation_time: Option<TimePoint>,
    pub processing_time: Option<TimePoint>,
    pub implementation_version: Option<ImplementationVersion>,
}

#[derive(Clone, Debug, Default)]
pub struct SourceCatalog {
    sources: Vec<ConstitutionalSourceRef>,
}

impl SourceCatalog {
    pub fn new(sources: Vec<ConstitutionalSourceRef>) -> Self {
        Self { sources }
    }

    pub fn from_admitted_source_set(
        source_set: &AdmittedSourceSet,
    ) -> Result<Self, Vec<ValidationFinding>> {
        let mut findings = Vec::new();
        let mut sources = Vec::new();
        for source in &source_set.sources {
            match source.admission_status.as_str() {
                "ADMITTED" | "ADMITTED_WITH_FINDINGS" => {
                    sources.push(source.descriptor.source_ref.clone())
                }
                status => findings.push(ValidationFinding {
                    code: FindingCode::ConflictingInput,
                    severity: FindingSeverity::Error,
                    subject: source.descriptor.identity.relative_path.clone(),
                    detail: format!(
                        "source admission status {status} cannot enter context resolution"
                    ),
                }),
            }
        }
        if findings.is_empty() {
            Ok(Self::new(sources))
        } else {
            Err(findings)
        }
    }

    pub fn resolve_admitted(
        source_set: &AdmittedSourceSet,
        request: &ContextResolutionRequest,
    ) -> ContextResolutionResult {
        let catalog = match Self::from_admitted_source_set(source_set) {
            Ok(catalog) => catalog,
            Err(findings) => return ContextResolutionResult::Indeterminate { findings },
        };
        match catalog.resolve(request) {
            ContextResolutionResult::Resolved(mut context) => {
                context.admitted_source_set = Some(source_set.clone());
                ContextResolutionResult::Resolved(context)
            }
            other => other,
        }
    }

    pub fn resolve(&self, request: &ContextResolutionRequest) -> ContextResolutionResult {
        let mut findings = Vec::new();
        let mut resolved = Vec::new();
        for id in &request.required_sources {
            let matches: Vec<_> = self
                .sources
                .iter()
                .filter(|source| &source.id == id)
                .collect();
            match matches.as_slice() {
                [] => findings.push(ValidationFinding {
                    code: FindingCode::MissingSource,
                    severity: FindingSeverity::Error,
                    subject: id.to_string(),
                    detail: "required source is not present in the declared catalog".into(),
                }),
                [source] => resolved.push((*source).clone()),
                _ => findings.push(ValidationFinding {
                    code: FindingCode::AmbiguousSource,
                    severity: FindingSeverity::Error,
                    subject: id.to_string(),
                    detail:
                        "multiple candidate source references exist without governed precedence"
                            .into(),
                }),
            }
        }
        let baseline = required(&mut findings, "baseline", request.baseline.clone());
        let jurisdiction = match &request.jurisdiction {
            Some(value) => Some(value.clone()),
            None => {
                findings.push(ValidationFinding {
                    code: FindingCode::UnresolvedJurisdiction,
                    severity: FindingSeverity::Error,
                    subject: "jurisdiction".into(),
                    detail: "jurisdiction is absent or indeterminate".into(),
                });
                None
            }
        };
        let scope = match &request.scope {
            Some(value) => Some(value.clone()),
            None => {
                findings.push(ValidationFinding {
                    code: FindingCode::UnresolvedScope,
                    severity: FindingSeverity::Error,
                    subject: "scope".into(),
                    detail: "scope is absent or indeterminate".into(),
                });
                None
            }
        };
        let effective_context = required(
            &mut findings,
            "effective_context",
            request.effective_context.clone(),
        );
        let boundary_context = required(
            &mut findings,
            "boundary_context",
            request.boundary_context.clone(),
        );
        let effective_time = required(
            &mut findings,
            "constitutional_effective_time",
            request.constitutional_effective_time.clone(),
        );
        let observation_time = required(
            &mut findings,
            "observation_time",
            request.observation_time.clone(),
        );
        let processing_time = required(
            &mut findings,
            "processing_time",
            request.processing_time.clone(),
        );
        let implementation_version = required(
            &mut findings,
            "implementation_version",
            request.implementation_version.clone(),
        );
        if !findings.is_empty() {
            return ContextResolutionResult::Indeterminate { findings };
        }
        ContextResolutionResult::Resolved(Box::new(OperationalConstitutionalContext {
            operation_id: request.operation_id.clone(),
            source_set: resolved,
            baseline: baseline.expect("checked above"),
            jurisdiction: jurisdiction.expect("checked above"),
            scope: scope.expect("checked above"),
            effective_context: effective_context.expect("checked above"),
            boundary_context: boundary_context.expect("checked above"),
            constitutional_effective_time: effective_time.expect("checked above"),
            observation_time: observation_time.expect("checked above"),
            processing_time: processing_time.expect("checked above"),
            implementation_version: implementation_version.expect("checked above"),
            admitted_source_set: None,
        }))
    }
}

fn missing(name: &str) -> ValidationFinding {
    ValidationFinding {
        code: FindingCode::IncompleteCanonicalInput,
        severity: FindingSeverity::Error,
        subject: name.into(),
        detail: "required context element is missing".into(),
    }
}

fn required<T: Clone>(
    findings: &mut Vec<ValidationFinding>,
    name: &str,
    value: Option<T>,
) -> Option<T> {
    value.or_else(|| {
        findings.push(missing(name));
        None
    })
}
