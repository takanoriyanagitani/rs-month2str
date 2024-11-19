pub const INVALID: u16 = 0x2020;

static MONTH_TO_STR: [u16; 16] = [
    INVALID, // 00
    0x3031, 0x3032, 0x3033, 0x3034, 0x3035, 0x3036, 0x3037, 0x3038, 0x3039, // 01-09
    0x3130, 0x3131, 0x3132, // 10,11,12
    INVALID, INVALID, INVALID, // 13,14,15
];

pub fn month_to_string(month: u8) -> u16 {
    let ix: u8 = month & 0x0f;
    MONTH_TO_STR[ix as usize]
}

#[cfg(target_family = "wasm")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mon2str(month: u8) -> u16 {
    month_to_string(month)
}

#[cfg(test)]
mod test_month2str {

    mod month_to_string {
        macro_rules! create_test_month2str {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let input: u8 = $input;
                    let expected: u16 = $expected;

                    let got: u16 = crate::month_to_string(input);

                    assert_eq!(got, expected);
                }
            };
        }

        create_test_month2str!(test01, 1, 0x3031);
        create_test_month2str!(test02, 2, 0x3032);
        create_test_month2str!(test03, 3, 0x3033);
        create_test_month2str!(test04, 4, 0x3034);
        create_test_month2str!(test05, 5, 0x3035);
        create_test_month2str!(test06, 6, 0x3036);
        create_test_month2str!(test07, 7, 0x3037);
        create_test_month2str!(test08, 8, 0x3038);
        create_test_month2str!(test09, 9, 0x3039);

        create_test_month2str!(test10, 10, 0x3130);
        create_test_month2str!(test11, 11, 0x3131);
        create_test_month2str!(test12, 12, 0x3132);
    }
}
