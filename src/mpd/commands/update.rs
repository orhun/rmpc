use crate::mpd::errors::MpdError;
use crate::mpd::{FromMpd, LineHandled};

#[derive(Default, Debug, Clone, Copy)]
pub struct Update {
    pub job_id: u32,
}

impl FromMpd for Update {
    fn next_internal(&mut self, _key: &str, value: String) -> Result<LineHandled, MpdError> {
        match value.as_str() {
            "updating_db" => self.job_id = value.parse()?,
            _ => return Ok(LineHandled::No { value }),
        };
        Ok(LineHandled::Yes)
    }
}
