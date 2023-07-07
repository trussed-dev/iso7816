#![no_main]

use libfuzzer_sys::fuzz_target;

use arbitrary::Arbitrary;
use iso7816::command::{class, CommandBuilder, CommandView};

#[derive(Debug, Arbitrary)]
struct Input<'a> {
    class_1: u8,
    instruction_1: u8,
    p1_1: u8,
    p2_1: u8,
    le_1: u16,
    class_2: u8,
    instruction_2: u8,
    p1_2: u8,
    p2_2: u8,
    le_2: u16,
    data_1: &'a [u8],
}

fuzz_target!(|data: Input| {
    let Input {
        class_1,
        instruction_1,
        p1_1,
        p2_1,
        le_1,
        data_1,
        class_2,
        instruction_2,
        p1_2,
        p2_2,
        le_2,
    } = data;
    let Ok(class_1) = class::Class::try_from(class_1) else {
        return;
    };
    let Ok(class_2) = class::Class::try_from(class_2) else {
        return;
    };

    let inner = CommandBuilder::new(class_1, instruction_1.into(), p1_1, p2_1, data_1, le_1);
    let outer = CommandBuilder::new(class_2, instruction_2.into(), p1_2, p2_2, &inner, le_2);
    let res = outer.serialize_to_vec();
    let view = CommandView::try_from(&*res).unwrap();
    let inner_view = CommandView::try_from(view.data()).unwrap();
    assert_eq!(inner_view, inner);

    let outer_ref =
        CommandBuilder::new(class_2, instruction_2.into(), p1_2, p2_2, view.data(), le_2);
    assert_eq!(view, outer_ref);
});
