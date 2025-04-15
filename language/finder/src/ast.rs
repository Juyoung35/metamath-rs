use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Document {
    pub major_parts: Vec<MajorPart>,
}
impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for major_part in &self.major_parts {
            major_part.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct MajorPart {
    pub title: String,
    pub statements: Vec<Statement>,
    pub sections: Vec<Section>,
}
impl Display for MajorPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# {}", self.title)?;
        for statement in &self.statements {
            writeln!(f, "    {}", statement)?;
        }
        for section in &self.sections {
            for line in format!("{}", section).split("\n") {
                writeln!(f, "    {}", line)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Section {
    pub title: String,
    pub statements: Vec<Statement>,
    pub subsections: Vec<SubSection>,
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "## {}", self.title)?;
        for statement in &self.statements {
            writeln!(f, "    {}", statement)?;
        }
        for subsection in &self.subsections {
            for line in format!("{}", subsection).split("\n") {
                writeln!(f, "    {}", line)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct SubSection {
    pub title: String,
    pub statements: Vec<Statement>,
    pub subsubsections: Vec<SubsubSection>,
}
impl Display for SubSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "### {}", self.title)?;
        for statement in &self.statements {
            writeln!(f, "    {}", statement)?;
        }
        for subsubsection in &self.subsubsections {
            for line in format!("{}", subsubsection).split("\n") {
                writeln!(f, "    {}", line)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct SubsubSection {
    pub title: String,
    pub statements: Vec<Statement>,
}
impl Display for SubsubSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#### {}", self.title)?;
        for statement in &self.statements {
            writeln!(f, "    {}", statement)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    ConstantDeclaration(ConstantDeclaration),
    AxiomaticAssertion(AxiomaticAssertion),
    DefinitionAssertion(AxiomaticAssertion),
    WffAssertion(AxiomaticAssertion),
    ClassAssertion(AxiomaticAssertion),
    OtherAssertion(AxiomaticAssertion),
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstantDeclaration(const_decl) => const_decl.fmt(f),
            Self::AxiomaticAssertion(axiom_assert) => {
                write!(f, "AXIOM ")?;
                axiom_assert.fmt(f)
            },
            Self::DefinitionAssertion(def_assert) => {
                write!(f, "DEFINITION ")?;
                def_assert.fmt(f)
            },
            Self::WffAssertion(wff_assert) => {
                write!(f, "WFF ")?;
                wff_assert.fmt(f)
            },
            Self::ClassAssertion(class_assert) => {
                write!(f, "CLASS ")?;
                class_assert.fmt(f)
            },
            Self::OtherAssertion(other_assert) => {
                write!(f, "OTHER ")?;
                other_assert.fmt(f)
            }

        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ConstantDeclaration {
    pub symbol_list: Vec<String>,
}
impl Display for ConstantDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$c")?;
        for symbol in &self.symbol_list {
            write!(f, " {}", symbol)?;
        }
        write!(f, " $.")?;
        Ok(())
    }
}


#[derive(Clone, Debug, Default)]
pub struct AxiomaticAssertion {
    pub label: String,
    pub symbol_list: Vec<String>,
}
impl Display for AxiomaticAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} $a", self.label)?;
        for symbol in &self.symbol_list {
            write!(f, " {}", symbol)?;
        }
        write!(f, " $.")?;
        Ok(())
    }
}