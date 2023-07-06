#![no_main]

use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use iso7816::command::{class, BufferFull, Command, CommandBuilder, CommandView};

use std::ops::Deref;

#[derive(Debug)]
struct WriteMock {
    buffer: [u8; 4096],
    written: usize,
    capacity: usize,
}

impl Deref for WriteMock {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.buffer[..self.written]
    }
}

impl iso7816::command::Writer for WriteMock {
    type Error = BufferFull;
    fn write(&mut self, data: &[u8]) -> Result<usize, BufferFull> {
        let available = self.capacity - self.written;
        let written = available.min(data.len());
        self.buffer[self.written..][..written].copy_from_slice(&data[..written]);
        self.written += written;
        if written == 0 {
            Err(BufferFull::BufferFull)
        } else {
            Ok(written)
        }
    }
}

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    class: u8,
    instruction: u8,
    p1: u8,
    p2: u8,
    le: u16,
    buf_len: usize,
    supports_extended: bool,
    data: &'a [u8],
}

fuzz_target!(|data: Input| {
    let Input {
        class,
        instruction,
        p1,
        p2,
        mut le,
        mut buf_len,
        supports_extended,
        data,
    } = data;
    if class == 0b11101111 {
        // pathological class that can't be chained because it makes it a 0xFF
        return;
    }
    let Ok(class) = class::Class::try_from(class) else {
        return;
    };

    if !supports_extended {
        le = le.min(255);
    }

    buf_len = buf_len.min(4096).max(128);

    let ins = instruction.into();

    let command = CommandBuilder::new(class, ins, p1, p2, data, le);
    // Test for the length information
    {
        command.clone().serialize_to_vec();
        for command in CommandBuilder::new_non_extended(class, ins, p1, p2, data, le, None) {
            command.serialize_to_vec();
        }
    }

    let mut buffer = WriteMock {
        buffer: [0; 4096],
        written: 0,
        capacity: buf_len,
    };

    if !supports_extended {
        let mut acc: Option<Command<4096>> = None;
        let mut iter =
            CommandBuilder::new_non_extended(class, ins, p1, p2, data, le, Some(buf_len))
                .peekable();
        while let Some(cmd) = iter.next() {
            buffer.written = 0;
            let (cla, le) = if iter.peek().is_some() {
                (class.as_chained(), 0)
            } else {
                (class, le.min(256))
            };
            let reference_command = CommandBuilder::new(cla, ins, p1, p2, cmd.data(), le);
            cmd.serialize_into(&mut buffer).unwrap();
            let view = CommandView::try_from(&*buffer).unwrap();
            assert!(view.data().len() <= 256);
            assert!(!view.extended());
            // Without extended support, le is truncated to 256 at max, and the response will come with command chaining
            assert_eq!(view, reference_command);

            if let Some(partial) = &mut acc {
                partial.extend_from_command_view(view).unwrap();
            } else {
                acc = Some(view.to_owned().unwrap());
            }
        }
        assert_eq!(acc.unwrap().as_view(), command);
    } else {
        match command.should_split(buffer.capacity) {
            None => {
                command.clone().serialize_into(&mut buffer).unwrap();
                let view = CommandView::try_from(&*buffer).unwrap();
                assert_eq!(view, command);
            }
            Some((current_command, mut remaining_command)) => {
                current_command.clone().serialize_into(&mut buffer).unwrap();
                let mut parsed_command: iso7816::Command<4096> =
                    Command::try_from(&buffer).unwrap();
                assert_eq!(parsed_command.as_view(), current_command);

                loop {
                    let mut buffer = WriteMock {
                        buffer: [0; 4096],
                        written: 0,
                        capacity: buf_len,
                    };

                    let Some((left, rem)) = remaining_command.should_split(buf_len) else {
                        remaining_command.clone().serialize_into(&mut buffer).unwrap();

                        let view = CommandView::try_from(&*buffer).unwrap();
                        assert_eq!(view, remaining_command);
                        parsed_command.extend_from_command_view(view).unwrap();
                        let view = parsed_command.as_view();
                        assert_eq!(view, command);
                        break;
                    };
                    remaining_command = rem;

                    left.clone().serialize_into(&mut buffer).unwrap();
                    let view = CommandView::try_from(&*buffer).unwrap();
                    assert_eq!(view, left);

                    parsed_command.extend_from_command_view(view).unwrap();
                }
            }
        }
    }
});
