use jetted_core::target::{self, inflect, metadata};
use jetted_core::Result;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
}

pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

impl jetted_core::target::Target for Target {
    type FileState = ();

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("index.ts".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::NativeSupport,
            booleans_are_nullable: false,
            int8s_are_nullable: false,
            uint8s_are_nullable: false,
            int16s_are_nullable: false,
            uint16s_are_nullable: false,
            int32s_are_nullable: false,
            uint32s_are_nullable: false,
            float32s_are_nullable: false,
            float64s_are_nullable: false,
            strings_are_nullable: false,
            timestamps_are_nullable: false,
            arrays_are_nullable: false,
            dicts_are_nullable: false,
            aliases_are_nullable: false,
            enums_are_nullable: false,
            structs_are_nullable: false,
            discriminators_are_nullable: false,
        }
    }

    fn name(&self, kind: target::NameableKind, parts: &[String]) -> String {
        match kind {
            target::NameableKind::Type => TYPE_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::EnumMember => ENUM_MEMBER_NAMING_CONVENTION.inflect(parts),

            // Not used. TypeScript maps directly to the JSON data, so we don't
            // have the option of distinguishing the JSON name from the
            // TypeScript name
            target::NameableKind::Field => "".into(),
        }
    }

    fn expr(&self, _state: &mut (), metadata: metadata::Metadata, expr: target::Expr) -> String {
        if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
            return s.into();
        }

        match expr {
            target::Expr::Empty => "any".into(),
            target::Expr::Boolean => "boolean".into(),
            target::Expr::Int8 => "number".into(),
            target::Expr::Uint8 => "number".into(),
            target::Expr::Int16 => "number".into(),
            target::Expr::Uint16 => "number".into(),
            target::Expr::Int32 => "number".into(),
            target::Expr::Uint32 => "number".into(),
            target::Expr::Float32 => "number".into(),
            target::Expr::Float64 => "number".into(),
            target::Expr::String => "string".into(),
            target::Expr::Timestamp => "string".into(),
            target::Expr::ArrayOf(sub_expr) => format!("{}[]", sub_expr),
            target::Expr::DictOf(sub_expr) => format!("{{ [key: string]: {} }}", sub_expr),
            target::Expr::NullableOf(sub_expr) => format!("({} | null)", sub_expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        _state: &mut (),
        item: target::Item,
    ) -> Result<Option<String>> {
        Ok(match item {
            target::Item::Auxiliary { .. } => {
                // No auxiliary files needed.
                None
            }

            target::Item::Preamble => {
                writeln!(
                    out,
                    "// Code generated by jetted for TypeScript v{}",
                    env!("CARGO_PKG_VERSION")
                )?;

                None
            }

            target::Item::Postamble => None,

            target::Item::Alias {
                metadata,
                name,
                type_,
            } => {
                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "export type {} = {};", name, type_)?;

                None
            }

            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "export enum {} {{", name)?;
                for (index, member) in members.iter().enumerate() {
                    let description = enum_variant_description(&metadata, 1, &member.json_value);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }

                    write!(out, "{}", &description)?;
                    writeln!(out, "  {} = {:?},", member.name, member.json_value)?;
                }
                writeln!(out, "}}")?;

                None
            }

            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "export interface {} {{", name)?;
                for (index, field) in fields.iter().enumerate() {
                    let description = description(&field.metadata, 1);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }

                    write!(out, "{}", &description)?;
                    if field.optional {
                        writeln!(
                            out,
                            "  {}?: {};",
                            format_property(field.json_name.clone()),
                            field.type_
                        )?;
                    } else {
                        writeln!(
                            out,
                            "  {}: {};",
                            format_property(field.json_name.clone()),
                            field.type_
                        )?;
                    }
                }
                writeln!(out, "}}")?;

                None
            }

            target::Item::Discriminator {
                metadata,
                name,
                variants,
                ..
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(
                    out,
                    "export type {} = {};",
                    name,
                    variants
                        .into_iter()
                        .map(|v| v.type_name)
                        .collect::<Vec<_>>()
                        .join(" | ")
                )?;

                None
            }

            target::Item::DiscriminatorVariant {
                metadata,
                name,
                tag_json_name,
                tag_value,
                fields,
                ..
            } => {
                if let Some(s) = metadata.get("typescriptType").and_then(|v| v.as_str()) {
                    return Ok(Some(s.into()));
                }

                writeln!(out)?;
                write!(out, "{}", description(&metadata, 0))?;
                writeln!(out, "export interface {} {{", name)?;
                writeln!(out, "  {}: {:?};", tag_json_name, tag_value)?;
                for field in &fields {
                    let description = description(&field.metadata, 1);

                    if !description.is_empty() {
                        writeln!(out)?;
                    }

                    write!(out, "{}", &description)?;
                    if field.optional {
                        writeln!(
                            out,
                            "  {}?: {};",
                            format_property(field.json_name.clone()),
                            field.type_
                        )?;
                    } else {
                        writeln!(
                            out,
                            "  {}: {};",
                            format_property(field.json_name.clone()),
                            field.type_
                        )?;
                    }
                }
                writeln!(out, "}}")?;

                None
            }
        })
    }
}

fn format_property(s: String) -> String {
    // This implements a conservative subset of the set of allowable identifiers
    // in JavaScript:
    //
    // https://tc39.es/ecma262/#sec-names-and-keywords
    //
    // If a property isn't an allowable identifier by these rules, then we
    // escape the property.
    lazy_static! {
        static ref IDENTIFIER: Regex = Regex::new("^[a-zA-Z_$][a-zA-Z0-9_$]*$").unwrap();
    }

    if IDENTIFIER.is_match(&s) {
        s
    } else {
        format!("{:?}", s)
    }
}

pub fn description(metadata: &BTreeMap<String, Value>, indent: usize) -> String {
    doc(indent, jetted_core::target::metadata::description(metadata))
}

pub fn enum_variant_description(
    metadata: &BTreeMap<String, Value>,
    indent: usize,
    value: &str,
) -> String {
    doc(
        indent,
        jetted_core::target::metadata::enum_variant_description(metadata, value),
    )
}

fn doc(ident: usize, s: &str) -> String {
    let prefix = "  ".repeat(ident);
    jetted_core::target::fmt::comment_block(
        &format!("{}/**", prefix),
        &format!("{} * ", prefix),
        &format!("{} */", prefix),
        s,
    )
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jetted_test::std_test_cases!(&crate::Target::new());
    }

    mod optional_std_tests {
        jetted_test::strict_std_test_case!(&crate::Target::new(), empty_and_nonascii_properties);

        jetted_test::strict_std_test_case!(&crate::Target::new(), empty_and_nonascii_enum_values);
    }
}
