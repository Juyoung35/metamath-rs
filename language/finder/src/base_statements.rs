use std::{fs::File, io::{Read, Write as IoWrite}, fmt::Write as FmtWrite};
use nom::{bytes::complete::tag, Parser};

pub fn find_and_write_base_statements(file_path: &str, from: usize, to: usize) {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lookbehind = b' ';
    let mut buffer = Vec::new();
    let mut is_comment = false;
    let mut is_const_decl = false;
    let mut is_axiom_assert = false;
    let mut document = crate::ast::Document::default();
    let mut last_nonspace_start = None;
    let mut last_nonspace_end = None;
    let mut last_nonspace_start_candidate = 0;
    let mut axiom_buffer = Vec::new();
    let mut axiom_label = String::new();

    let mut o = String::new();

    for (i, &b) in contents.as_bytes()[from..to].into_iter().enumerate() {
        if lookbehind.is_ascii_whitespace() && !b.is_ascii_whitespace() {
            last_nonspace_start_candidate = i;
        }
        if !lookbehind.is_ascii_whitespace() && b.is_ascii_whitespace() {
            last_nonspace_start = Some(last_nonspace_start_candidate);
            last_nonspace_end = Some(i - 1);
            if is_axiom_assert {
                let symbol = String::from_utf8_lossy(&contents.as_bytes()[from..to][last_nonspace_start.unwrap()..=last_nonspace_end.unwrap()]).to_string();
                if symbol.as_str() != "$a" {
                    axiom_buffer.push(symbol);
                }
            }
        }
        if !is_comment && lookbehind == b'$' && b == b'(' {
            is_comment = true;
        }
        if !is_comment && lookbehind == b'$' && b == b'c' {
            is_const_decl = true;
        }
        if !is_comment && lookbehind == b'$' && b == b'a' {
            axiom_label = String::from_utf8_lossy(&contents.as_bytes()[from..to][last_nonspace_start.unwrap()..=last_nonspace_end.unwrap()]).into_owned();
            is_axiom_assert = true;
        }
        if is_comment | is_const_decl | is_axiom_assert {
            buffer.push(lookbehind);
        }
        if lookbehind == b'$' && b == b')' {
            is_comment = false;
            buffer.push(b);
            if let Ok((_, (title, _))) = db_parser::document::heading_comment('#', '#').parse(&buffer) {
                let title = String::from_utf8_lossy(title.into()).into_owned();
                document.major_parts.push(crate::ast::MajorPart {
                    title,
                    statements: Vec::new(),
                    sections: Vec::new(),
                });
            } else if let Ok((_, (title, _))) = db_parser::document::heading_comment('*', '#').parse(&buffer) {
                let title = String::from_utf8_lossy(title.into()).into_owned();
                if document.major_parts.last().is_none() {
                    document.major_parts.push(crate::ast::MajorPart {
                        title: String::from("EmptyMajorPart"),
                        statements: Vec::new(),
                        sections: Vec::new(),
                    });
                }
                document.major_parts.last_mut().unwrap().sections.push(crate::ast::Section {
                    title,
                    statements: Vec::new(),
                    subsections: Vec::new(),
                });
            } else if let Ok((_, (title, _))) = db_parser::document::heading_comment('-', '=').parse(&buffer) {
                let title = String::from_utf8_lossy(title.into()).into_owned();
                if document.major_parts.last().is_none() {
                    document.major_parts.push(crate::ast::MajorPart {
                        title: String::from("EmptyMajorPart"),
                        statements: Vec::new(),
                        sections: Vec::new(),
                    });
                }
                if document.major_parts.last().unwrap().sections.last().is_none() {
                    document.major_parts.last_mut().unwrap().sections.push(crate::ast::Section {
                        title: String::from("EmptySection"),
                        statements: Vec::new(),
                        subsections: Vec::new(),
                    });
                }
                document.major_parts.last_mut().unwrap().sections.last_mut().unwrap().subsections.push(crate::ast::SubSection {
                    title,
                    statements: Vec::new(),
                    subsubsections: Vec::new(),
                });
            } else if let Ok((_, (title, _))) = db_parser::document::heading_comment('.', '-').parse(&buffer) {
                let title = String::from_utf8_lossy(title.into()).into_owned();
                if document.major_parts.last().is_none() {
                    document.major_parts.push(crate::ast::MajorPart {
                        title: String::from("EmptyMajorPart"),
                        statements: Vec::new(),
                        sections: Vec::new(),
                    });
                }
                if document.major_parts.last().unwrap().sections.last().is_none() {
                    document.major_parts.last_mut().unwrap().sections.push(crate::ast::Section {
                        title: String::from("EmptySection"),
                        statements: Vec::new(),
                        subsections: Vec::new(),
                    });
                }
                if document.major_parts.last().unwrap().sections.last().unwrap().subsections.last().is_none() {
                    document.major_parts.last_mut().unwrap().sections.last_mut().unwrap().subsections.push(crate::ast::SubSection {
                        title: String::from("EmptySubSection"),
                        statements: Vec::new(),
                        subsubsections: Vec::new(),
                    });
                }
                document.major_parts.last_mut().unwrap().sections.last_mut().unwrap().subsections.last_mut().unwrap().subsubsections.push(crate::ast::SubsubSection {
                    title,
                    statements: Vec::new(),
                });
            }
            buffer.clear();
        }
        if lookbehind == b'$' && b == b'.' {
            buffer.push(b);
            if is_const_decl {
                let const_decl = crate::ast::ConstantDeclaration {
                    symbol_list: db_parser::statement::constant_declaration(&buffer).unwrap().1.symbol_list.into_iter().map(|symbol| String::from_utf8_lossy(symbol.0.into()).into_owned()).collect(),
                };
                let stmt = crate::ast::Statement::ConstantDeclaration(const_decl);
                if let Some(major_part) = document.major_parts.last_mut() {
                    if let Some(section) = major_part.sections.last_mut() {
                        if let Some(subsection) = section.subsections.last_mut() {
                            if let Some(subsubsection) = subsection.subsubsections.last_mut() {
                                subsubsection.statements.push(stmt);
                            } else {
                                subsection.statements.push(stmt);
                            }
                        } else {
                            section.statements.push(stmt);
                        }
                    } else {
                        major_part.statements.push(stmt);
                    }
                }
            }
            if is_axiom_assert {
                let label = axiom_label.clone();
                let axiom_assert = crate::ast::AxiomaticAssertion {
                    label,
                    symbol_list: axiom_buffer.clone(),
                };
                axiom_buffer.clear();
                let stmt = if &axiom_assert.label[0..2] == "ax" {
                    crate::ast::Statement::AxiomaticAssertion(axiom_assert)
                } else if &axiom_assert.label[0..2] == "df" {
                    crate::ast::Statement::DefinitionAssertion(axiom_assert)
                } else if &axiom_assert.label[0..1] == "w" {
                    crate::ast::Statement::WffAssertion(axiom_assert)
                } else if &axiom_assert.label[0..1] == "c" {
                    crate::ast::Statement::ClassAssertion(axiom_assert)
                } else {
                    crate::ast::Statement::OtherAssertion(axiom_assert)
                };
                if let Some(major_part) = document.major_parts.last_mut() {
                    if let Some(section) = major_part.sections.last_mut() {
                        if let Some(subsection) = section.subsections.last_mut() {
                            if let Some(subsubsection) = subsection.subsubsections.last_mut() {
                                subsubsection.statements.push(stmt);
                            } else {
                                subsection.statements.push(stmt);
                            }
                        } else {
                            section.statements.push(stmt);
                        }
                    } else {
                        major_part.statements.push(stmt);
                    }
                }
            }
            is_const_decl = false;
            is_axiom_assert = false;
            buffer.clear();
        }
        lookbehind = b;
    }
    let output_file_name = format!("../{}-base_statements.txt", file_path.split('/').last().unwrap());
    let mut output_file = File::create(output_file_name).unwrap();
    output_file.write(&format!("{document}").as_bytes()).unwrap();
}