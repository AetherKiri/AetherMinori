use std::collections::HashMap;

use crate::command::CommandKind;

/// Which localisation of a script to execute.
///
/// ef - the first tale ships a single script set in which individual lines are
/// prefixed with `[e]` or `[j]`. The two variants differ in pacing (transition
/// and wait durations) rather than in dialogue, and each variant is a complete,
/// self-contained program: every `.label`, `.goto` and `.chain` target resolves
/// inside its own variant. Lines belonging to the other variant are skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Japanese,
    English,
}

impl Language {
    pub fn from_code(code: &str) -> Option<Self> {
        Some(match code.to_ascii_lowercase().as_str() {
            "j" | "ja" | "jp" => Self::Japanese,
            "e" | "en" => Self::English,
            _ => return None,
        })
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::Japanese => "j",
            Self::English => "e",
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Empty,
    Comment,
    Control {
        name: String,
        arguments: Vec<String>,
    },
    Label(String),
    Include(String),
    Command {
        kind: CommandKind,
        arguments: Vec<String>,
    },
    Message(String),
}

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub labels: HashMap<String, usize>,
}

impl Program {
    pub fn parse(source: &str) -> Result<Self, String> {
        Self::parse_with_language(source, Language::default())
    }

    pub fn parse_with_language(source: &str, language: Language) -> Result<Self, String> {
        let mut program = Program::default();
        for (line_number, line) in source.lines().enumerate() {
            let Some(line) = select_line(line, language) else {
                continue;
            };
            let statement =
                parse_line(line).map_err(|e| format!("line {}: {e}", line_number + 1))?;
            if let Statement::Label(label) = &statement
                && program
                    .labels
                    .insert(label.clone(), program.statements.len())
                    .is_some()
            {
                return Err(format!("line {}: duplicate label {label}", line_number + 1));
            }
            program.statements.push(statement);
        }
        Ok(program)
    }
}

/// Strip the optional `[x]` localisation prefix. Returns `None` when the line
/// belongs to a different variant, which is how the other language's pass is
/// skipped. A line without a prefix belongs to every variant.
fn select_line(line: &str, language: Language) -> Option<&str> {
    let trimmed = line.trim();
    let Some(rest) = trimmed.strip_prefix('[') else {
        return Some(trimmed);
    };
    let Some((code, body)) = rest.split_once(']') else {
        return Some(trimmed);
    };
    if code.len() != 1 {
        return Some(trimmed);
    }
    match Language::from_code(code) {
        // 62 lines in the corpus write the variant prefix with a colon
        // (`[j]:.wait 200`), always on the Japanese pacing pass. The colon is a
        // separator, not part of the command: left in place the line parsed as a
        // narration message and `:wait 200` appeared on screen as dialogue.
        Some(found) if found == language => Some(body.trim_start().trim_start_matches(':')),
        Some(_) => None,
        None => Some(trimmed),
    }
}

pub fn parse_line(line: &str) -> Result<Statement, String> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(Statement::Empty);
    }
    if let Some(rest) = line.strip_prefix('@') {
        let mut fields = split_arguments(rest)?;
        if fields.is_empty() {
            return Err("at-sign control name is empty".into());
        }
        let name = fields.remove(0);
        return Ok(Statement::Control {
            name,
            arguments: fields,
        });
    }
    if let Some(rest) = line.strip_prefix("#include") {
        let path = rest.trim().trim_matches('"');
        if path.is_empty() {
            return Err("include path is empty".into());
        }
        return Ok(Statement::Include(path.to_string()));
    }
    if let Some(rest) = line.strip_prefix('.') {
        if let Some(message) = rest.strip_prefix("message ") {
            return Ok(Statement::Command {
                kind: CommandKind::Message,
                arguments: split_message_arguments(message)?,
            });
        }
        let mut fields = split_arguments(rest)?;
        if fields.is_empty() {
            return Err("command name is empty".into());
        }
        let name = fields.remove(0);
        if name == "label" {
            let label = fields.first().ok_or("label name is empty")?;
            return Ok(Statement::Label(label.clone()));
        }
        let kind = CommandKind::from_name(&name)
            .ok_or_else(|| format!("command is not in the Minori registry: .{name}"))?;
        return Ok(Statement::Command {
            kind,
            arguments: fields,
        });
    }
    Ok(Statement::Message(line.to_string()))
}

fn split_message_arguments(source: &str) -> Result<Vec<String>, String> {
    let (id, rest) = source
        .split_once(' ')
        .ok_or("message requires id, voice, speaker, and text")?;
    let (voice, rest) = rest
        .split_once(' ')
        .ok_or("message requires voice, speaker, and text")?;
    let (speaker, text) = rest
        .split_once(' ')
        .ok_or("message requires speaker and text")?;
    Ok(vec![id.into(), voice.into(), speaker.into(), text.into()])
}

fn split_arguments(source: &str) -> Result<Vec<String>, String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut escaped = false;
    for ch in source.chars() {
        if escaped {
            field.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && quoted {
            escaped = true;
            continue;
        }
        if ch == '"' {
            quoted = !quoted;
            continue;
        }
        if ch.is_whitespace() && !quoted {
            if !field.is_empty() {
                fields.push(std::mem::take(&mut field));
            }
        } else {
            field.push(ch);
        }
    }
    if quoted {
        return Err("unterminated quoted argument".into());
    }
    if !field.is_empty() {
        fields.push(field);
    }
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minori_control_lines() {
        assert_eq!(
            parse_line(".label start").unwrap(),
            Statement::Label("start".into())
        );
        assert_eq!(
            parse_line(".goto start").unwrap(),
            Statement::Command {
                kind: CommandKind::Goto,
                arguments: vec!["start".into()],
            }
        );
        assert_eq!(
            parse_line("@load ad image.png").unwrap(),
            Statement::Control {
                name: "load".into(),
                arguments: vec!["ad".into(), "image.png".into()],
            }
        );
        assert_eq!(
            parse_line(".message 100   \\a\\vtext").unwrap(),
            Statement::Command {
                kind: CommandKind::Message,
                arguments: vec!["100".into(), "".into(), "".into(), "\\a\\vtext".into()],
            }
        );
        assert_eq!(
            parse_line(".message 130 voice speaker words with spaces").unwrap(),
            Statement::Command {
                kind: CommandKind::Message,
                arguments: vec![
                    "130".into(),
                    "voice".into(),
                    "speaker".into(),
                    "words with spaces".into(),
                ],
            }
        );
    }

    #[test]
    fn colon_separated_variant_prefix_still_names_a_command() {
        // 701_01.sc ships `[j]:.wait 200` between two transitions.
        let source = "[e].wait 40\n[j]:.wait 200\n.wait 20\n";
        let japanese = Program::parse_with_language(source, Language::Japanese).unwrap();
        assert_eq!(
            japanese.statements[0],
            Statement::Command {
                kind: CommandKind::Wait,
                arguments: vec!["200".into()],
            }
        );
        assert_eq!(japanese.statements.len(), 2);
        let english = Program::parse_with_language(source, Language::English).unwrap();
        assert_eq!(
            english.statements[0],
            Statement::Command {
                kind: CommandKind::Wait,
                arguments: vec!["40".into()],
            }
        );
        assert_eq!(english.statements.len(), 2);
    }

    #[test]
    fn localisation_prefixes_select_one_variant() {
        let source = "[e].wait 350\n[j].wait 100\n.wait 20\n[e].label start\n";
        let english = Program::parse_with_language(source, Language::English).unwrap();
        assert_eq!(english.statements.len(), 3);
        assert_eq!(english.labels.get("start"), Some(&2));
        let japanese = Program::parse_with_language(source, Language::Japanese).unwrap();
        assert_eq!(japanese.statements.len(), 2);
        assert!(japanese.labels.is_empty());
        assert_eq!(
            japanese.statements[0],
            Statement::Command {
                kind: CommandKind::Wait,
                arguments: vec!["100".into()],
            }
        );
    }
}
