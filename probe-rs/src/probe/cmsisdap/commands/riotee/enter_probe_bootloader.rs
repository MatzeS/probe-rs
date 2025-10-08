use super::super::{CommandId, Request, SendError};

#[derive(Clone, Copy, Debug)]
pub struct EnterProbeBootloader;

impl Request for EnterProbeBootloader {
    const COMMAND_ID: CommandId = CommandId::RioteeEnterProbeBootloader;

    type Response = ();

    fn to_bytes(&self, _buffer: &mut [u8]) -> Result<usize, SendError> {
        Ok(0)
    }

    fn parse_response(&self, _buffer: &[u8]) -> Result<Self::Response, SendError> {
        Ok(())
    }
}
