use super::super::{CommandId, Request, SendError};

#[derive(Clone, Copy, Debug)]
pub struct TargetPower {
    on: bool,
}

impl TargetPower {
    pub fn new(on: bool) -> Self {
        Self { on }
    }
}

impl Request for TargetPower {
    const COMMAND_ID: CommandId = CommandId::RioteeTargetPower;

    type Response = RioteeStatusResponse;

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, SendError> {
        buffer[0] = if self.on { 1 } else { 0 };
        Ok(1)
    }

    fn parse_response(&self, _buffer: &[u8]) -> Result<Self::Response, SendError> {
        Ok(RioteeStatusResponse)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RioteeStatusResponse;
