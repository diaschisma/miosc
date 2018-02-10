extern crate rosc;

use rosc::{OscType};

#[derive(Debug, Clone, PartialEq)]
pub enum MioscMessage {
    Reference(f32),
    NoteOn(i32, f32, f32),
    NoteOff(i32),
    Pitch(i32, f32, f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MioscError {
    UnknownMessage,
    InvalidMessage,
}

impl From<MioscMessage> for rosc::OscMessage {
    fn from(source: MioscMessage) -> Self {
        use rosc::{OscMessage, OscType};
        use MioscMessage::*;

        match source {
            Reference(pitch) => OscMessage {
                addr: "/m/reference".into(),
                args: Some(vec![
                    OscType::Float(pitch)
                ]),
            },
            NoteOn(id, pitch, vel) => OscMessage {
                addr: "/m/note_on".into(),
                args: Some(vec![
                    OscType::Int(id),
                    OscType::Float(pitch),
                    OscType::Float(vel),
                ]),
            },
            NoteOff(id) => OscMessage {
                addr: "/m/note_off".into(),
                args: Some(vec![
                    OscType::Int(id),
                ]),
            },
            Pitch(id, pitch, time) => OscMessage {
                addr: "/m/pitch".into(),
                args: Some(vec![
                    OscType::Int(id),
                    OscType::Float(pitch),
                    OscType::Float(time),
                ]),
            },
        }
    }
}

fn pop_float(vec: &mut Vec<rosc::OscType>) -> Result<f32, MioscError> {
    match vec.pop() {
        Some(OscType::Float(f)) => Ok(f),
        _ => Err(MioscError::InvalidMessage),
    }
}

fn pop_integer(vec: &mut Vec<rosc::OscType>) -> Result<i32, MioscError> {
    match vec.pop() {
        Some(OscType::Int(i)) => Ok(i),
        _ => Err(MioscError::InvalidMessage),
    }
}

pub fn into_miosc(msg: rosc::OscMessage) -> Result<MioscMessage, MioscError> {
    let (addr, args) = (msg.addr, msg.args);
    match &*addr {
        "/m/reference" => {
            let mut args = args.ok_or(MioscError::InvalidMessage)?;
            if args.len() != 1 { return Err(MioscError::InvalidMessage) }

            let reference = pop_float(&mut args)?;

            Ok(MioscMessage::Reference(reference))
        },
        "/m/note_on" => {
            let mut args = args.ok_or(MioscError::InvalidMessage)?;
            if args.len() != 3 { return Err(MioscError::InvalidMessage) }

            let vel = pop_float(&mut args)?;
            let pitch = pop_float(&mut args)?;
            let id = pop_integer(&mut args)?;

            Ok(MioscMessage::NoteOn(id, pitch, vel))
        },
        "/m/note_off" => {
            let mut args = args.ok_or(MioscError::InvalidMessage)?;
            if args.len() != 1 { return Err(MioscError::InvalidMessage) }

            let id = pop_integer(&mut args)?;

            Ok(MioscMessage::NoteOff(id))
        },
        "/m/pitch" => {
            let mut args = args.ok_or(MioscError::InvalidMessage)?;
            if args.len() != 3 { return Err(MioscError::InvalidMessage) }

            let time = pop_float(&mut args)?;
            let pitch = pop_float(&mut args)?;
            let id = pop_integer(&mut args)?;

            Ok(MioscMessage::Pitch(id, pitch, time))
        },
        _ => Err(MioscError::UnknownMessage),
    }
}
