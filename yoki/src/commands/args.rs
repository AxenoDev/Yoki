use std::collections::HashMap;
use std::sync::Arc;

use minecraft_packet::packets::play::commands::ArgumentProperties;

use crate::ServerState;
use crate::commands::error::ArgError;
use crate::server::players::ConnectedPlayer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgKind {
    Word,
    Greedy,
    Int { min: Option<i32>, max: Option<i32> },
    Float { min: Option<f32>, max: Option<f32> },
    Bool,
    Player,
}

#[derive(Debug, Clone, Copy)]
pub struct ArgSpec {
    pub name: &'static str,
    pub kind: ArgKind,
    pub required: bool,
}

impl ArgSpec {
    pub const fn required(name: &'static str, kind: ArgKind) -> Self {
        Self {
            name,
            kind,
            required: true,
        }
    }

    pub const fn optional(name: &'static str, kind: ArgKind) -> Self {
        Self {
            name,
            kind,
            required: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArgValue {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Player(Arc<ConnectedPlayer>),
}

impl ArgKind {
    pub const fn parser_name(&self) -> &'static str {
        match self {
            Self::Word | Self::Greedy => "brigadier:string",
            Self::Int { .. } => "brigadier:integer",
            Self::Float { .. } => "brigadier:float",
            Self::Bool => "brigadier:bool",
            Self::Player => "minecraft:entity",
        }
    }

    pub const fn properties(&self) -> ArgumentProperties {
        match self {
            Self::Word => ArgumentProperties::StringWord,
            Self::Greedy => ArgumentProperties::StringGreedy,
            Self::Int { min, max } => ArgumentProperties::Int {
                min: *min,
                max: *max,
            },
            Self::Float { min, max } => ArgumentProperties::Float {
                min: *min,
                max: *max,
            },
            Self::Bool => ArgumentProperties::None,
            Self::Player => ArgumentProperties::SinglePlayer,
        }
    }

    fn parse(
        &self,
        name: &'static str,
        token: &str,
        server_state: &ServerState,
    ) -> Result<ArgValue, ArgError> {
        let invalid = |reason: String| ArgError::Invalid { name, reason };

        match self {
            Self::Word | Self::Greedy => Ok(ArgValue::Str(token.to_string())),
            Self::Int { min, max } => {
                let value: i32 = token
                    .parse()
                    .map_err(|_| invalid(format!("'{token}' is not an integer")))?;
                if let Some(min) = min
                    && value < *min
                {
                    return Err(invalid(format!("must be at least {min}")));
                }
                if let Some(max) = max
                    && value > *max
                {
                    return Err(invalid(format!("must be at most {max}")));
                }
                Ok(ArgValue::Int(value))
            }
            Self::Float { min, max } => {
                let value: f32 = token
                    .parse()
                    .map_err(|_| invalid(format!("'{token}' is not a number")))?;
                if let Some(min) = min
                    && value < *min
                {
                    return Err(invalid(format!("must be at least {min}")));
                }
                if let Some(max) = max
                    && value > *max
                {
                    return Err(invalid(format!("must be at most {max}")));
                }
                Ok(ArgValue::Float(value))
            }
            Self::Bool => match token {
                "true" => Ok(ArgValue::Bool(true)),
                "false" => Ok(ArgValue::Bool(false)),
                _ => Err(invalid(format!("'{token}' is not true or false"))),
            },
            Self::Player => server_state
                .players()
                .all()
                .into_iter()
                .find(|player| player.profile().username().eq_ignore_ascii_case(token))
                .map(ArgValue::Player)
                .ok_or_else(|| invalid(format!("player '{token}' is not online"))),
        }
    }
}

pub fn validate_specs(command_name: &str, specs: &[ArgSpec]) {
    let mut seen_optional = false;
    for (index, spec) in specs.iter().enumerate() {
        if matches!(spec.kind, ArgKind::Greedy) && index != specs.len() - 1 {
            panic!(
                "command '{command_name}': greedy argument <{}> must be last",
                spec.name
            );
        }
        if spec.required && seen_optional {
            panic!(
                "command '{command_name}': required argument <{}> after an optional one",
                spec.name
            );
        }
        seen_optional |= !spec.required;
    }
}

pub fn parse_args(
    specs: &[ArgSpec],
    input: &str,
    server_state: &ServerState,
) -> Result<HashMap<&'static str, ArgValue>, ArgError> {
    let mut values = HashMap::new();
    let mut rest = input;

    for spec in specs {
        if matches!(spec.kind, ArgKind::Greedy) {
            let remainder = rest.trim();
            rest = "";
            if remainder.is_empty() {
                if spec.required {
                    return Err(ArgError::Missing(spec.name));
                }
                continue;
            }
            values.insert(spec.name, ArgValue::Str(remainder.to_string()));
            continue;
        }

        let Some(token) = next_token(&mut rest) else {
            if spec.required {
                return Err(ArgError::Missing(spec.name));
            }
            continue;
        };
        values.insert(spec.name, spec.kind.parse(spec.name, &token, server_state)?);
    }

    if !rest.trim().is_empty() {
        return Err(ArgError::TooMany);
    }

    Ok(values)
}

fn next_token(rest: &mut &str) -> Option<String> {
    *rest = rest.trim_start();
    if rest.is_empty() {
        return None;
    }
    match rest.find(char::is_whitespace) {
        Some(index) => {
            let token = rest[..index].to_string();
            *rest = &rest[index..];
            Some(token)
        }
        None => {
            let token = rest.to_string();
            *rest = "";
            Some(token)
        }
    }
}
