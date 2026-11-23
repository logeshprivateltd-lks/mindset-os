//! Framed Wire Protocol & Packet Envelope with Strict Size Bounds.
//! Protects against unbounded allocations and buffer overflow attacks during peer sync.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::ProtocolError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Maximum allowable frame payload size across peer wireless connections (16 MB).
pub const MAX_FRAME_PAYLOAD_BYTES: usize = 16 * 1024 * 1024;

/// Maximum allowable raw JSON message size for control messages (2 MB).
pub const MAX_CONTROL_MESSAGE_BYTES: usize = 2 * 1024 * 1024;

/// Protocol packet frame header and enveloped typed payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolFrame {
    /// Incremental message sequence ID.
    pub sequence: u64,
    /// Originating device UUID.
    pub sender_id: String,
    /// Human-readable message category or route tag (e.g., "sync_oplog", "continuity", "heartbeat").
    pub route: String,
    /// Raw serialized payload bytes.
    pub payload: Vec<u8>,
}

impl ProtocolFrame {
    /// Creates a bounded protocol frame wrapping an arbitrary serializable payload.
    pub fn encode<T: Serialize>(
        sequence: u64,
        sender_id: String,
        route: String,
        data: &T,
    ) -> Result<Self, ProtocolError> {
        let payload = serde_json::to_vec(data).map_err(ProtocolError::Serialization)?;

        if payload.len() > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ProtocolError::FramePayloadTooLarge {
                size: payload.len(),
                max: MAX_FRAME_PAYLOAD_BYTES,
            });
        }

        Ok(Self {
            sequence,
            sender_id,
            route,
            payload,
        })
    }

    /// Deserializes the enveloped frame payload into target typed structure `T`.
    /// Validates payload length against `MAX_FRAME_PAYLOAD_BYTES` to prevent denial-of-service allocations.
    pub fn decode<T: DeserializeOwned>(&self) -> Result<T, ProtocolError> {
        if self.payload.len() > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ProtocolError::FramePayloadTooLarge {
                size: self.payload.len(),
                max: MAX_FRAME_PAYLOAD_BYTES,
            });
        }

        serde_json::from_slice(&self.payload).map_err(ProtocolError::Serialization)
    }

    /// Serializes entire frame to wire format bytes, validating total bounds.
    pub fn to_wire_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        let bytes = serde_json::to_vec(self).map_err(ProtocolError::Serialization)?;
        if bytes.len() > MAX_FRAME_PAYLOAD_BYTES + 4096 {
            return Err(ProtocolError::FramePayloadTooLarge {
                size: bytes.len(),
                max: MAX_FRAME_PAYLOAD_BYTES + 4096,
            });
        }
        Ok(bytes)
    }

    /// Deserializes frame from wire format slice with strict upfront length verification.
    pub fn from_wire_slice(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() > MAX_FRAME_PAYLOAD_BYTES + 4096 {
            return Err(ProtocolError::FramePayloadTooLarge {
                size: bytes.len(),
                max: MAX_FRAME_PAYLOAD_BYTES + 4096,
            });
        }

        let frame: Self = serde_json::from_slice(bytes).map_err(ProtocolError::Serialization)?;
        if frame.payload.len() > MAX_FRAME_PAYLOAD_BYTES {
            return Err(ProtocolError::FramePayloadTooLarge {
                size: frame.payload.len(),
                max: MAX_FRAME_PAYLOAD_BYTES,
            });
        }

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct TestMsg {
        content: String,
    }

    #[test]
    fn test_protocol_frame_roundtrip() {
        let msg = TestMsg {
            content: "Hello MindSet Connect Peer".to_string(),
        };

        let frame = ProtocolFrame::encode(1, "laptop-01".into(), "test".into(), &msg).unwrap();
        let decoded: TestMsg = frame.decode().unwrap();
        assert_eq!(decoded, msg);

        let wire = frame.to_wire_bytes().unwrap();
        let frame2 = ProtocolFrame::from_wire_slice(&wire).unwrap();
        assert_eq!(frame, frame2);
    }

    #[test]
    fn test_protocol_frame_payload_too_large() {
        let oversized = vec![0u8; MAX_FRAME_PAYLOAD_BYTES + 10];
        let frame = ProtocolFrame {
            sequence: 1,
            sender_id: "attacker".into(),
            route: "flood".into(),
            payload: oversized,
        };

        let res: Result<TestMsg, ProtocolError> = frame.decode();
        assert!(matches!(
            res,
            Err(ProtocolError::FramePayloadTooLarge { .. })
        ));
    }
}
