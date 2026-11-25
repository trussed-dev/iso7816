#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Interface {
    Contact,
    Contactless,
}

pub type Data<const S: usize> = heapless::Vec<u8, S>;
pub type Result<T = ()> = core::result::Result<T, Status>;

pub mod aid;
pub mod command;
pub mod response;

pub use aid::{Aid, App};
pub use command::{Command, Instruction};
pub use response::{Response, Status};
pub mod tlv;

#[cfg(test)]
mod tests {
    use super::Command;
    use proptest::prelude::*;

    const COMMAND_SIZE: usize = 7609;

    proptest! {
        #[test]
        fn parse_no_panic(data in proptest::collection::vec(any::<u8>(), ..COMMAND_SIZE)) {
            let _command = Command::<COMMAND_SIZE>::try_from(&data);
        }

        #[test]
        fn parse_apdu(
            cla: u8,
            ins: u8,
            p1: u8,
            p2: u8,
            data in proptest::collection::vec(any::<u8>(), ..u8::MAX as usize) ,
            le: Option<u8>,
        ) {
            let cla = if cla == u8::MAX { 0 } else { cla };

            let mut command = vec![cla, ins, p1, p2];
            if !data.is_empty() {
                command.push(data.len() as u8);
                command.extend_from_slice(&data);
            }
            if let Some(le) = le {
                command.push(le);
            }

            let command = Command::<COMMAND_SIZE>::try_from(&command).expect("unexpected error");
            assert_eq!(command.class().into_inner(), cla);
            assert_eq!(u8::from(command.instruction()), ins);
            assert_eq!(command.p1, p1);
            assert_eq!(command.p2, p2);
            assert!(!command.extended);
            assert_eq!(command.data().as_slice(), &data);
            assert_eq!(
                command.expected(),
                le.map(usize::from)
                    .map(|val| if val == 0 { 256 } else { val })
                    .unwrap_or_default()
            );
        }
    }
}
