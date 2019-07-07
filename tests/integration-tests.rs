extern crate architecture;

#[cfg(test)]
mod tests {
    use crate::architecture::condition_codes::ConditionCodes;
    use architecture::units::arithmetic_logic_unit::*;

    #[test]
    fn test_add_happy_path() {
        let mut cc = ConditionCodes{
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            ac: 0,
            pad: 0
        };

        let bytes = vec![0x00,0x00];

        let result = add(bytes,&mut cc);

        assert_eq!(result, 0x00);
    }
}