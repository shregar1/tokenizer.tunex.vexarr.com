//! Tokenizer resume validation.

/// One tokenizer resume validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResumeValidationError {
    /// Invalid field path.
    pub path: String,
    /// Human-readable validation message.
    pub message: String,
}

impl ResumeValidationError {
    /// Creates a validation error.
    #[must_use]
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Tokenizer resume validation report.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResumeValidationReport {
    /// Validation errors.
    pub errors: Vec<ResumeValidationError>,
}

impl ResumeValidationReport {
    /// Returns true when no validation errors exist.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Pushes one validation error.
    pub fn push(&mut self, path: impl Into<String>, message: impl Into<String>) {
        self.errors.push(ResumeValidationError::new(path, message));
    }
}

impl std::fmt::Display for ResumeValidationReport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return formatter.write_str("resume state is valid");
        }

        for (index, error) in self.errors.iter().enumerate() {
            if index > 0 {
                formatter.write_str("; ")?;
            }
            write!(formatter, "{}: {}", error.path, error.message)?;
        }

        Ok(())
    }
}
