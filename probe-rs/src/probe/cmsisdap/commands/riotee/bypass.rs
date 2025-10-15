use super::super::{CommandId, Request, SendError};

// TODO naming is obviously terrible

#[derive(Clone, Copy, Debug)]
pub struct Bypass {
    on: bool,
}

impl Bypass {
    pub fn new(on: bool) -> Self {
        Self { on }
    }
}

impl Request for Bypass {
    const COMMAND_ID: CommandId = CommandId::RioteeBypass;

    type Response = ();

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, SendError> {
        buffer[0] = if self.on { 1 } else { 0 };
        Ok(1)
    }

    fn parse_response(&self, _buffer: &[u8]) -> Result<Self::Response, SendError> {
        Ok(())
    }
}
