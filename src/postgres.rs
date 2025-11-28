use tracing::debug;

use crate::errors::ConnectionPoolerError;

#[derive(Debug)]
pub(crate) struct WireMessage {
    r#type: char,
    payload: String,
}

pub(crate) fn try_parse_wire_message(buffer: &[u8]) -> Result<(usize, Option<WireMessage>), ConnectionPoolerError> {
    let buffer_len = buffer.len();
    if buffer_len < 5 {
        return Ok((0, None));
    }

    let message_type = buffer[0] as char;
    let message_length = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]) as usize;
    
    // Total message size is: 1 byte (type) + message_length
    let total_message_size = 1 + message_length;
    
    // Check if we have enough data for this message
    if buffer_len < total_message_size {
        return Ok((0, None));
    }

    // The payload is from byte 5 onwards (after type byte and 4-byte length field)
    let payload = String::from_utf8_lossy(&buffer[5..total_message_size]).to_string();

    Ok((total_message_size, Some(WireMessage { r#type: message_type, payload })))
}

pub(crate) fn try_parse_wire_messages(mut buffer: &[u8]) -> Result<Vec<WireMessage>, ConnectionPoolerError> {
    let mut messages = vec![];

    while buffer.len() > 0 {
        let (bytes_consumed, wire_message) = try_parse_wire_message(buffer)?;
        buffer = &buffer[bytes_consumed..];
        if let Some(wire_message) = wire_message {
            messages.push(wire_message);
        }

        if bytes_consumed == 0 {
            break;
        }
    }

    debug!("Parsed {} messages", messages.len());
    Ok(messages)
}