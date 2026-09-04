//! Blank-id rejection, unsafe-id rejection, and path-segment encoding for ops
//! UI hrefs.

/// Blank, oversized, or path-unsafe job id/name, run id, or script name rejected
/// before Chronon lookups.
///
/// Callers map this into Leptos `ServerFnError` (or equivalent) at the `#[server]`
/// boundary; the Display text stays stable for UI and contract tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChrononIdError {
    /// Job id was empty or whitespace-only.
    EmptyJobId,
    /// Job id-or-name key was empty or whitespace-only.
    EmptyJobIdOrName,
    /// Run id was empty or whitespace-only.
    EmptyRunId,
    /// Job name was empty or whitespace-only.
    EmptyJobName,
    /// Script name was empty or whitespace-only.
    EmptyScriptName,
    /// Job id exceeded [`MAX_CHRONON_ID_CHARS`].
    JobIdTooLong,
    /// Job id-or-name key exceeded [`MAX_CHRONON_ID_CHARS`].
    JobIdOrNameTooLong,
    /// Run id exceeded [`MAX_CHRONON_ID_CHARS`].
    RunIdTooLong,
    /// Job name exceeded [`MAX_CHRONON_ID_CHARS`].
    JobNameTooLong,
    /// Script name exceeded [`MAX_CHRONON_ID_CHARS`].
    ScriptNameTooLong,
    /// Job id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeJobId,
    /// Job id-or-name key contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeJobIdOrName,
    /// Run id contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeRunId,
    /// Job name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeJobName,
    /// Script name contained `/`, `\`, controls, or was `.` / `..`.
    UnsafeScriptName,
}

impl std::fmt::Display for ChrononIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyJobId => write!(f, "Job id is required"),
            Self::EmptyJobIdOrName => write!(f, "Job id or name is required"),
            Self::EmptyRunId => write!(f, "Run id is required"),
            Self::EmptyJobName => write!(f, "Job name is required"),
            Self::EmptyScriptName => write!(f, "Script selection is required"),
            Self::JobIdTooLong => write!(f, "Job id is too long"),
            Self::JobIdOrNameTooLong => write!(f, "Job id or name is too long"),
            Self::RunIdTooLong => write!(f, "Run id is too long"),
            Self::JobNameTooLong => write!(f, "Job name is too long"),
            Self::ScriptNameTooLong => write!(f, "Script name is too long"),
            Self::UnsafeJobId => {
                write!(f, "Job id contains unsafe path characters")
            }
            Self::UnsafeJobIdOrName => {
                write!(f, "Job id or name contains unsafe path characters")
            }
            Self::UnsafeRunId => {
                write!(f, "Run id contains unsafe path characters")
            }
            Self::UnsafeJobName => {
                write!(f, "Job name contains unsafe path characters")
            }
            Self::UnsafeScriptName => {
                write!(f, "Script name contains unsafe path characters")
            }
        }
    }
}

impl std::error::Error for ChrononIdError {}

/// Maximum Unicode scalar count for job ids/names, run ids, and script names
/// accepted by ops detail lookups.
pub const MAX_CHRONON_ID_CHARS: usize = 256;

/// ASCII controls (C0 + DEL) plus path separators. Avoids `char::is_control`,
/// which is not `const` on the leptos-lints pinned nightly.
const fn is_unsafe_ops_id_char(c: char) -> bool {
    c <= '\u{1f}' || c == '\u{7f}' || c == '/' || c == '\\'
}

fn check_ops_id(raw: &str) -> Result<&str, ChrononIdErrorKind> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ChrononIdErrorKind::Empty);
    }
    if trimmed.chars().count() > MAX_CHRONON_ID_CHARS {
        return Err(ChrononIdErrorKind::TooLong);
    }
    if trimmed == "." || trimmed == ".." {
        return Err(ChrononIdErrorKind::Unsafe);
    }
    if trimmed.chars().any(is_unsafe_ops_id_char) {
        return Err(ChrononIdErrorKind::Unsafe);
    }
    Ok(trimmed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChrononIdErrorKind {
    Empty,
    TooLong,
    Unsafe,
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` job ids
/// before detail / update / run-now lookups.
///
/// # Errors
///
/// Returns a [`ChrononIdError`] variant when the id is empty/whitespace-only,
/// longer than [`MAX_CHRONON_ID_CHARS`], contains `/` `\` or ASCII controls, or is
/// exactly `.` / `..`.
pub fn validate_job_id(job_id: &str) -> Result<(), ChrononIdError> {
    match check_ops_id(job_id) {
        Ok(_) => Ok(()),
        Err(ChrononIdErrorKind::Empty) => Err(ChrononIdError::EmptyJobId),
        Err(ChrononIdErrorKind::TooLong) => Err(ChrononIdError::JobIdTooLong),
        Err(ChrononIdErrorKind::Unsafe) => Err(ChrononIdError::UnsafeJobId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` job
/// id-or-name keys before resolve / `get_job` lookups.
///
/// # Errors
///
/// Returns a [`ChrononIdError`] variant when the key fails the same rules as
/// [`validate_job_id`].
pub fn validate_job_id_or_name(job_id_or_name: &str) -> Result<(), ChrononIdError> {
    match check_ops_id(job_id_or_name) {
        Ok(_) => Ok(()),
        Err(ChrononIdErrorKind::Empty) => Err(ChrononIdError::EmptyJobIdOrName),
        Err(ChrononIdErrorKind::TooLong) => Err(ChrononIdError::JobIdOrNameTooLong),
        Err(ChrononIdErrorKind::Unsafe) => Err(ChrononIdError::UnsafeJobIdOrName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` run ids
/// before run detail lookups.
///
/// # Errors
///
/// Returns a [`ChrononIdError`] variant when the id fails the same rules as
/// [`validate_job_id`].
pub fn validate_run_id(run_id: &str) -> Result<(), ChrononIdError> {
    match check_ops_id(run_id) {
        Ok(_) => Ok(()),
        Err(ChrononIdErrorKind::Empty) => Err(ChrononIdError::EmptyRunId),
        Err(ChrononIdErrorKind::TooLong) => Err(ChrononIdError::RunIdTooLong),
        Err(ChrononIdErrorKind::Unsafe) => Err(ChrononIdError::UnsafeRunId),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` job names
/// before create / uniqueness checks.
///
/// # Errors
///
/// Returns a [`ChrononIdError`] variant when the name fails the same rules as
/// [`validate_job_id`].
pub fn validate_job_name(job_name: &str) -> Result<(), ChrononIdError> {
    match check_ops_id(job_name) {
        Ok(_) => Ok(()),
        Err(ChrononIdErrorKind::Empty) => Err(ChrononIdError::EmptyJobName),
        Err(ChrononIdErrorKind::TooLong) => Err(ChrononIdError::JobNameTooLong),
        Err(ChrononIdErrorKind::Unsafe) => Err(ChrononIdError::UnsafeJobName),
    }
}

/// Rejects blank, oversized, path-separating, control, or `.` / `..` script names
/// before registry lookups.
///
/// # Errors
///
/// Returns a [`ChrononIdError`] variant when the name fails the same rules as
/// [`validate_job_id`].
pub fn validate_script_name(script_name: &str) -> Result<(), ChrononIdError> {
    match check_ops_id(script_name) {
        Ok(_) => Ok(()),
        Err(ChrononIdErrorKind::Empty) => Err(ChrononIdError::EmptyScriptName),
        Err(ChrononIdErrorKind::TooLong) => Err(ChrononIdError::ScriptNameTooLong),
        Err(ChrononIdErrorKind::Unsafe) => Err(ChrononIdError::UnsafeScriptName),
    }
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

/// Percent-encode a single path (or query-value) segment for `/chronon/...` hrefs.
///
/// Leaves RFC 3986 unreserved characters alone (`ALPHA` / `DIGIT` / `-` `.` `_`
/// `~`). Encodes `/`, `\`, controls, spaces, and other bytes so Orbital
/// `paths::*` format strings cannot smuggle extra path segments.
#[must_use]
pub fn encode_ops_path_segment(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for &b in raw.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0f) as usize] as char);
            }
        }
    }
    out
}

/// `/chronon/jobs/{encoded}` detail href.
#[must_use]
pub fn chronon_job_path(job_id: &str) -> String {
    format!("/chronon/jobs/{}", encode_ops_path_segment(job_id))
}

/// `/chronon/runs/{encoded}` detail href.
#[must_use]
pub fn chronon_run_path(run_id: &str) -> String {
    format!("/chronon/runs/{}", encode_ops_path_segment(run_id))
}
