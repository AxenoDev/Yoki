use core::fmt;
use std::str::FromStr;

use protocol_version::protocol_version::ProtocolVersion;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub fn known_versions() -> Vec<ProtocolVersion> {
    let mut versions: Vec<_> = ProtocolVersion::ALL_VERSION
        .iter()
        .copied()
        .filter(|version| version.protocol_number() > 0)
        .collect();
    versions.sort_by_key(|version| version.protocol_number());
    versions
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("invalid version '{input}': {reason}")]
pub struct VersionSpecError {
    input: String,
    reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bound {
    number: i32,
    inclusive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct VersionSpec {
    raw: String,
    min: Option<Bound>,
    max: Option<Bound>,
}

impl VersionSpec {
    pub fn at_least(version: ProtocolVersion) -> Self {
        Self {
            raw: format!("[{},)", version.humanize()),
            min: Some(Bound {
                number: version.protocol_number(),
                inclusive: true,
            }),
            max: None,
        }
    }

    pub fn matches(&self, version: ProtocolVersion) -> bool {
        let number = version.protocol_number();
        let above_min = self
            .min
            .is_none_or(|b| number > b.number || (b.inclusive && number == b.number));
        let below_max = self
            .max
            .is_none_or(|b| number < b.number || (b.inclusive && number == b.number));
        above_min && below_max
    }
}

fn parse_bound(text: &str, inclusive: bool) -> Result<Option<Bound>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }

    let known = known_versions();
    match known.iter().find(|version| version.humanize() == text) {
        Some(version) => Ok(Some(Bound {
            number: version.protocol_number(),
            inclusive,
        })),
        None => Err(format!(
            "unknown version '{text}' (known versions: {})",
            known
                .iter()
                .map(|version| version.humanize())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

impl FromStr for VersionSpec {
    type Err = VersionSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        let error = |reason: String| VersionSpecError {
            input: input.to_string(),
            reason,
        };

        if input.is_empty() {
            return Err(error("empty version".to_string()));
        }

        let (min, max) = if input.starts_with(['[', '(']) {
            let min_inclusive = input.starts_with('[');
            let max_inclusive = match input.chars().last() {
                Some(']') => true,
                Some(')') => false,
                _ => return Err(error("a range must be end with ']' or ')'".to_string())),
            };

            let inner = &input[1..input.len() - 1];
            match inner.split_once(',') {
                Some((low, high)) => (
                    parse_bound(low, min_inclusive).map_err(&error)?,
                    parse_bound(high, max_inclusive).map_err(&error)?,
                ),
                None if min_inclusive && max_inclusive => {
                    let bound = parse_bound(inner, true).map_err(&error)?;
                    if bound.is_none() {
                        return Err(error("empty version".to_string()));
                    }
                    (bound, bound)
                }
                None => {
                    return Err(error(
                        "expected 'min,max', for example [26.1,26.3)".to_string(),
                    ));
                }
            }
        } else {
            let bound = parse_bound(input, true).map_err(&error)?;
            (bound, bound)
        };

        if let (Some(min), Some(max)) = (min, max)
            && min.number > max.number
        {
            return Err(error("the minimum is greater than the maximum".to_string()));
        }

        let spec = Self {
            raw: input.to_string(),
            min,
            max,
        };
        if !known_versions().into_iter().any(|v| spec.matches(v)) {
            return Err(error(
                "this range doesn't match any known version".to_string(),
            ));
        }
        Ok(spec)
    }
}

impl fmt::Display for VersionSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

impl TryFrom<String> for VersionSpec {
    type Error = VersionSpecError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<VersionSpec> for String {
    fn from(spec: VersionSpec) -> Self {
        spec.raw
    }
}
