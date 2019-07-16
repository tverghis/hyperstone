use nom::bytes::complete::{tag, take};
use nom::Err::Incomplete;
use nom::IResult;
use nom::Needed::Unknown;
use snap::Decoder;

use crate::outer_message::OuterMessage;
use crate::protos::demo::EDemoCommands;

const IS_COMPRESSED_MASK: u32 = EDemoCommands::DEM_IsCompressed as u32;

pub fn take_source2_signature(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(b"PBDEMS2\0")(input)
}

pub fn take_replay_size_info(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(8u8)(input)
}

pub fn take_outer_message(input: &[u8]) -> IResult<&[u8], OuterMessage> {
    let (input, message) = take_command(input)?;

    let message_type = message & !IS_COMPRESSED_MASK;
    let is_compressed = (message & IS_COMPRESSED_MASK) == IS_COMPRESSED_MASK;

    let (input, mut tick) = take_tick(input)?;
    if tick == std::u32::MAX {
        tick = 0;
    }

    let (input, size) = take_size(input)?;

    let (input, data) = take_data(input, size)?;

    let data = if is_compressed {
        let mut decoder = Decoder::new();
        decoder.decompress_vec(data).unwrap()
    } else {
        data.into()
    };

    Ok((input, OuterMessage::new(tick, message_type, data)))
}

pub fn take_command(input: &[u8]) -> IResult<&[u8], u32> {
    take_varint(input)
}

pub fn take_tick(input: &[u8]) -> IResult<&[u8], u32> {
    take_varint(input)
}

pub fn take_size(input: &[u8]) -> IResult<&[u8], u32> {
    take_varint(input)
}

pub fn take_data(input: &[u8], size: u32) -> IResult<&[u8], &[u8]> {
    take(size)(input)
}

pub fn take_varint(input: &[u8]) -> IResult<&[u8], u32> {
    let mut res: usize = 0;
    let mut count: usize = 0;
    let mut remainder = input;

    loop {
        let byte = match take::<usize, &[u8], ()>(1)(remainder) {
            Ok((rest, bytes)) => {
                remainder = rest;
                bytes[0]
            }
            Err(_) => return Err(Incomplete(Unknown)),
        };
        res |= ((byte as usize) & 127) << (count * 7);
        count += 1;

        if (count == 5) || ((byte >> 7) == 0) {
            return Ok((remainder, res as u32));
        }
    }
}
