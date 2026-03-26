use crate::error::AppError;

use super::models::{BrowserRunResult, LightpandaRawOutput};

pub fn parse_output(raw: LightpandaRawOutput) -> Result<BrowserRunResult, AppError> {
    if !raw.ok {
        return Err(AppError::Runner(
            raw.stderr.unwrap_or_else(|| "lightpanda execution failed".to_string()),
        ));
    }

    Ok(BrowserRunResult {
        result_json: raw.result.unwrap_or_default(),
        logs: raw.stdout.map(|s| vec![s]).unwrap_or_default(),
        artifacts: Vec::new(),
    })
}
