#[cfg(test)]
mod tests {
    use crate::z80::{FDEPhase, Z80, Z80IO};
    use serde::Deserialize;
    use std::fs::{metadata, File};
    use std::io::Read;

    #[derive(Deserialize)]
    struct Z80State {
        pc: u16,
        sp: u16,
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        h: u8,
        l: u8,
        i: u8,
        r: u8,
        ei: u8,
        wz: u16,
        ix: u16,
        iy: u16,
        af_: u16,
        bc_: u16,
        de_: u16,
        hl_: u16,
        im: u8,
        p: u8,
        q: u8,
        iff1: u8,
        iff2: u8,
        ram: Vec<(u16, u8)>,
    }

    #[derive(Deserialize)]
    struct Z80Test {
        name: String,
        initial: Z80State,
        r#final: Z80State,
        cycles: Vec<(Option<u16>, Option<u8>, String)>,
        ports: Option<Vec<(u16, u8, char)>>,
    }

    fn load_file(fname: String) -> Vec<Z80Test> {
        let mut file = File::open(fname).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        serde_json::from_str(&data).unwrap()
    }

    fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }

    #[test]
    fn test_opcode_00() {
        test_jsmoo_file("00")
    }
    #[test]
    fn test_opcode_01() {
        test_jsmoo_file("01")
    }
    #[test]
    fn test_opcode_02() {
        test_jsmoo_file("02")
    }
    #[test]
    fn test_opcode_03() {
        test_jsmoo_file("03")
    }
    #[test]
    fn test_opcode_04() {
        test_jsmoo_file("04")
    }
    #[test]
    fn test_opcode_05() {
        test_jsmoo_file("05")
    }
    #[test]
    fn test_opcode_06() {
        test_jsmoo_file("06")
    }
    #[test]
    fn test_opcode_07() {
        test_jsmoo_file("07")
    }
    #[test]
    fn test_opcode_08() {
        test_jsmoo_file("08")
    }
    #[test]
    fn test_opcode_09() {
        test_jsmoo_file("09")
    }
    #[test]
    fn test_opcode_0a() {
        test_jsmoo_file("0a")
    }
    #[test]
    fn test_opcode_0b() {
        test_jsmoo_file("0b")
    }
    #[test]
    fn test_opcode_0c() {
        test_jsmoo_file("0c")
    }
    #[test]
    fn test_opcode_0d() {
        test_jsmoo_file("0d")
    }
    #[test]
    fn test_opcode_0e() {
        test_jsmoo_file("0e")
    }
    #[test]
    fn test_opcode_0f() {
        test_jsmoo_file("0f")
    }
    #[test]
    fn test_opcode_10() {
        test_jsmoo_file("10")
    }
    #[test]
    fn test_opcode_11() {
        test_jsmoo_file("11")
    }
    #[test]
    fn test_opcode_12() {
        test_jsmoo_file("12")
    }
    #[test]
    fn test_opcode_13() {
        test_jsmoo_file("13")
    }
    #[test]
    fn test_opcode_14() {
        test_jsmoo_file("14")
    }
    #[test]
    fn test_opcode_15() {
        test_jsmoo_file("15")
    }
    #[test]
    fn test_opcode_16() {
        test_jsmoo_file("16")
    }
    #[test]
    fn test_opcode_17() {
        test_jsmoo_file("17")
    }
    #[test]
    fn test_opcode_18() {
        test_jsmoo_file("18")
    }
    #[test]
    fn test_opcode_19() {
        test_jsmoo_file("19")
    }
    #[test]
    fn test_opcode_1a() {
        test_jsmoo_file("1a")
    }
    #[test]
    fn test_opcode_1b() {
        test_jsmoo_file("1b")
    }
    #[test]
    fn test_opcode_1c() {
        test_jsmoo_file("1c")
    }
    #[test]
    fn test_opcode_1d() {
        test_jsmoo_file("1d")
    }
    #[test]
    fn test_opcode_1e() {
        test_jsmoo_file("1e")
    }
    #[test]
    fn test_opcode_1f() {
        test_jsmoo_file("1f")
    }
    #[test]
    fn test_opcode_20() {
        test_jsmoo_file("20")
    }
    #[test]
    fn test_opcode_21() {
        test_jsmoo_file("21")
    }
    #[test]
    fn test_opcode_22() {
        test_jsmoo_file("22")
    }
    #[test]
    fn test_opcode_23() {
        test_jsmoo_file("23")
    }
    #[test]
    fn test_opcode_24() {
        test_jsmoo_file("24")
    }
    #[test]
    fn test_opcode_25() {
        test_jsmoo_file("25")
    }
    #[test]
    fn test_opcode_26() {
        test_jsmoo_file("26")
    }
    #[test]
    fn test_opcode_27() {
        test_jsmoo_file("27")
    }
    #[test]
    fn test_opcode_28() {
        test_jsmoo_file("28")
    }
    #[test]
    fn test_opcode_29() {
        test_jsmoo_file("29")
    }
    #[test]
    fn test_opcode_2a() {
        test_jsmoo_file("2a")
    }
    #[test]
    fn test_opcode_2b() {
        test_jsmoo_file("2b")
    }
    #[test]
    fn test_opcode_2c() {
        test_jsmoo_file("2c")
    }
    #[test]
    fn test_opcode_2d() {
        test_jsmoo_file("2d")
    }
    #[test]
    fn test_opcode_2e() {
        test_jsmoo_file("2e")
    }
    #[test]
    fn test_opcode_2f() {
        test_jsmoo_file("2f")
    }
    #[test]
    fn test_opcode_30() {
        test_jsmoo_file("30")
    }
    #[test]
    fn test_opcode_31() {
        test_jsmoo_file("31")
    }
    #[test]
    fn test_opcode_32() {
        test_jsmoo_file("32")
    }
    #[test]
    fn test_opcode_33() {
        test_jsmoo_file("33")
    }
    #[test]
    fn test_opcode_34() {
        test_jsmoo_file("34")
    }
    #[test]
    fn test_opcode_35() {
        test_jsmoo_file("35")
    }
    #[test]
    fn test_opcode_36() {
        test_jsmoo_file("36")
    }
    #[test]
    fn test_opcode_37() {
        test_jsmoo_file("37")
    }
    #[test]
    fn test_opcode_38() {
        test_jsmoo_file("38")
    }
    #[test]
    fn test_opcode_39() {
        test_jsmoo_file("39")
    }
    #[test]
    fn test_opcode_3a() {
        test_jsmoo_file("3a")
    }
    #[test]
    fn test_opcode_3b() {
        test_jsmoo_file("3b")
    }
    #[test]
    fn test_opcode_3c() {
        test_jsmoo_file("3c")
    }
    #[test]
    fn test_opcode_3d() {
        test_jsmoo_file("3d")
    }
    #[test]
    fn test_opcode_3e() {
        test_jsmoo_file("3e")
    }
    #[test]
    fn test_opcode_3f() {
        test_jsmoo_file("3f")
    }
    #[test]
    fn test_opcode_40() {
        test_jsmoo_file("40")
    }
    #[test]
    fn test_opcode_41() {
        test_jsmoo_file("41")
    }
    #[test]
    fn test_opcode_42() {
        test_jsmoo_file("42")
    }
    #[test]
    fn test_opcode_43() {
        test_jsmoo_file("43")
    }
    #[test]
    fn test_opcode_44() {
        test_jsmoo_file("44")
    }
    #[test]
    fn test_opcode_45() {
        test_jsmoo_file("45")
    }
    #[test]
    fn test_opcode_46() {
        test_jsmoo_file("46")
    }
    #[test]
    fn test_opcode_47() {
        test_jsmoo_file("47")
    }
    #[test]
    fn test_opcode_48() {
        test_jsmoo_file("48")
    }
    #[test]
    fn test_opcode_49() {
        test_jsmoo_file("49")
    }
    #[test]
    fn test_opcode_4a() {
        test_jsmoo_file("4a")
    }
    #[test]
    fn test_opcode_4b() {
        test_jsmoo_file("4b")
    }
    #[test]
    fn test_opcode_4c() {
        test_jsmoo_file("4c")
    }
    #[test]
    fn test_opcode_4d() {
        test_jsmoo_file("4d")
    }
    #[test]
    fn test_opcode_4e() {
        test_jsmoo_file("4e")
    }
    #[test]
    fn test_opcode_4f() {
        test_jsmoo_file("4f")
    }
    #[test]
    fn test_opcode_50() {
        test_jsmoo_file("50")
    }
    #[test]
    fn test_opcode_51() {
        test_jsmoo_file("51")
    }
    #[test]
    fn test_opcode_52() {
        test_jsmoo_file("52")
    }
    #[test]
    fn test_opcode_53() {
        test_jsmoo_file("53")
    }
    #[test]
    fn test_opcode_54() {
        test_jsmoo_file("54")
    }
    #[test]
    fn test_opcode_55() {
        test_jsmoo_file("55")
    }
    #[test]
    fn test_opcode_56() {
        test_jsmoo_file("56")
    }
    #[test]
    fn test_opcode_57() {
        test_jsmoo_file("57")
    }
    #[test]
    fn test_opcode_58() {
        test_jsmoo_file("58")
    }
    #[test]
    fn test_opcode_59() {
        test_jsmoo_file("59")
    }
    #[test]
    fn test_opcode_5a() {
        test_jsmoo_file("5a")
    }
    #[test]
    fn test_opcode_5b() {
        test_jsmoo_file("5b")
    }
    #[test]
    fn test_opcode_5c() {
        test_jsmoo_file("5c")
    }
    #[test]
    fn test_opcode_5d() {
        test_jsmoo_file("5d")
    }
    #[test]
    fn test_opcode_5e() {
        test_jsmoo_file("5e")
    }
    #[test]
    fn test_opcode_5f() {
        test_jsmoo_file("5f")
    }
    #[test]
    fn test_opcode_60() {
        test_jsmoo_file("60")
    }
    #[test]
    fn test_opcode_61() {
        test_jsmoo_file("61")
    }
    #[test]
    fn test_opcode_62() {
        test_jsmoo_file("62")
    }
    #[test]
    fn test_opcode_63() {
        test_jsmoo_file("63")
    }
    #[test]
    fn test_opcode_64() {
        test_jsmoo_file("64")
    }
    #[test]
    fn test_opcode_65() {
        test_jsmoo_file("65")
    }
    #[test]
    fn test_opcode_66() {
        test_jsmoo_file("66")
    }
    #[test]
    fn test_opcode_67() {
        test_jsmoo_file("67")
    }
    #[test]
    fn test_opcode_68() {
        test_jsmoo_file("68")
    }
    #[test]
    fn test_opcode_69() {
        test_jsmoo_file("69")
    }
    #[test]
    fn test_opcode_6a() {
        test_jsmoo_file("6a")
    }
    #[test]
    fn test_opcode_6b() {
        test_jsmoo_file("6b")
    }
    #[test]
    fn test_opcode_6c() {
        test_jsmoo_file("6c")
    }
    #[test]
    fn test_opcode_6d() {
        test_jsmoo_file("6d")
    }
    #[test]
    fn test_opcode_6e() {
        test_jsmoo_file("6e")
    }
    #[test]
    fn test_opcode_6f() {
        test_jsmoo_file("6f")
    }
    #[test]
    fn test_opcode_70() {
        test_jsmoo_file("70")
    }
    #[test]
    fn test_opcode_71() {
        test_jsmoo_file("71")
    }
    #[test]
    fn test_opcode_72() {
        test_jsmoo_file("72")
    }
    #[test]
    fn test_opcode_73() {
        test_jsmoo_file("73")
    }
    #[test]
    fn test_opcode_74() {
        test_jsmoo_file("74")
    }
    #[test]
    fn test_opcode_75() {
        test_jsmoo_file("75")
    }
    #[test]
    fn test_opcode_76() {
        test_jsmoo_file("76")
    }
    #[test]
    fn test_opcode_77() {
        test_jsmoo_file("77")
    }
    #[test]
    fn test_opcode_78() {
        test_jsmoo_file("78")
    }
    #[test]
    fn test_opcode_79() {
        test_jsmoo_file("79")
    }
    #[test]
    fn test_opcode_7a() {
        test_jsmoo_file("7a")
    }
    #[test]
    fn test_opcode_7b() {
        test_jsmoo_file("7b")
    }
    #[test]
    fn test_opcode_7c() {
        test_jsmoo_file("7c")
    }
    #[test]
    fn test_opcode_7d() {
        test_jsmoo_file("7d")
    }
    #[test]
    fn test_opcode_7e() {
        test_jsmoo_file("7e")
    }
    #[test]
    fn test_opcode_7f() {
        test_jsmoo_file("7f")
    }
    #[test]
    fn test_opcode_80() {
        test_jsmoo_file("80")
    }
    #[test]
    fn test_opcode_81() {
        test_jsmoo_file("81")
    }
    #[test]
    fn test_opcode_82() {
        test_jsmoo_file("82")
    }
    #[test]
    fn test_opcode_83() {
        test_jsmoo_file("83")
    }
    #[test]
    fn test_opcode_84() {
        test_jsmoo_file("84")
    }
    #[test]
    fn test_opcode_85() {
        test_jsmoo_file("85")
    }
    #[test]
    fn test_opcode_86() {
        test_jsmoo_file("86")
    }
    #[test]
    fn test_opcode_87() {
        test_jsmoo_file("87")
    }
    #[test]
    fn test_opcode_88() {
        test_jsmoo_file("88")
    }
    #[test]
    fn test_opcode_89() {
        test_jsmoo_file("89")
    }
    #[test]
    fn test_opcode_8a() {
        test_jsmoo_file("8a")
    }
    #[test]
    fn test_opcode_8b() {
        test_jsmoo_file("8b")
    }
    #[test]
    fn test_opcode_8c() {
        test_jsmoo_file("8c")
    }
    #[test]
    fn test_opcode_8d() {
        test_jsmoo_file("8d")
    }
    #[test]
    fn test_opcode_8e() {
        test_jsmoo_file("8e")
    }
    #[test]
    fn test_opcode_8f() {
        test_jsmoo_file("8f")
    }
    #[test]
    fn test_opcode_90() {
        test_jsmoo_file("90")
    }
    #[test]
    fn test_opcode_91() {
        test_jsmoo_file("91")
    }
    #[test]
    fn test_opcode_92() {
        test_jsmoo_file("92")
    }
    #[test]
    fn test_opcode_93() {
        test_jsmoo_file("93")
    }
    #[test]
    fn test_opcode_94() {
        test_jsmoo_file("94")
    }
    #[test]
    fn test_opcode_95() {
        test_jsmoo_file("95")
    }
    #[test]
    fn test_opcode_96() {
        test_jsmoo_file("96")
    }
    #[test]
    fn test_opcode_97() {
        test_jsmoo_file("97")
    }
    #[test]
    fn test_opcode_98() {
        test_jsmoo_file("98")
    }
    #[test]
    fn test_opcode_99() {
        test_jsmoo_file("99")
    }
    #[test]
    fn test_opcode_9a() {
        test_jsmoo_file("9a")
    }
    #[test]
    fn test_opcode_9b() {
        test_jsmoo_file("9b")
    }
    #[test]
    fn test_opcode_9c() {
        test_jsmoo_file("9c")
    }
    #[test]
    fn test_opcode_9d() {
        test_jsmoo_file("9d")
    }
    #[test]
    fn test_opcode_9e() {
        test_jsmoo_file("9e")
    }
    #[test]
    fn test_opcode_9f() {
        test_jsmoo_file("9f")
    }
    #[test]
    fn test_opcode_a0() {
        test_jsmoo_file("a0")
    }
    #[test]
    fn test_opcode_a1() {
        test_jsmoo_file("a1")
    }
    #[test]
    fn test_opcode_a2() {
        test_jsmoo_file("a2")
    }
    #[test]
    fn test_opcode_a3() {
        test_jsmoo_file("a3")
    }
    #[test]
    fn test_opcode_a4() {
        test_jsmoo_file("a4")
    }
    #[test]
    fn test_opcode_a5() {
        test_jsmoo_file("a5")
    }
    #[test]
    fn test_opcode_a6() {
        test_jsmoo_file("a6")
    }
    #[test]
    fn test_opcode_a7() {
        test_jsmoo_file("a7")
    }
    #[test]
    fn test_opcode_a8() {
        test_jsmoo_file("a8")
    }
    #[test]
    fn test_opcode_a9() {
        test_jsmoo_file("a9")
    }
    #[test]
    fn test_opcode_aa() {
        test_jsmoo_file("aa")
    }
    #[test]
    fn test_opcode_ab() {
        test_jsmoo_file("ab")
    }
    #[test]
    fn test_opcode_ac() {
        test_jsmoo_file("ac")
    }
    #[test]
    fn test_opcode_ad() {
        test_jsmoo_file("ad")
    }
    #[test]
    fn test_opcode_ae() {
        test_jsmoo_file("ae")
    }
    #[test]
    fn test_opcode_af() {
        test_jsmoo_file("af")
    }
    #[test]
    fn test_opcode_b0() {
        test_jsmoo_file("b0")
    }
    #[test]
    fn test_opcode_b1() {
        test_jsmoo_file("b1")
    }
    #[test]
    fn test_opcode_b2() {
        test_jsmoo_file("b2")
    }
    #[test]
    fn test_opcode_b3() {
        test_jsmoo_file("b3")
    }
    #[test]
    fn test_opcode_b4() {
        test_jsmoo_file("b4")
    }
    #[test]
    fn test_opcode_b5() {
        test_jsmoo_file("b5")
    }
    #[test]
    fn test_opcode_b6() {
        test_jsmoo_file("b6")
    }
    #[test]
    fn test_opcode_b7() {
        test_jsmoo_file("b7")
    }
    #[test]
    fn test_opcode_b8() {
        test_jsmoo_file("b8")
    }
    #[test]
    fn test_opcode_b9() {
        test_jsmoo_file("b9")
    }
    #[test]
    fn test_opcode_ba() {
        test_jsmoo_file("ba")
    }
    #[test]
    fn test_opcode_bb() {
        test_jsmoo_file("bb")
    }
    #[test]
    fn test_opcode_bc() {
        test_jsmoo_file("bc")
    }
    #[test]
    fn test_opcode_bd() {
        test_jsmoo_file("bd")
    }
    #[test]
    fn test_opcode_be() {
        test_jsmoo_file("be")
    }
    #[test]
    fn test_opcode_bf() {
        test_jsmoo_file("bf")
    }
    #[test]
    fn test_opcode_c0() {
        test_jsmoo_file("c0")
    }
    #[test]
    fn test_opcode_c1() {
        test_jsmoo_file("c1")
    }
    #[test]
    fn test_opcode_c2() {
        test_jsmoo_file("c2")
    }
    #[test]
    fn test_opcode_c3() {
        test_jsmoo_file("c3")
    }
    #[test]
    fn test_opcode_c4() {
        test_jsmoo_file("c4")
    }
    #[test]
    fn test_opcode_c5() {
        test_jsmoo_file("c5")
    }
    #[test]
    fn test_opcode_c6() {
        test_jsmoo_file("c6")
    }
    #[test]
    fn test_opcode_c7() {
        test_jsmoo_file("c7")
    }
    #[test]
    fn test_opcode_c8() {
        test_jsmoo_file("c8")
    }
    #[test]
    fn test_opcode_c9() {
        test_jsmoo_file("c9")
    }
    #[test]
    fn test_opcode_ca() {
        test_jsmoo_file("ca")
    }
    #[test]
    fn test_opcode_cc() {
        test_jsmoo_file("cc")
    }
    #[test]
    fn test_opcode_cd() {
        test_jsmoo_file("cd")
    }
    #[test]
    fn test_opcode_ce() {
        test_jsmoo_file("ce")
    }
    #[test]
    fn test_opcode_cf() {
        test_jsmoo_file("cf")
    }
    #[test]
    fn test_opcode_d0() {
        test_jsmoo_file("d0")
    }
    #[test]
    fn test_opcode_d1() {
        test_jsmoo_file("d1")
    }
    #[test]
    fn test_opcode_d2() {
        test_jsmoo_file("d2")
    }
    #[test]
    fn test_opcode_d3() {
        test_jsmoo_file("d3")
    }
    #[test]
    fn test_opcode_d4() {
        test_jsmoo_file("d4")
    }
    #[test]
    fn test_opcode_d5() {
        test_jsmoo_file("d5")
    }
    #[test]
    fn test_opcode_d6() {
        test_jsmoo_file("d6")
    }
    #[test]
    fn test_opcode_d7() {
        test_jsmoo_file("d7")
    }
    #[test]
    fn test_opcode_d8() {
        test_jsmoo_file("d8")
    }
    #[test]
    fn test_opcode_d9() {
        test_jsmoo_file("d9")
    }
    #[test]
    fn test_opcode_da() {
        test_jsmoo_file("da")
    }
    #[test]
    fn test_opcode_db() {
        test_jsmoo_file("db")
    }
    #[test]
    fn test_opcode_dc() {
        test_jsmoo_file("dc")
    }
    #[test]
    fn test_opcode_de() {
        test_jsmoo_file("de")
    }
    #[test]
    fn test_opcode_df() {
        test_jsmoo_file("df")
    }
    #[test]
    fn test_opcode_e0() {
        test_jsmoo_file("e0")
    }
    #[test]
    fn test_opcode_e1() {
        test_jsmoo_file("e1")
    }
    #[test]
    fn test_opcode_e2() {
        test_jsmoo_file("e2")
    }
    #[test]
    fn test_opcode_e3() {
        test_jsmoo_file("e3")
    }
    #[test]
    fn test_opcode_e4() {
        test_jsmoo_file("e4")
    }
    #[test]
    fn test_opcode_e5() {
        test_jsmoo_file("e5")
    }
    #[test]
    fn test_opcode_e6() {
        test_jsmoo_file("e6")
    }
    #[test]
    fn test_opcode_e7() {
        test_jsmoo_file("e7")
    }
    #[test]
    fn test_opcode_e8() {
        test_jsmoo_file("e8")
    }
    #[test]
    fn test_opcode_e9() {
        test_jsmoo_file("e9")
    }
    #[test]
    fn test_opcode_ea() {
        test_jsmoo_file("ea")
    }
    #[test]
    fn test_opcode_eb() {
        test_jsmoo_file("eb")
    }
    #[test]
    fn test_opcode_ec() {
        test_jsmoo_file("ec")
    }
    #[test]
    fn test_opcode_ee() {
        test_jsmoo_file("ee")
    }
    #[test]
    fn test_opcode_ef() {
        test_jsmoo_file("ef")
    }
    #[test]
    fn test_opcode_f0() {
        test_jsmoo_file("f0")
    }
    #[test]
    fn test_opcode_f1() {
        test_jsmoo_file("f1")
    }
    #[test]
    fn test_opcode_f2() {
        test_jsmoo_file("f2")
    }
    #[test]
    fn test_opcode_f3() {
        test_jsmoo_file("f3")
    }
    #[test]
    fn test_opcode_f4() {
        test_jsmoo_file("f4")
    }
    #[test]
    fn test_opcode_f5() {
        test_jsmoo_file("f5")
    }
    #[test]
    fn test_opcode_f6() {
        test_jsmoo_file("f6")
    }
    #[test]
    fn test_opcode_f7() {
        test_jsmoo_file("f7")
    }
    #[test]
    fn test_opcode_f8() {
        test_jsmoo_file("f8")
    }
    #[test]
    fn test_opcode_f9() {
        test_jsmoo_file("f9")
    }
    #[test]
    fn test_opcode_fa() {
        test_jsmoo_file("fa")
    }
    #[test]
    fn test_opcode_fb() {
        test_jsmoo_file("fb")
    }
    #[test]
    fn test_opcode_fc() {
        test_jsmoo_file("fc")
    }
    #[test]
    fn test_opcode_fe() {
        test_jsmoo_file("fe")
    }
    #[test]
    fn test_opcode_ff() {
        test_jsmoo_file("ff")
    }
    #[test]
    fn test_opcode_100() {
        test_jsmoo_file("100")
    }
    #[test]
    fn test_opcode_101() {
        test_jsmoo_file("101")
    }

    #[test]
    fn test_opcode_cb00() {
        test_jsmoo_file("cb 00")
    }
    #[test]
    fn test_opcode_cb01() {
        test_jsmoo_file("cb 01")
    }
    #[test]
    fn test_opcode_cb02() {
        test_jsmoo_file("cb 02")
    }
    #[test]
    fn test_opcode_cb03() {
        test_jsmoo_file("cb 03")
    }
    #[test]
    fn test_opcode_cb04() {
        test_jsmoo_file("cb 04")
    }
    #[test]
    fn test_opcode_cb05() {
        test_jsmoo_file("cb 05")
    }
    #[test]
    fn test_opcode_cb06() {
        test_jsmoo_file("cb 06")
    }
    #[test]
    fn test_opcode_cb07() {
        test_jsmoo_file("cb 07")
    }
    #[test]
    fn test_opcode_cb08() {
        test_jsmoo_file("cb 08")
    }
    #[test]
    fn test_opcode_cb09() {
        test_jsmoo_file("cb 09")
    }
    #[test]
    fn test_opcode_cb0a() {
        test_jsmoo_file("cb 0a")
    }
    #[test]
    fn test_opcode_cb0b() {
        test_jsmoo_file("cb 0b")
    }
    #[test]
    fn test_opcode_cb0c() {
        test_jsmoo_file("cb 0c")
    }
    #[test]
    fn test_opcode_cb0d() {
        test_jsmoo_file("cb 0d")
    }
    #[test]
    fn test_opcode_cb0e() {
        test_jsmoo_file("cb 0e")
    }
    #[test]
    fn test_opcode_cb0f() {
        test_jsmoo_file("cb 0f")
    }
    #[test]
    fn test_opcode_cb10() {
        test_jsmoo_file("cb 10")
    }
    #[test]
    fn test_opcode_cb11() {
        test_jsmoo_file("cb 11")
    }
    #[test]
    fn test_opcode_cb12() {
        test_jsmoo_file("cb 12")
    }
    #[test]
    fn test_opcode_cb13() {
        test_jsmoo_file("cb 13")
    }
    #[test]
    fn test_opcode_cb14() {
        test_jsmoo_file("cb 14")
    }
    #[test]
    fn test_opcode_cb15() {
        test_jsmoo_file("cb 15")
    }
    #[test]
    fn test_opcode_cb16() {
        test_jsmoo_file("cb 16")
    }
    #[test]
    fn test_opcode_cb17() {
        test_jsmoo_file("cb 17")
    }
    #[test]
    fn test_opcode_cb18() {
        test_jsmoo_file("cb 18")
    }
    #[test]
    fn test_opcode_cb19() {
        test_jsmoo_file("cb 19")
    }
    #[test]
    fn test_opcode_cb1a() {
        test_jsmoo_file("cb 1a")
    }
    #[test]
    fn test_opcode_cb1b() {
        test_jsmoo_file("cb 1b")
    }
    #[test]
    fn test_opcode_cb1c() {
        test_jsmoo_file("cb 1c")
    }
    #[test]
    fn test_opcode_cb1d() {
        test_jsmoo_file("cb 1d")
    }
    #[test]
    fn test_opcode_cb1e() {
        test_jsmoo_file("cb 1e")
    }
    #[test]
    fn test_opcode_cb1f() {
        test_jsmoo_file("cb 1f")
    }
    #[test]
    fn test_opcode_cb20() {
        test_jsmoo_file("cb 20")
    }
    #[test]
    fn test_opcode_cb21() {
        test_jsmoo_file("cb 21")
    }
    #[test]
    fn test_opcode_cb22() {
        test_jsmoo_file("cb 22")
    }
    #[test]
    fn test_opcode_cb23() {
        test_jsmoo_file("cb 23")
    }
    #[test]
    fn test_opcode_cb24() {
        test_jsmoo_file("cb 24")
    }
    #[test]
    fn test_opcode_cb25() {
        test_jsmoo_file("cb 25")
    }
    #[test]
    fn test_opcode_cb26() {
        test_jsmoo_file("cb 26")
    }
    #[test]
    fn test_opcode_cb27() {
        test_jsmoo_file("cb 27")
    }
    #[test]
    fn test_opcode_cb28() {
        test_jsmoo_file("cb 28")
    }
    #[test]
    fn test_opcode_cb29() {
        test_jsmoo_file("cb 29")
    }
    #[test]
    fn test_opcode_cb2a() {
        test_jsmoo_file("cb 2a")
    }
    #[test]
    fn test_opcode_cb2b() {
        test_jsmoo_file("cb 2b")
    }
    #[test]
    fn test_opcode_cb2c() {
        test_jsmoo_file("cb 2c")
    }
    #[test]
    fn test_opcode_cb2d() {
        test_jsmoo_file("cb 2d")
    }
    #[test]
    fn test_opcode_cb2e() {
        test_jsmoo_file("cb 2e")
    }
    #[test]
    fn test_opcode_cb2f() {
        test_jsmoo_file("cb 2f")
    }
    #[test]
    fn test_opcode_cb30() {
        test_jsmoo_file("cb 30")
    }
    #[test]
    fn test_opcode_cb31() {
        test_jsmoo_file("cb 31")
    }
    #[test]
    fn test_opcode_cb32() {
        test_jsmoo_file("cb 32")
    }
    #[test]
    fn test_opcode_cb33() {
        test_jsmoo_file("cb 33")
    }
    #[test]
    fn test_opcode_cb34() {
        test_jsmoo_file("cb 34")
    }
    #[test]
    fn test_opcode_cb35() {
        test_jsmoo_file("cb 35")
    }
    #[test]
    fn test_opcode_cb36() {
        test_jsmoo_file("cb 36")
    }
    #[test]
    fn test_opcode_cb37() {
        test_jsmoo_file("cb 37")
    }
    #[test]
    fn test_opcode_cb38() {
        test_jsmoo_file("cb 38")
    }
    #[test]
    fn test_opcode_cb39() {
        test_jsmoo_file("cb 39")
    }
    #[test]
    fn test_opcode_cb3a() {
        test_jsmoo_file("cb 3a")
    }
    #[test]
    fn test_opcode_cb3b() {
        test_jsmoo_file("cb 3b")
    }
    #[test]
    fn test_opcode_cb3c() {
        test_jsmoo_file("cb 3c")
    }
    #[test]
    fn test_opcode_cb3d() {
        test_jsmoo_file("cb 3d")
    }
    #[test]
    fn test_opcode_cb3e() {
        test_jsmoo_file("cb 3e")
    }
    #[test]
    fn test_opcode_cb3f() {
        test_jsmoo_file("cb 3f")
    }
    #[test]
    fn test_opcode_cb40() {
        test_jsmoo_file("cb 40")
    }
    #[test]
    fn test_opcode_cb41() {
        test_jsmoo_file("cb 41")
    }
    #[test]
    fn test_opcode_cb42() {
        test_jsmoo_file("cb 42")
    }
    #[test]
    fn test_opcode_cb43() {
        test_jsmoo_file("cb 43")
    }
    #[test]
    fn test_opcode_cb44() {
        test_jsmoo_file("cb 44")
    }
    #[test]
    fn test_opcode_cb45() {
        test_jsmoo_file("cb 45")
    }
    #[test]
    fn test_opcode_cb46() {
        test_jsmoo_file("cb 46")
    }
    #[test]
    fn test_opcode_cb47() {
        test_jsmoo_file("cb 47")
    }
    #[test]
    fn test_opcode_cb48() {
        test_jsmoo_file("cb 48")
    }
    #[test]
    fn test_opcode_cb49() {
        test_jsmoo_file("cb 49")
    }
    #[test]
    fn test_opcode_cb4a() {
        test_jsmoo_file("cb 4a")
    }
    #[test]
    fn test_opcode_cb4b() {
        test_jsmoo_file("cb 4b")
    }
    #[test]
    fn test_opcode_cb4c() {
        test_jsmoo_file("cb 4c")
    }
    #[test]
    fn test_opcode_cb4d() {
        test_jsmoo_file("cb 4d")
    }
    #[test]
    fn test_opcode_cb4e() {
        test_jsmoo_file("cb 4e")
    }
    #[test]
    fn test_opcode_cb4f() {
        test_jsmoo_file("cb 4f")
    }
    #[test]
    fn test_opcode_cb50() {
        test_jsmoo_file("cb 50")
    }
    #[test]
    fn test_opcode_cb51() {
        test_jsmoo_file("cb 51")
    }
    #[test]
    fn test_opcode_cb52() {
        test_jsmoo_file("cb 52")
    }
    #[test]
    fn test_opcode_cb53() {
        test_jsmoo_file("cb 53")
    }
    #[test]
    fn test_opcode_cb54() {
        test_jsmoo_file("cb 54")
    }
    #[test]
    fn test_opcode_cb55() {
        test_jsmoo_file("cb 55")
    }
    #[test]
    fn test_opcode_cb56() {
        test_jsmoo_file("cb 56")
    }
    #[test]
    fn test_opcode_cb57() {
        test_jsmoo_file("cb 57")
    }
    #[test]
    fn test_opcode_cb58() {
        test_jsmoo_file("cb 58")
    }
    #[test]
    fn test_opcode_cb59() {
        test_jsmoo_file("cb 59")
    }
    #[test]
    fn test_opcode_cb5a() {
        test_jsmoo_file("cb 5a")
    }
    #[test]
    fn test_opcode_cb5b() {
        test_jsmoo_file("cb 5b")
    }
    #[test]
    fn test_opcode_cb5c() {
        test_jsmoo_file("cb 5c")
    }
    #[test]
    fn test_opcode_cb5d() {
        test_jsmoo_file("cb 5d")
    }
    #[test]
    fn test_opcode_cb5e() {
        test_jsmoo_file("cb 5e")
    }
    #[test]
    fn test_opcode_cb5f() {
        test_jsmoo_file("cb 5f")
    }
    #[test]
    fn test_opcode_cb60() {
        test_jsmoo_file("cb 60")
    }
    #[test]
    fn test_opcode_cb61() {
        test_jsmoo_file("cb 61")
    }
    #[test]
    fn test_opcode_cb62() {
        test_jsmoo_file("cb 62")
    }
    #[test]
    fn test_opcode_cb63() {
        test_jsmoo_file("cb 63")
    }
    #[test]
    fn test_opcode_cb64() {
        test_jsmoo_file("cb 64")
    }
    #[test]
    fn test_opcode_cb65() {
        test_jsmoo_file("cb 65")
    }
    #[test]
    fn test_opcode_cb66() {
        test_jsmoo_file("cb 66")
    }
    #[test]
    fn test_opcode_cb67() {
        test_jsmoo_file("cb 67")
    }
    #[test]
    fn test_opcode_cb68() {
        test_jsmoo_file("cb 68")
    }
    #[test]
    fn test_opcode_cb69() {
        test_jsmoo_file("cb 69")
    }
    #[test]
    fn test_opcode_cb6a() {
        test_jsmoo_file("cb 6a")
    }
    #[test]
    fn test_opcode_cb6b() {
        test_jsmoo_file("cb 6b")
    }
    #[test]
    fn test_opcode_cb6c() {
        test_jsmoo_file("cb 6c")
    }
    #[test]
    fn test_opcode_cb6d() {
        test_jsmoo_file("cb 6d")
    }
    #[test]
    fn test_opcode_cb6e() {
        test_jsmoo_file("cb 6e")
    }
    #[test]
    fn test_opcode_cb6f() {
        test_jsmoo_file("cb 6f")
    }
    #[test]
    fn test_opcode_cb70() {
        test_jsmoo_file("cb 70")
    }
    #[test]
    fn test_opcode_cb71() {
        test_jsmoo_file("cb 71")
    }
    #[test]
    fn test_opcode_cb72() {
        test_jsmoo_file("cb 72")
    }
    #[test]
    fn test_opcode_cb73() {
        test_jsmoo_file("cb 73")
    }
    #[test]
    fn test_opcode_cb74() {
        test_jsmoo_file("cb 74")
    }
    #[test]
    fn test_opcode_cb75() {
        test_jsmoo_file("cb 75")
    }
    #[test]
    fn test_opcode_cb76() {
        test_jsmoo_file("cb 76")
    }
    #[test]
    fn test_opcode_cb77() {
        test_jsmoo_file("cb 77")
    }
    #[test]
    fn test_opcode_cb78() {
        test_jsmoo_file("cb 78")
    }
    #[test]
    fn test_opcode_cb79() {
        test_jsmoo_file("cb 79")
    }
    #[test]
    fn test_opcode_cb7a() {
        test_jsmoo_file("cb 7a")
    }
    #[test]
    fn test_opcode_cb7b() {
        test_jsmoo_file("cb 7b")
    }
    #[test]
    fn test_opcode_cb7c() {
        test_jsmoo_file("cb 7c")
    }
    #[test]
    fn test_opcode_cb7d() {
        test_jsmoo_file("cb 7d")
    }
    #[test]
    fn test_opcode_cb7e() {
        test_jsmoo_file("cb 7e")
    }
    #[test]
    fn test_opcode_cb7f() {
        test_jsmoo_file("cb 7f")
    }
    #[test]
    fn test_opcode_cb80() {
        test_jsmoo_file("cb 80")
    }
    #[test]
    fn test_opcode_cb81() {
        test_jsmoo_file("cb 81")
    }
    #[test]
    fn test_opcode_cb82() {
        test_jsmoo_file("cb 82")
    }
    #[test]
    fn test_opcode_cb83() {
        test_jsmoo_file("cb 83")
    }
    #[test]
    fn test_opcode_cb84() {
        test_jsmoo_file("cb 84")
    }
    #[test]
    fn test_opcode_cb85() {
        test_jsmoo_file("cb 85")
    }
    #[test]
    fn test_opcode_cb86() {
        test_jsmoo_file("cb 86")
    }
    #[test]
    fn test_opcode_cb87() {
        test_jsmoo_file("cb 87")
    }
    #[test]
    fn test_opcode_cb88() {
        test_jsmoo_file("cb 88")
    }
    #[test]
    fn test_opcode_cb89() {
        test_jsmoo_file("cb 89")
    }
    #[test]
    fn test_opcode_cb8a() {
        test_jsmoo_file("cb 8a")
    }
    #[test]
    fn test_opcode_cb8b() {
        test_jsmoo_file("cb 8b")
    }
    #[test]
    fn test_opcode_cb8c() {
        test_jsmoo_file("cb 8c")
    }
    #[test]
    fn test_opcode_cb8d() {
        test_jsmoo_file("cb 8d")
    }
    #[test]
    fn test_opcode_cb8e() {
        test_jsmoo_file("cb 8e")
    }
    #[test]
    fn test_opcode_cb8f() {
        test_jsmoo_file("cb 8f")
    }
    #[test]
    fn test_opcode_cb90() {
        test_jsmoo_file("cb 90")
    }
    #[test]
    fn test_opcode_cb91() {
        test_jsmoo_file("cb 91")
    }
    #[test]
    fn test_opcode_cb92() {
        test_jsmoo_file("cb 92")
    }
    #[test]
    fn test_opcode_cb93() {
        test_jsmoo_file("cb 93")
    }
    #[test]
    fn test_opcode_cb94() {
        test_jsmoo_file("cb 94")
    }
    #[test]
    fn test_opcode_cb95() {
        test_jsmoo_file("cb 95")
    }
    #[test]
    fn test_opcode_cb96() {
        test_jsmoo_file("cb 96")
    }
    #[test]
    fn test_opcode_cb97() {
        test_jsmoo_file("cb 97")
    }
    #[test]
    fn test_opcode_cb98() {
        test_jsmoo_file("cb 98")
    }
    #[test]
    fn test_opcode_cb99() {
        test_jsmoo_file("cb 99")
    }
    #[test]
    fn test_opcode_cb9a() {
        test_jsmoo_file("cb 9a")
    }
    #[test]
    fn test_opcode_cb9b() {
        test_jsmoo_file("cb 9b")
    }
    #[test]
    fn test_opcode_cb9c() {
        test_jsmoo_file("cb 9c")
    }
    #[test]
    fn test_opcode_cb9d() {
        test_jsmoo_file("cb 9d")
    }
    #[test]
    fn test_opcode_cb9e() {
        test_jsmoo_file("cb 9e")
    }
    #[test]
    fn test_opcode_cb9f() {
        test_jsmoo_file("cb 9f")
    }
    #[test]
    fn test_opcode_cba0() {
        test_jsmoo_file("cb a0")
    }
    #[test]
    fn test_opcode_cba1() {
        test_jsmoo_file("cb a1")
    }
    #[test]
    fn test_opcode_cba2() {
        test_jsmoo_file("cb a2")
    }
    #[test]
    fn test_opcode_cba3() {
        test_jsmoo_file("cb a3")
    }
    #[test]
    fn test_opcode_cba4() {
        test_jsmoo_file("cb a4")
    }
    #[test]
    fn test_opcode_cba5() {
        test_jsmoo_file("cb a5")
    }
    #[test]
    fn test_opcode_cba6() {
        test_jsmoo_file("cb a6")
    }
    #[test]
    fn test_opcode_cba7() {
        test_jsmoo_file("cb a7")
    }
    #[test]
    fn test_opcode_cba8() {
        test_jsmoo_file("cb a8")
    }
    #[test]
    fn test_opcode_cba9() {
        test_jsmoo_file("cb a9")
    }
    #[test]
    fn test_opcode_cbaa() {
        test_jsmoo_file("cb aa")
    }
    #[test]
    fn test_opcode_cbab() {
        test_jsmoo_file("cb ab")
    }
    #[test]
    fn test_opcode_cbac() {
        test_jsmoo_file("cb ac")
    }
    #[test]
    fn test_opcode_cbad() {
        test_jsmoo_file("cb ad")
    }
    #[test]
    fn test_opcode_cbae() {
        test_jsmoo_file("cb ae")
    }
    #[test]
    fn test_opcode_cbaf() {
        test_jsmoo_file("cb af")
    }
    #[test]
    fn test_opcode_cbb0() {
        test_jsmoo_file("cb b0")
    }
    #[test]
    fn test_opcode_cbb1() {
        test_jsmoo_file("cb b1")
    }
    #[test]
    fn test_opcode_cbb2() {
        test_jsmoo_file("cb b2")
    }
    #[test]
    fn test_opcode_cbb3() {
        test_jsmoo_file("cb b3")
    }
    #[test]
    fn test_opcode_cbb4() {
        test_jsmoo_file("cb b4")
    }
    #[test]
    fn test_opcode_cbb5() {
        test_jsmoo_file("cb b5")
    }
    #[test]
    fn test_opcode_cbb6() {
        test_jsmoo_file("cb b6")
    }
    #[test]
    fn test_opcode_cbb7() {
        test_jsmoo_file("cb b7")
    }
    #[test]
    fn test_opcode_cbb8() {
        test_jsmoo_file("cb b8")
    }
    #[test]
    fn test_opcode_cbb9() {
        test_jsmoo_file("cb b9")
    }
    #[test]
    fn test_opcode_cbba() {
        test_jsmoo_file("cb ba")
    }
    #[test]
    fn test_opcode_cbbb() {
        test_jsmoo_file("cb bb")
    }
    #[test]
    fn test_opcode_cbbc() {
        test_jsmoo_file("cb bc")
    }
    #[test]
    fn test_opcode_cbbd() {
        test_jsmoo_file("cb bd")
    }
    #[test]
    fn test_opcode_cbbe() {
        test_jsmoo_file("cb be")
    }
    #[test]
    fn test_opcode_cbbf() {
        test_jsmoo_file("cb bf")
    }
    #[test]
    fn test_opcode_cbc0() {
        test_jsmoo_file("cb c0")
    }
    #[test]
    fn test_opcode_cbc1() {
        test_jsmoo_file("cb c1")
    }
    #[test]
    fn test_opcode_cbc2() {
        test_jsmoo_file("cb c2")
    }
    #[test]
    fn test_opcode_cbc3() {
        test_jsmoo_file("cb c3")
    }
    #[test]
    fn test_opcode_cbc4() {
        test_jsmoo_file("cb c4")
    }
    #[test]
    fn test_opcode_cbc5() {
        test_jsmoo_file("cb c5")
    }
    #[test]
    fn test_opcode_cbc6() {
        test_jsmoo_file("cb c6")
    }
    #[test]
    fn test_opcode_cbc7() {
        test_jsmoo_file("cb c7")
    }
    #[test]
    fn test_opcode_cbc8() {
        test_jsmoo_file("cb c8")
    }
    #[test]
    fn test_opcode_cbc9() {
        test_jsmoo_file("cb c9")
    }
    #[test]
    fn test_opcode_cbca() {
        test_jsmoo_file("cb ca")
    }
    #[test]
    fn test_opcode_cbcb() {
        test_jsmoo_file("cb cb")
    }
    #[test]
    fn test_opcode_cbcc() {
        test_jsmoo_file("cb cc")
    }
    #[test]
    fn test_opcode_cbcd() {
        test_jsmoo_file("cb cd")
    }
    #[test]
    fn test_opcode_cbce() {
        test_jsmoo_file("cb ce")
    }
    #[test]
    fn test_opcode_cbcf() {
        test_jsmoo_file("cb cf")
    }
    #[test]
    fn test_opcode_cbd0() {
        test_jsmoo_file("cb d0")
    }
    #[test]
    fn test_opcode_cbd1() {
        test_jsmoo_file("cb d1")
    }
    #[test]
    fn test_opcode_cbd2() {
        test_jsmoo_file("cb d2")
    }
    #[test]
    fn test_opcode_cbd3() {
        test_jsmoo_file("cb d3")
    }
    #[test]
    fn test_opcode_cbd4() {
        test_jsmoo_file("cb d4")
    }
    #[test]
    fn test_opcode_cbd5() {
        test_jsmoo_file("cb d5")
    }
    #[test]
    fn test_opcode_cbd6() {
        test_jsmoo_file("cb d6")
    }
    #[test]
    fn test_opcode_cbd7() {
        test_jsmoo_file("cb d7")
    }
    #[test]
    fn test_opcode_cbd8() {
        test_jsmoo_file("cb d8")
    }
    #[test]
    fn test_opcode_cbd9() {
        test_jsmoo_file("cb d9")
    }
    #[test]
    fn test_opcode_cbda() {
        test_jsmoo_file("cb da")
    }
    #[test]
    fn test_opcode_cbdb() {
        test_jsmoo_file("cb db")
    }
    #[test]
    fn test_opcode_cbdc() {
        test_jsmoo_file("cb dc")
    }
    #[test]
    fn test_opcode_cbdd() {
        test_jsmoo_file("cb dd")
    }
    #[test]
    fn test_opcode_cbde() {
        test_jsmoo_file("cb de")
    }
    #[test]
    fn test_opcode_cbdf() {
        test_jsmoo_file("cb df")
    }
    #[test]
    fn test_opcode_cbe0() {
        test_jsmoo_file("cb e0")
    }
    #[test]
    fn test_opcode_cbe1() {
        test_jsmoo_file("cb e1")
    }
    #[test]
    fn test_opcode_cbe2() {
        test_jsmoo_file("cb e2")
    }
    #[test]
    fn test_opcode_cbe3() {
        test_jsmoo_file("cb e3")
    }
    #[test]
    fn test_opcode_cbe4() {
        test_jsmoo_file("cb e4")
    }
    #[test]
    fn test_opcode_cbe5() {
        test_jsmoo_file("cb e5")
    }
    #[test]
    fn test_opcode_cbe6() {
        test_jsmoo_file("cb e6")
    }
    #[test]
    fn test_opcode_cbe7() {
        test_jsmoo_file("cb e7")
    }
    #[test]
    fn test_opcode_cbe8() {
        test_jsmoo_file("cb e8")
    }
    #[test]
    fn test_opcode_cbe9() {
        test_jsmoo_file("cb e9")
    }
    #[test]
    fn test_opcode_cbea() {
        test_jsmoo_file("cb ea")
    }
    #[test]
    fn test_opcode_cbeb() {
        test_jsmoo_file("cb eb")
    }
    #[test]
    fn test_opcode_cbec() {
        test_jsmoo_file("cb ec")
    }
    #[test]
    fn test_opcode_cbed() {
        test_jsmoo_file("cb ed")
    }
    #[test]
    fn test_opcode_cbee() {
        test_jsmoo_file("cb ee")
    }
    #[test]
    fn test_opcode_cbef() {
        test_jsmoo_file("cb ef")
    }
    #[test]
    fn test_opcode_cbf0() {
        test_jsmoo_file("cb f0")
    }
    #[test]
    fn test_opcode_cbf1() {
        test_jsmoo_file("cb f1")
    }
    #[test]
    fn test_opcode_cbf2() {
        test_jsmoo_file("cb f2")
    }
    #[test]
    fn test_opcode_cbf3() {
        test_jsmoo_file("cb f3")
    }
    #[test]
    fn test_opcode_cbf4() {
        test_jsmoo_file("cb f4")
    }
    #[test]
    fn test_opcode_cbf5() {
        test_jsmoo_file("cb f5")
    }
    #[test]
    fn test_opcode_cbf6() {
        test_jsmoo_file("cb f6")
    }
    #[test]
    fn test_opcode_cbf7() {
        test_jsmoo_file("cb f7")
    }
    #[test]
    fn test_opcode_cbf8() {
        test_jsmoo_file("cb f8")
    }
    #[test]
    fn test_opcode_cbf9() {
        test_jsmoo_file("cb f9")
    }
    #[test]
    fn test_opcode_cbfa() {
        test_jsmoo_file("cb fa")
    }
    #[test]
    fn test_opcode_cbfb() {
        test_jsmoo_file("cb fb")
    }
    #[test]
    fn test_opcode_cbfc() {
        test_jsmoo_file("cb fc")
    }
    #[test]
    fn test_opcode_cbfd() {
        test_jsmoo_file("cb fd")
    }
    #[test]
    fn test_opcode_cbfe() {
        test_jsmoo_file("cb fe")
    }
    #[test]
    fn test_opcode_cbff() {
        test_jsmoo_file("cb ff")
    }

    #[test]
    fn test_opcode_dd00() {
        test_jsmoo_file("dd 00")
    }
    #[test]
    fn test_opcode_dd01() {
        test_jsmoo_file("dd 01")
    }
    #[test]
    fn test_opcode_dd02() {
        test_jsmoo_file("dd 02")
    }
    #[test]
    fn test_opcode_dd03() {
        test_jsmoo_file("dd 03")
    }
    #[test]
    fn test_opcode_dd04() {
        test_jsmoo_file("dd 04")
    }
    #[test]
    fn test_opcode_dd05() {
        test_jsmoo_file("dd 05")
    }
    #[test]
    fn test_opcode_dd06() {
        test_jsmoo_file("dd 06")
    }
    #[test]
    fn test_opcode_dd07() {
        test_jsmoo_file("dd 07")
    }
    #[test]
    fn test_opcode_dd08() {
        test_jsmoo_file("dd 08")
    }
    #[test]
    fn test_opcode_dd09() {
        test_jsmoo_file("dd 09")
    }
    #[test]
    fn test_opcode_dd0a() {
        test_jsmoo_file("dd 0a")
    }
    #[test]
    fn test_opcode_dd0b() {
        test_jsmoo_file("dd 0b")
    }
    #[test]
    fn test_opcode_dd0c() {
        test_jsmoo_file("dd 0c")
    }
    #[test]
    fn test_opcode_dd0d() {
        test_jsmoo_file("dd 0d")
    }
    #[test]
    fn test_opcode_dd0e() {
        test_jsmoo_file("dd 0e")
    }
    #[test]
    fn test_opcode_dd0f() {
        test_jsmoo_file("dd 0f")
    }
    #[test]
    fn test_opcode_dd10() {
        test_jsmoo_file("dd 10")
    }
    #[test]
    fn test_opcode_dd11() {
        test_jsmoo_file("dd 11")
    }
    #[test]
    fn test_opcode_dd12() {
        test_jsmoo_file("dd 12")
    }
    #[test]
    fn test_opcode_dd13() {
        test_jsmoo_file("dd 13")
    }
    #[test]
    fn test_opcode_dd14() {
        test_jsmoo_file("dd 14")
    }
    #[test]
    fn test_opcode_dd15() {
        test_jsmoo_file("dd 15")
    }
    #[test]
    fn test_opcode_dd16() {
        test_jsmoo_file("dd 16")
    }
    #[test]
    fn test_opcode_dd17() {
        test_jsmoo_file("dd 17")
    }
    #[test]
    fn test_opcode_dd18() {
        test_jsmoo_file("dd 18")
    }
    #[test]
    fn test_opcode_dd19() {
        test_jsmoo_file("dd 19")
    }
    #[test]
    fn test_opcode_dd1a() {
        test_jsmoo_file("dd 1a")
    }
    #[test]
    fn test_opcode_dd1b() {
        test_jsmoo_file("dd 1b")
    }
    #[test]
    fn test_opcode_dd1c() {
        test_jsmoo_file("dd 1c")
    }
    #[test]
    fn test_opcode_dd1d() {
        test_jsmoo_file("dd 1d")
    }
    #[test]
    fn test_opcode_dd1e() {
        test_jsmoo_file("dd 1e")
    }
    #[test]
    fn test_opcode_dd1f() {
        test_jsmoo_file("dd 1f")
    }
    #[test]
    fn test_opcode_dd20() {
        test_jsmoo_file("dd 20")
    }
    #[test]
    fn test_opcode_dd21() {
        test_jsmoo_file("dd 21")
    }
    #[test]
    fn test_opcode_dd22() {
        test_jsmoo_file("dd 22")
    }
    #[test]
    fn test_opcode_dd23() {
        test_jsmoo_file("dd 23")
    }
    #[test]
    fn test_opcode_dd24() {
        test_jsmoo_file("dd 24")
    }
    #[test]
    fn test_opcode_dd25() {
        test_jsmoo_file("dd 25")
    }
    #[test]
    fn test_opcode_dd26() {
        test_jsmoo_file("dd 26")
    }
    #[test]
    fn test_opcode_dd27() {
        test_jsmoo_file("dd 27")
    }
    #[test]
    fn test_opcode_dd28() {
        test_jsmoo_file("dd 28")
    }
    #[test]
    fn test_opcode_dd29() {
        test_jsmoo_file("dd 29")
    }
    #[test]
    fn test_opcode_dd2a() {
        test_jsmoo_file("dd 2a")
    }
    #[test]
    fn test_opcode_dd2b() {
        test_jsmoo_file("dd 2b")
    }
    #[test]
    fn test_opcode_dd2c() {
        test_jsmoo_file("dd 2c")
    }
    #[test]
    fn test_opcode_dd2d() {
        test_jsmoo_file("dd 2d")
    }
    #[test]
    fn test_opcode_dd2e() {
        test_jsmoo_file("dd 2e")
    }
    #[test]
    fn test_opcode_dd2f() {
        test_jsmoo_file("dd 2f")
    }
    #[test]
    fn test_opcode_dd30() {
        test_jsmoo_file("dd 30")
    }
    #[test]
    fn test_opcode_dd31() {
        test_jsmoo_file("dd 31")
    }
    #[test]
    fn test_opcode_dd32() {
        test_jsmoo_file("dd 32")
    }
    #[test]
    fn test_opcode_dd33() {
        test_jsmoo_file("dd 33")
    }
    #[test]
    fn test_opcode_dd34() {
        test_jsmoo_file("dd 34")
    }
    #[test]
    fn test_opcode_dd35() {
        test_jsmoo_file("dd 35")
    }
    #[test]
    fn test_opcode_dd36() {
        test_jsmoo_file("dd 36")
    }
    #[test]
    fn test_opcode_dd37() {
        test_jsmoo_file("dd 37")
    }
    #[test]
    fn test_opcode_dd38() {
        test_jsmoo_file("dd 38")
    }
    #[test]
    fn test_opcode_dd39() {
        test_jsmoo_file("dd 39")
    }
    #[test]
    fn test_opcode_dd3a() {
        test_jsmoo_file("dd 3a")
    }
    #[test]
    fn test_opcode_dd3b() {
        test_jsmoo_file("dd 3b")
    }
    #[test]
    fn test_opcode_dd3c() {
        test_jsmoo_file("dd 3c")
    }
    #[test]
    fn test_opcode_dd3d() {
        test_jsmoo_file("dd 3d")
    }
    #[test]
    fn test_opcode_dd3e() {
        test_jsmoo_file("dd 3e")
    }
    #[test]
    fn test_opcode_dd3f() {
        test_jsmoo_file("dd 3f")
    }
    #[test]
    fn test_opcode_dd40() {
        test_jsmoo_file("dd 40")
    }
    #[test]
    fn test_opcode_dd41() {
        test_jsmoo_file("dd 41")
    }
    #[test]
    fn test_opcode_dd42() {
        test_jsmoo_file("dd 42")
    }
    #[test]
    fn test_opcode_dd43() {
        test_jsmoo_file("dd 43")
    }
    #[test]
    fn test_opcode_dd44() {
        test_jsmoo_file("dd 44")
    }
    #[test]
    fn test_opcode_dd45() {
        test_jsmoo_file("dd 45")
    }
    #[test]
    fn test_opcode_dd46() {
        test_jsmoo_file("dd 46")
    }
    #[test]
    fn test_opcode_dd47() {
        test_jsmoo_file("dd 47")
    }
    #[test]
    fn test_opcode_dd48() {
        test_jsmoo_file("dd 48")
    }
    #[test]
    fn test_opcode_dd49() {
        test_jsmoo_file("dd 49")
    }
    #[test]
    fn test_opcode_dd4a() {
        test_jsmoo_file("dd 4a")
    }
    #[test]
    fn test_opcode_dd4b() {
        test_jsmoo_file("dd 4b")
    }
    #[test]
    fn test_opcode_dd4c() {
        test_jsmoo_file("dd 4c")
    }
    #[test]
    fn test_opcode_dd4d() {
        test_jsmoo_file("dd 4d")
    }
    #[test]
    fn test_opcode_dd4e() {
        test_jsmoo_file("dd 4e")
    }
    #[test]
    fn test_opcode_dd4f() {
        test_jsmoo_file("dd 4f")
    }
    #[test]
    fn test_opcode_dd50() {
        test_jsmoo_file("dd 50")
    }
    #[test]
    fn test_opcode_dd51() {
        test_jsmoo_file("dd 51")
    }
    #[test]
    fn test_opcode_dd52() {
        test_jsmoo_file("dd 52")
    }
    #[test]
    fn test_opcode_dd53() {
        test_jsmoo_file("dd 53")
    }
    #[test]
    fn test_opcode_dd54() {
        test_jsmoo_file("dd 54")
    }
    #[test]
    fn test_opcode_dd55() {
        test_jsmoo_file("dd 55")
    }
    #[test]
    fn test_opcode_dd56() {
        test_jsmoo_file("dd 56")
    }
    #[test]
    fn test_opcode_dd57() {
        test_jsmoo_file("dd 57")
    }
    #[test]
    fn test_opcode_dd58() {
        test_jsmoo_file("dd 58")
    }
    #[test]
    fn test_opcode_dd59() {
        test_jsmoo_file("dd 59")
    }
    #[test]
    fn test_opcode_dd5a() {
        test_jsmoo_file("dd 5a")
    }
    #[test]
    fn test_opcode_dd5b() {
        test_jsmoo_file("dd 5b")
    }
    #[test]
    fn test_opcode_dd5c() {
        test_jsmoo_file("dd 5c")
    }
    #[test]
    fn test_opcode_dd5d() {
        test_jsmoo_file("dd 5d")
    }
    #[test]
    fn test_opcode_dd5e() {
        test_jsmoo_file("dd 5e")
    }
    #[test]
    fn test_opcode_dd5f() {
        test_jsmoo_file("dd 5f")
    }
    #[test]
    fn test_opcode_dd60() {
        test_jsmoo_file("dd 60")
    }
    #[test]
    fn test_opcode_dd61() {
        test_jsmoo_file("dd 61")
    }
    #[test]
    fn test_opcode_dd62() {
        test_jsmoo_file("dd 62")
    }
    #[test]
    fn test_opcode_dd63() {
        test_jsmoo_file("dd 63")
    }
    #[test]
    fn test_opcode_dd64() {
        test_jsmoo_file("dd 64")
    }
    #[test]
    fn test_opcode_dd65() {
        test_jsmoo_file("dd 65")
    }
    #[test]
    fn test_opcode_dd66() {
        test_jsmoo_file("dd 66")
    }
    #[test]
    fn test_opcode_dd67() {
        test_jsmoo_file("dd 67")
    }
    #[test]
    fn test_opcode_dd68() {
        test_jsmoo_file("dd 68")
    }
    #[test]
    fn test_opcode_dd69() {
        test_jsmoo_file("dd 69")
    }
    #[test]
    fn test_opcode_dd6a() {
        test_jsmoo_file("dd 6a")
    }
    #[test]
    fn test_opcode_dd6b() {
        test_jsmoo_file("dd 6b")
    }
    #[test]
    fn test_opcode_dd6c() {
        test_jsmoo_file("dd 6c")
    }
    #[test]
    fn test_opcode_dd6d() {
        test_jsmoo_file("dd 6d")
    }
    #[test]
    fn test_opcode_dd6e() {
        test_jsmoo_file("dd 6e")
    }
    #[test]
    fn test_opcode_dd6f() {
        test_jsmoo_file("dd 6f")
    }
    #[test]
    fn test_opcode_dd70() {
        test_jsmoo_file("dd 70")
    }
    #[test]
    fn test_opcode_dd71() {
        test_jsmoo_file("dd 71")
    }
    #[test]
    fn test_opcode_dd72() {
        test_jsmoo_file("dd 72")
    }
    #[test]
    fn test_opcode_dd73() {
        test_jsmoo_file("dd 73")
    }
    #[test]
    fn test_opcode_dd74() {
        test_jsmoo_file("dd 74")
    }
    #[test]
    fn test_opcode_dd75() {
        test_jsmoo_file("dd 75")
    }
    #[test]
    fn test_opcode_dd76() {
        test_jsmoo_file("dd 76")
    }
    #[test]
    fn test_opcode_dd77() {
        test_jsmoo_file("dd 77")
    }
    #[test]
    fn test_opcode_dd78() {
        test_jsmoo_file("dd 78")
    }
    #[test]
    fn test_opcode_dd79() {
        test_jsmoo_file("dd 79")
    }
    #[test]
    fn test_opcode_dd7a() {
        test_jsmoo_file("dd 7a")
    }
    #[test]
    fn test_opcode_dd7b() {
        test_jsmoo_file("dd 7b")
    }
    #[test]
    fn test_opcode_dd7c() {
        test_jsmoo_file("dd 7c")
    }
    #[test]
    fn test_opcode_dd7d() {
        test_jsmoo_file("dd 7d")
    }
    #[test]
    fn test_opcode_dd7e() {
        test_jsmoo_file("dd 7e")
    }
    #[test]
    fn test_opcode_dd7f() {
        test_jsmoo_file("dd 7f")
    }
    #[test]
    fn test_opcode_dd80() {
        test_jsmoo_file("dd 80")
    }
    #[test]
    fn test_opcode_dd81() {
        test_jsmoo_file("dd 81")
    }
    #[test]
    fn test_opcode_dd82() {
        test_jsmoo_file("dd 82")
    }
    #[test]
    fn test_opcode_dd83() {
        test_jsmoo_file("dd 83")
    }
    #[test]
    fn test_opcode_dd84() {
        test_jsmoo_file("dd 84")
    }
    #[test]
    fn test_opcode_dd85() {
        test_jsmoo_file("dd 85")
    }
    #[test]
    fn test_opcode_dd86() {
        test_jsmoo_file("dd 86")
    }
    #[test]
    fn test_opcode_dd87() {
        test_jsmoo_file("dd 87")
    }
    #[test]
    fn test_opcode_dd88() {
        test_jsmoo_file("dd 88")
    }
    #[test]
    fn test_opcode_dd89() {
        test_jsmoo_file("dd 89")
    }
    #[test]
    fn test_opcode_dd8a() {
        test_jsmoo_file("dd 8a")
    }
    #[test]
    fn test_opcode_dd8b() {
        test_jsmoo_file("dd 8b")
    }
    #[test]
    fn test_opcode_dd8c() {
        test_jsmoo_file("dd 8c")
    }
    #[test]
    fn test_opcode_dd8d() {
        test_jsmoo_file("dd 8d")
    }
    #[test]
    fn test_opcode_dd8e() {
        test_jsmoo_file("dd 8e")
    }
    #[test]
    fn test_opcode_dd8f() {
        test_jsmoo_file("dd 8f")
    }
    #[test]
    fn test_opcode_dd90() {
        test_jsmoo_file("dd 90")
    }
    #[test]
    fn test_opcode_dd91() {
        test_jsmoo_file("dd 91")
    }
    #[test]
    fn test_opcode_dd92() {
        test_jsmoo_file("dd 92")
    }
    #[test]
    fn test_opcode_dd93() {
        test_jsmoo_file("dd 93")
    }
    #[test]
    fn test_opcode_dd94() {
        test_jsmoo_file("dd 94")
    }
    #[test]
    fn test_opcode_dd95() {
        test_jsmoo_file("dd 95")
    }
    #[test]
    fn test_opcode_dd96() {
        test_jsmoo_file("dd 96")
    }
    #[test]
    fn test_opcode_dd97() {
        test_jsmoo_file("dd 97")
    }
    #[test]
    fn test_opcode_dd98() {
        test_jsmoo_file("dd 98")
    }
    #[test]
    fn test_opcode_dd99() {
        test_jsmoo_file("dd 99")
    }
    #[test]
    fn test_opcode_dd9a() {
        test_jsmoo_file("dd 9a")
    }
    #[test]
    fn test_opcode_dd9b() {
        test_jsmoo_file("dd 9b")
    }
    #[test]
    fn test_opcode_dd9c() {
        test_jsmoo_file("dd 9c")
    }
    #[test]
    fn test_opcode_dd9d() {
        test_jsmoo_file("dd 9d")
    }
    #[test]
    fn test_opcode_dd9e() {
        test_jsmoo_file("dd 9e")
    }
    #[test]
    fn test_opcode_dd9f() {
        test_jsmoo_file("dd 9f")
    }
    #[test]
    fn test_opcode_dda0() {
        test_jsmoo_file("dd a0")
    }
    #[test]
    fn test_opcode_dda1() {
        test_jsmoo_file("dd a1")
    }
    #[test]
    fn test_opcode_dda2() {
        test_jsmoo_file("dd a2")
    }
    #[test]
    fn test_opcode_dda3() {
        test_jsmoo_file("dd a3")
    }
    #[test]
    fn test_opcode_dda4() {
        test_jsmoo_file("dd a4")
    }
    #[test]
    fn test_opcode_dda5() {
        test_jsmoo_file("dd a5")
    }
    #[test]
    fn test_opcode_dda6() {
        test_jsmoo_file("dd a6")
    }
    #[test]
    fn test_opcode_dda7() {
        test_jsmoo_file("dd a7")
    }
    #[test]
    fn test_opcode_dda8() {
        test_jsmoo_file("dd a8")
    }
    #[test]
    fn test_opcode_dda9() {
        test_jsmoo_file("dd a9")
    }
    #[test]
    fn test_opcode_ddaa() {
        test_jsmoo_file("dd aa")
    }
    #[test]
    fn test_opcode_ddab() {
        test_jsmoo_file("dd ab")
    }
    #[test]
    fn test_opcode_ddac() {
        test_jsmoo_file("dd ac")
    }
    #[test]
    fn test_opcode_ddad() {
        test_jsmoo_file("dd ad")
    }
    #[test]
    fn test_opcode_ddae() {
        test_jsmoo_file("dd ae")
    }
    #[test]
    fn test_opcode_ddaf() {
        test_jsmoo_file("dd af")
    }
    #[test]
    fn test_opcode_ddb0() {
        test_jsmoo_file("dd b0")
    }
    #[test]
    fn test_opcode_ddb1() {
        test_jsmoo_file("dd b1")
    }
    #[test]
    fn test_opcode_ddb2() {
        test_jsmoo_file("dd b2")
    }
    #[test]
    fn test_opcode_ddb3() {
        test_jsmoo_file("dd b3")
    }
    #[test]
    fn test_opcode_ddb4() {
        test_jsmoo_file("dd b4")
    }
    #[test]
    fn test_opcode_ddb5() {
        test_jsmoo_file("dd b5")
    }
    #[test]
    fn test_opcode_ddb6() {
        test_jsmoo_file("dd b6")
    }
    #[test]
    fn test_opcode_ddb7() {
        test_jsmoo_file("dd b7")
    }
    #[test]
    fn test_opcode_ddb8() {
        test_jsmoo_file("dd b8")
    }
    #[test]
    fn test_opcode_ddb9() {
        test_jsmoo_file("dd b9")
    }
    #[test]
    fn test_opcode_ddba() {
        test_jsmoo_file("dd ba")
    }
    #[test]
    fn test_opcode_ddbb() {
        test_jsmoo_file("dd bb")
    }
    #[test]
    fn test_opcode_ddbc() {
        test_jsmoo_file("dd bc")
    }
    #[test]
    fn test_opcode_ddbd() {
        test_jsmoo_file("dd bd")
    }
    #[test]
    fn test_opcode_ddbe() {
        test_jsmoo_file("dd be")
    }
    #[test]
    fn test_opcode_ddbf() {
        test_jsmoo_file("dd bf")
    }
    #[test]
    fn test_opcode_ddc0() {
        test_jsmoo_file("dd c0")
    }
    #[test]
    fn test_opcode_ddc1() {
        test_jsmoo_file("dd c1")
    }
    #[test]
    fn test_opcode_ddc2() {
        test_jsmoo_file("dd c2")
    }
    #[test]
    fn test_opcode_ddc3() {
        test_jsmoo_file("dd c3")
    }
    #[test]
    fn test_opcode_ddc4() {
        test_jsmoo_file("dd c4")
    }
    #[test]
    fn test_opcode_ddc5() {
        test_jsmoo_file("dd c5")
    }
    #[test]
    fn test_opcode_ddc6() {
        test_jsmoo_file("dd c6")
    }
    #[test]
    fn test_opcode_ddc7() {
        test_jsmoo_file("dd c7")
    }
    #[test]
    fn test_opcode_ddc8() {
        test_jsmoo_file("dd c8")
    }
    #[test]
    fn test_opcode_ddc9() {
        test_jsmoo_file("dd c9")
    }
    #[test]
    fn test_opcode_ddca() {
        test_jsmoo_file("dd ca")
    }
    #[test]
    fn test_opcode_ddcc() {
        test_jsmoo_file("dd cc")
    }
    #[test]
    fn test_opcode_ddcd() {
        test_jsmoo_file("dd cd")
    }
    #[test]
    fn test_opcode_ddce() {
        test_jsmoo_file("dd ce")
    }
    #[test]
    fn test_opcode_ddcf() {
        test_jsmoo_file("dd cf")
    }
    #[test]
    fn test_opcode_ddd0() {
        test_jsmoo_file("dd d0")
    }
    #[test]
    fn test_opcode_ddd1() {
        test_jsmoo_file("dd d1")
    }
    #[test]
    fn test_opcode_ddd2() {
        test_jsmoo_file("dd d2")
    }
    #[test]
    fn test_opcode_ddd3() {
        test_jsmoo_file("dd d3")
    }
    #[test]
    fn test_opcode_ddd4() {
        test_jsmoo_file("dd d4")
    }
    #[test]
    fn test_opcode_ddd5() {
        test_jsmoo_file("dd d5")
    }
    #[test]
    fn test_opcode_ddd6() {
        test_jsmoo_file("dd d6")
    }
    #[test]
    fn test_opcode_ddd7() {
        test_jsmoo_file("dd d7")
    }
    #[test]
    fn test_opcode_ddd8() {
        test_jsmoo_file("dd d8")
    }
    #[test]
    fn test_opcode_ddd9() {
        test_jsmoo_file("dd d9")
    }
    #[test]
    fn test_opcode_ddda() {
        test_jsmoo_file("dd da")
    }
    #[test]
    fn test_opcode_dddb() {
        test_jsmoo_file("dd db")
    }
    #[test]
    fn test_opcode_dddc() {
        test_jsmoo_file("dd dc")
    }
    #[test]
    fn test_opcode_ddde() {
        test_jsmoo_file("dd de")
    }
    #[test]
    fn test_opcode_dddf() {
        test_jsmoo_file("dd df")
    }
    #[test]
    fn test_opcode_dde0() {
        test_jsmoo_file("dd e0")
    }
    #[test]
    fn test_opcode_dde1() {
        test_jsmoo_file("dd e1")
    }
    #[test]
    fn test_opcode_dde2() {
        test_jsmoo_file("dd e2")
    }
    #[test]
    fn test_opcode_dde3() {
        test_jsmoo_file("dd e3")
    }
    #[test]
    fn test_opcode_dde4() {
        test_jsmoo_file("dd e4")
    }
    #[test]
    fn test_opcode_dde5() {
        test_jsmoo_file("dd e5")
    }
    #[test]
    fn test_opcode_dde6() {
        test_jsmoo_file("dd e6")
    }
    #[test]
    fn test_opcode_dde7() {
        test_jsmoo_file("dd e7")
    }
    #[test]
    fn test_opcode_dde8() {
        test_jsmoo_file("dd e8")
    }
    #[test]
    fn test_opcode_dde9() {
        test_jsmoo_file("dd e9")
    }
    #[test]
    fn test_opcode_ddea() {
        test_jsmoo_file("dd ea")
    }
    #[test]
    fn test_opcode_ddeb() {
        test_jsmoo_file("dd eb")
    }
    #[test]
    fn test_opcode_ddec() {
        test_jsmoo_file("dd ec")
    }
    #[test]
    fn test_opcode_ddee() {
        test_jsmoo_file("dd ee")
    }
    #[test]
    fn test_opcode_ddef() {
        test_jsmoo_file("dd ef")
    }
    #[test]
    fn test_opcode_ddf0() {
        test_jsmoo_file("dd f0")
    }
    #[test]
    fn test_opcode_ddf1() {
        test_jsmoo_file("dd f1")
    }
    #[test]
    fn test_opcode_ddf2() {
        test_jsmoo_file("dd f2")
    }
    #[test]
    fn test_opcode_ddf3() {
        test_jsmoo_file("dd f3")
    }
    #[test]
    fn test_opcode_ddf4() {
        test_jsmoo_file("dd f4")
    }
    #[test]
    fn test_opcode_ddf5() {
        test_jsmoo_file("dd f5")
    }
    #[test]
    fn test_opcode_ddf6() {
        test_jsmoo_file("dd f6")
    }
    #[test]
    fn test_opcode_ddf7() {
        test_jsmoo_file("dd f7")
    }
    #[test]
    fn test_opcode_ddf8() {
        test_jsmoo_file("dd f8")
    }
    #[test]
    fn test_opcode_ddf9() {
        test_jsmoo_file("dd f9")
    }
    #[test]
    fn test_opcode_ddfa() {
        test_jsmoo_file("dd fa")
    }
    #[test]
    fn test_opcode_ddfb() {
        test_jsmoo_file("dd fb")
    }
    #[test]
    fn test_opcode_ddfc() {
        test_jsmoo_file("dd fc")
    }
    #[test]
    fn test_opcode_ddfe() {
        test_jsmoo_file("dd fe")
    }
    #[test]
    fn test_opcode_ddff() {
        test_jsmoo_file("dd ff")
    }
    #[test]
    fn test_opcode_dd100() {
        test_jsmoo_file("dd 100")
    }
    #[test]
    fn test_opcode_dd101() {
        test_jsmoo_file("dd 101")
    }

    #[test]
    fn test_opcode_ddcb_00() {
        test_jsmoo_file("dd cb __ 00")
    }
    #[test]
    fn test_opcode_ddcb_01() {
        test_jsmoo_file("dd cb __ 01")
    }
    #[test]
    fn test_opcode_ddcb_02() {
        test_jsmoo_file("dd cb __ 02")
    }
    #[test]
    fn test_opcode_ddcb_03() {
        test_jsmoo_file("dd cb __ 03")
    }
    #[test]
    fn test_opcode_ddcb_04() {
        test_jsmoo_file("dd cb __ 04")
    }
    #[test]
    fn test_opcode_ddcb_05() {
        test_jsmoo_file("dd cb __ 05")
    }
    #[test]
    fn test_opcode_ddcb_06() {
        test_jsmoo_file("dd cb __ 06")
    }
    #[test]
    fn test_opcode_ddcb_07() {
        test_jsmoo_file("dd cb __ 07")
    }
    #[test]
    fn test_opcode_ddcb_08() {
        test_jsmoo_file("dd cb __ 08")
    }
    #[test]
    fn test_opcode_ddcb_09() {
        test_jsmoo_file("dd cb __ 09")
    }
    #[test]
    fn test_opcode_ddcb_0a() {
        test_jsmoo_file("dd cb __ 0a")
    }
    #[test]
    fn test_opcode_ddcb_0b() {
        test_jsmoo_file("dd cb __ 0b")
    }
    #[test]
    fn test_opcode_ddcb_0c() {
        test_jsmoo_file("dd cb __ 0c")
    }
    #[test]
    fn test_opcode_ddcb_0d() {
        test_jsmoo_file("dd cb __ 0d")
    }
    #[test]
    fn test_opcode_ddcb_0e() {
        test_jsmoo_file("dd cb __ 0e")
    }
    #[test]
    fn test_opcode_ddcb_0f() {
        test_jsmoo_file("dd cb __ 0f")
    }
    #[test]
    fn test_opcode_ddcb_10() {
        test_jsmoo_file("dd cb __ 10")
    }
    #[test]
    fn test_opcode_ddcb_11() {
        test_jsmoo_file("dd cb __ 11")
    }
    #[test]
    fn test_opcode_ddcb_12() {
        test_jsmoo_file("dd cb __ 12")
    }
    #[test]
    fn test_opcode_ddcb_13() {
        test_jsmoo_file("dd cb __ 13")
    }
    #[test]
    fn test_opcode_ddcb_14() {
        test_jsmoo_file("dd cb __ 14")
    }
    #[test]
    fn test_opcode_ddcb_15() {
        test_jsmoo_file("dd cb __ 15")
    }
    #[test]
    fn test_opcode_ddcb_16() {
        test_jsmoo_file("dd cb __ 16")
    }
    #[test]
    fn test_opcode_ddcb_17() {
        test_jsmoo_file("dd cb __ 17")
    }
    #[test]
    fn test_opcode_ddcb_18() {
        test_jsmoo_file("dd cb __ 18")
    }
    #[test]
    fn test_opcode_ddcb_19() {
        test_jsmoo_file("dd cb __ 19")
    }
    #[test]
    fn test_opcode_ddcb_1a() {
        test_jsmoo_file("dd cb __ 1a")
    }
    #[test]
    fn test_opcode_ddcb_1b() {
        test_jsmoo_file("dd cb __ 1b")
    }
    #[test]
    fn test_opcode_ddcb_1c() {
        test_jsmoo_file("dd cb __ 1c")
    }
    #[test]
    fn test_opcode_ddcb_1d() {
        test_jsmoo_file("dd cb __ 1d")
    }
    #[test]
    fn test_opcode_ddcb_1e() {
        test_jsmoo_file("dd cb __ 1e")
    }
    #[test]
    fn test_opcode_ddcb_1f() {
        test_jsmoo_file("dd cb __ 1f")
    }
    #[test]
    fn test_opcode_ddcb_20() {
        test_jsmoo_file("dd cb __ 20")
    }
    #[test]
    fn test_opcode_ddcb_21() {
        test_jsmoo_file("dd cb __ 21")
    }
    #[test]
    fn test_opcode_ddcb_22() {
        test_jsmoo_file("dd cb __ 22")
    }
    #[test]
    fn test_opcode_ddcb_23() {
        test_jsmoo_file("dd cb __ 23")
    }
    #[test]
    fn test_opcode_ddcb_24() {
        test_jsmoo_file("dd cb __ 24")
    }
    #[test]
    fn test_opcode_ddcb_25() {
        test_jsmoo_file("dd cb __ 25")
    }
    #[test]
    fn test_opcode_ddcb_26() {
        test_jsmoo_file("dd cb __ 26")
    }
    #[test]
    fn test_opcode_ddcb_27() {
        test_jsmoo_file("dd cb __ 27")
    }
    #[test]
    fn test_opcode_ddcb_28() {
        test_jsmoo_file("dd cb __ 28")
    }
    #[test]
    fn test_opcode_ddcb_29() {
        test_jsmoo_file("dd cb __ 29")
    }
    #[test]
    fn test_opcode_ddcb_2a() {
        test_jsmoo_file("dd cb __ 2a")
    }
    #[test]
    fn test_opcode_ddcb_2b() {
        test_jsmoo_file("dd cb __ 2b")
    }
    #[test]
    fn test_opcode_ddcb_2c() {
        test_jsmoo_file("dd cb __ 2c")
    }
    #[test]
    fn test_opcode_ddcb_2d() {
        test_jsmoo_file("dd cb __ 2d")
    }
    #[test]
    fn test_opcode_ddcb_2e() {
        test_jsmoo_file("dd cb __ 2e")
    }
    #[test]
    fn test_opcode_ddcb_2f() {
        test_jsmoo_file("dd cb __ 2f")
    }
    #[test]
    fn test_opcode_ddcb_30() {
        test_jsmoo_file("dd cb __ 30")
    }
    #[test]
    fn test_opcode_ddcb_31() {
        test_jsmoo_file("dd cb __ 31")
    }
    #[test]
    fn test_opcode_ddcb_32() {
        test_jsmoo_file("dd cb __ 32")
    }
    #[test]
    fn test_opcode_ddcb_33() {
        test_jsmoo_file("dd cb __ 33")
    }
    #[test]
    fn test_opcode_ddcb_34() {
        test_jsmoo_file("dd cb __ 34")
    }
    #[test]
    fn test_opcode_ddcb_35() {
        test_jsmoo_file("dd cb __ 35")
    }
    #[test]
    fn test_opcode_ddcb_36() {
        test_jsmoo_file("dd cb __ 36")
    }
    #[test]
    fn test_opcode_ddcb_37() {
        test_jsmoo_file("dd cb __ 37")
    }
    #[test]
    fn test_opcode_ddcb_38() {
        test_jsmoo_file("dd cb __ 38")
    }
    #[test]
    fn test_opcode_ddcb_39() {
        test_jsmoo_file("dd cb __ 39")
    }
    #[test]
    fn test_opcode_ddcb_3a() {
        test_jsmoo_file("dd cb __ 3a")
    }
    #[test]
    fn test_opcode_ddcb_3b() {
        test_jsmoo_file("dd cb __ 3b")
    }
    #[test]
    fn test_opcode_ddcb_3c() {
        test_jsmoo_file("dd cb __ 3c")
    }
    #[test]
    fn test_opcode_ddcb_3d() {
        test_jsmoo_file("dd cb __ 3d")
    }
    #[test]
    fn test_opcode_ddcb_3e() {
        test_jsmoo_file("dd cb __ 3e")
    }
    #[test]
    fn test_opcode_ddcb_3f() {
        test_jsmoo_file("dd cb __ 3f")
    }
    #[test]
    fn test_opcode_ddcb_40() {
        test_jsmoo_file("dd cb __ 40")
    }
    #[test]
    fn test_opcode_ddcb_41() {
        test_jsmoo_file("dd cb __ 41")
    }
    #[test]
    fn test_opcode_ddcb_42() {
        test_jsmoo_file("dd cb __ 42")
    }
    #[test]
    fn test_opcode_ddcb_43() {
        test_jsmoo_file("dd cb __ 43")
    }
    #[test]
    fn test_opcode_ddcb_44() {
        test_jsmoo_file("dd cb __ 44")
    }
    #[test]
    fn test_opcode_ddcb_45() {
        test_jsmoo_file("dd cb __ 45")
    }
    #[test]
    fn test_opcode_ddcb_46() {
        test_jsmoo_file("dd cb __ 46")
    }
    #[test]
    fn test_opcode_ddcb_47() {
        test_jsmoo_file("dd cb __ 47")
    }
    #[test]
    fn test_opcode_ddcb_48() {
        test_jsmoo_file("dd cb __ 48")
    }
    #[test]
    fn test_opcode_ddcb_49() {
        test_jsmoo_file("dd cb __ 49")
    }
    #[test]
    fn test_opcode_ddcb_4a() {
        test_jsmoo_file("dd cb __ 4a")
    }
    #[test]
    fn test_opcode_ddcb_4b() {
        test_jsmoo_file("dd cb __ 4b")
    }
    #[test]
    fn test_opcode_ddcb_4c() {
        test_jsmoo_file("dd cb __ 4c")
    }
    #[test]
    fn test_opcode_ddcb_4d() {
        test_jsmoo_file("dd cb __ 4d")
    }
    #[test]
    fn test_opcode_ddcb_4e() {
        test_jsmoo_file("dd cb __ 4e")
    }
    #[test]
    fn test_opcode_ddcb_4f() {
        test_jsmoo_file("dd cb __ 4f")
    }
    #[test]
    fn test_opcode_ddcb_50() {
        test_jsmoo_file("dd cb __ 50")
    }
    #[test]
    fn test_opcode_ddcb_51() {
        test_jsmoo_file("dd cb __ 51")
    }
    #[test]
    fn test_opcode_ddcb_52() {
        test_jsmoo_file("dd cb __ 52")
    }
    #[test]
    fn test_opcode_ddcb_53() {
        test_jsmoo_file("dd cb __ 53")
    }
    #[test]
    fn test_opcode_ddcb_54() {
        test_jsmoo_file("dd cb __ 54")
    }
    #[test]
    fn test_opcode_ddcb_55() {
        test_jsmoo_file("dd cb __ 55")
    }
    #[test]
    fn test_opcode_ddcb_56() {
        test_jsmoo_file("dd cb __ 56")
    }
    #[test]
    fn test_opcode_ddcb_57() {
        test_jsmoo_file("dd cb __ 57")
    }
    #[test]
    fn test_opcode_ddcb_58() {
        test_jsmoo_file("dd cb __ 58")
    }
    #[test]
    fn test_opcode_ddcb_59() {
        test_jsmoo_file("dd cb __ 59")
    }
    #[test]
    fn test_opcode_ddcb_5a() {
        test_jsmoo_file("dd cb __ 5a")
    }
    #[test]
    fn test_opcode_ddcb_5b() {
        test_jsmoo_file("dd cb __ 5b")
    }
    #[test]
    fn test_opcode_ddcb_5c() {
        test_jsmoo_file("dd cb __ 5c")
    }
    #[test]
    fn test_opcode_ddcb_5d() {
        test_jsmoo_file("dd cb __ 5d")
    }
    #[test]
    fn test_opcode_ddcb_5e() {
        test_jsmoo_file("dd cb __ 5e")
    }
    #[test]
    fn test_opcode_ddcb_5f() {
        test_jsmoo_file("dd cb __ 5f")
    }
    #[test]
    fn test_opcode_ddcb_60() {
        test_jsmoo_file("dd cb __ 60")
    }
    #[test]
    fn test_opcode_ddcb_61() {
        test_jsmoo_file("dd cb __ 61")
    }
    #[test]
    fn test_opcode_ddcb_62() {
        test_jsmoo_file("dd cb __ 62")
    }
    #[test]
    fn test_opcode_ddcb_63() {
        test_jsmoo_file("dd cb __ 63")
    }
    #[test]
    fn test_opcode_ddcb_64() {
        test_jsmoo_file("dd cb __ 64")
    }
    #[test]
    fn test_opcode_ddcb_65() {
        test_jsmoo_file("dd cb __ 65")
    }
    #[test]
    fn test_opcode_ddcb_66() {
        test_jsmoo_file("dd cb __ 66")
    }
    #[test]
    fn test_opcode_ddcb_67() {
        test_jsmoo_file("dd cb __ 67")
    }
    #[test]
    fn test_opcode_ddcb_68() {
        test_jsmoo_file("dd cb __ 68")
    }
    #[test]
    fn test_opcode_ddcb_69() {
        test_jsmoo_file("dd cb __ 69")
    }
    #[test]
    fn test_opcode_ddcb_6a() {
        test_jsmoo_file("dd cb __ 6a")
    }
    #[test]
    fn test_opcode_ddcb_6b() {
        test_jsmoo_file("dd cb __ 6b")
    }
    #[test]
    fn test_opcode_ddcb_6c() {
        test_jsmoo_file("dd cb __ 6c")
    }
    #[test]
    fn test_opcode_ddcb_6d() {
        test_jsmoo_file("dd cb __ 6d")
    }
    #[test]
    fn test_opcode_ddcb_6e() {
        test_jsmoo_file("dd cb __ 6e")
    }
    #[test]
    fn test_opcode_ddcb_6f() {
        test_jsmoo_file("dd cb __ 6f")
    }
    #[test]
    fn test_opcode_ddcb_70() {
        test_jsmoo_file("dd cb __ 70")
    }
    #[test]
    fn test_opcode_ddcb_71() {
        test_jsmoo_file("dd cb __ 71")
    }
    #[test]
    fn test_opcode_ddcb_72() {
        test_jsmoo_file("dd cb __ 72")
    }
    #[test]
    fn test_opcode_ddcb_73() {
        test_jsmoo_file("dd cb __ 73")
    }
    #[test]
    fn test_opcode_ddcb_74() {
        test_jsmoo_file("dd cb __ 74")
    }
    #[test]
    fn test_opcode_ddcb_75() {
        test_jsmoo_file("dd cb __ 75")
    }
    #[test]
    fn test_opcode_ddcb_76() {
        test_jsmoo_file("dd cb __ 76")
    }
    #[test]
    fn test_opcode_ddcb_77() {
        test_jsmoo_file("dd cb __ 77")
    }
    #[test]
    fn test_opcode_ddcb_78() {
        test_jsmoo_file("dd cb __ 78")
    }
    #[test]
    fn test_opcode_ddcb_79() {
        test_jsmoo_file("dd cb __ 79")
    }
    #[test]
    fn test_opcode_ddcb_7a() {
        test_jsmoo_file("dd cb __ 7a")
    }
    #[test]
    fn test_opcode_ddcb_7b() {
        test_jsmoo_file("dd cb __ 7b")
    }
    #[test]
    fn test_opcode_ddcb_7c() {
        test_jsmoo_file("dd cb __ 7c")
    }
    #[test]
    fn test_opcode_ddcb_7d() {
        test_jsmoo_file("dd cb __ 7d")
    }
    #[test]
    fn test_opcode_ddcb_7e() {
        test_jsmoo_file("dd cb __ 7e")
    }
    #[test]
    fn test_opcode_ddcb_7f() {
        test_jsmoo_file("dd cb __ 7f")
    }
    #[test]
    fn test_opcode_ddcb_80() {
        test_jsmoo_file("dd cb __ 80")
    }
    #[test]
    fn test_opcode_ddcb_81() {
        test_jsmoo_file("dd cb __ 81")
    }
    #[test]
    fn test_opcode_ddcb_82() {
        test_jsmoo_file("dd cb __ 82")
    }
    #[test]
    fn test_opcode_ddcb_83() {
        test_jsmoo_file("dd cb __ 83")
    }
    #[test]
    fn test_opcode_ddcb_84() {
        test_jsmoo_file("dd cb __ 84")
    }
    #[test]
    fn test_opcode_ddcb_85() {
        test_jsmoo_file("dd cb __ 85")
    }
    #[test]
    fn test_opcode_ddcb_86() {
        test_jsmoo_file("dd cb __ 86")
    }
    #[test]
    fn test_opcode_ddcb_87() {
        test_jsmoo_file("dd cb __ 87")
    }
    #[test]
    fn test_opcode_ddcb_88() {
        test_jsmoo_file("dd cb __ 88")
    }
    #[test]
    fn test_opcode_ddcb_89() {
        test_jsmoo_file("dd cb __ 89")
    }
    #[test]
    fn test_opcode_ddcb_8a() {
        test_jsmoo_file("dd cb __ 8a")
    }
    #[test]
    fn test_opcode_ddcb_8b() {
        test_jsmoo_file("dd cb __ 8b")
    }
    #[test]
    fn test_opcode_ddcb_8c() {
        test_jsmoo_file("dd cb __ 8c")
    }
    #[test]
    fn test_opcode_ddcb_8d() {
        test_jsmoo_file("dd cb __ 8d")
    }
    #[test]
    fn test_opcode_ddcb_8e() {
        test_jsmoo_file("dd cb __ 8e")
    }
    #[test]
    fn test_opcode_ddcb_8f() {
        test_jsmoo_file("dd cb __ 8f")
    }
    #[test]
    fn test_opcode_ddcb_90() {
        test_jsmoo_file("dd cb __ 90")
    }
    #[test]
    fn test_opcode_ddcb_91() {
        test_jsmoo_file("dd cb __ 91")
    }
    #[test]
    fn test_opcode_ddcb_92() {
        test_jsmoo_file("dd cb __ 92")
    }
    #[test]
    fn test_opcode_ddcb_93() {
        test_jsmoo_file("dd cb __ 93")
    }
    #[test]
    fn test_opcode_ddcb_94() {
        test_jsmoo_file("dd cb __ 94")
    }
    #[test]
    fn test_opcode_ddcb_95() {
        test_jsmoo_file("dd cb __ 95")
    }
    #[test]
    fn test_opcode_ddcb_96() {
        test_jsmoo_file("dd cb __ 96")
    }
    #[test]
    fn test_opcode_ddcb_97() {
        test_jsmoo_file("dd cb __ 97")
    }
    #[test]
    fn test_opcode_ddcb_98() {
        test_jsmoo_file("dd cb __ 98")
    }
    #[test]
    fn test_opcode_ddcb_99() {
        test_jsmoo_file("dd cb __ 99")
    }
    #[test]
    fn test_opcode_ddcb_9a() {
        test_jsmoo_file("dd cb __ 9a")
    }
    #[test]
    fn test_opcode_ddcb_9b() {
        test_jsmoo_file("dd cb __ 9b")
    }
    #[test]
    fn test_opcode_ddcb_9c() {
        test_jsmoo_file("dd cb __ 9c")
    }
    #[test]
    fn test_opcode_ddcb_9d() {
        test_jsmoo_file("dd cb __ 9d")
    }
    #[test]
    fn test_opcode_ddcb_9e() {
        test_jsmoo_file("dd cb __ 9e")
    }
    #[test]
    fn test_opcode_ddcb_9f() {
        test_jsmoo_file("dd cb __ 9f")
    }
    #[test]
    fn test_opcode_ddcb_a0() {
        test_jsmoo_file("dd cb __ a0")
    }
    #[test]
    fn test_opcode_ddcb_a1() {
        test_jsmoo_file("dd cb __ a1")
    }
    #[test]
    fn test_opcode_ddcb_a2() {
        test_jsmoo_file("dd cb __ a2")
    }
    #[test]
    fn test_opcode_ddcb_a3() {
        test_jsmoo_file("dd cb __ a3")
    }
    #[test]
    fn test_opcode_ddcb_a4() {
        test_jsmoo_file("dd cb __ a4")
    }
    #[test]
    fn test_opcode_ddcb_a5() {
        test_jsmoo_file("dd cb __ a5")
    }
    #[test]
    fn test_opcode_ddcb_a6() {
        test_jsmoo_file("dd cb __ a6")
    }
    #[test]
    fn test_opcode_ddcb_a7() {
        test_jsmoo_file("dd cb __ a7")
    }
    #[test]
    fn test_opcode_ddcb_a8() {
        test_jsmoo_file("dd cb __ a8")
    }
    #[test]
    fn test_opcode_ddcb_a9() {
        test_jsmoo_file("dd cb __ a9")
    }
    #[test]
    fn test_opcode_ddcb_aa() {
        test_jsmoo_file("dd cb __ aa")
    }
    #[test]
    fn test_opcode_ddcb_ab() {
        test_jsmoo_file("dd cb __ ab")
    }
    #[test]
    fn test_opcode_ddcb_ac() {
        test_jsmoo_file("dd cb __ ac")
    }
    #[test]
    fn test_opcode_ddcb_ad() {
        test_jsmoo_file("dd cb __ ad")
    }
    #[test]
    fn test_opcode_ddcb_ae() {
        test_jsmoo_file("dd cb __ ae")
    }
    #[test]
    fn test_opcode_ddcb_af() {
        test_jsmoo_file("dd cb __ af")
    }
    #[test]
    fn test_opcode_ddcb_b0() {
        test_jsmoo_file("dd cb __ b0")
    }
    #[test]
    fn test_opcode_ddcb_b1() {
        test_jsmoo_file("dd cb __ b1")
    }
    #[test]
    fn test_opcode_ddcb_b2() {
        test_jsmoo_file("dd cb __ b2")
    }
    #[test]
    fn test_opcode_ddcb_b3() {
        test_jsmoo_file("dd cb __ b3")
    }
    #[test]
    fn test_opcode_ddcb_b4() {
        test_jsmoo_file("dd cb __ b4")
    }
    #[test]
    fn test_opcode_ddcb_b5() {
        test_jsmoo_file("dd cb __ b5")
    }
    #[test]
    fn test_opcode_ddcb_b6() {
        test_jsmoo_file("dd cb __ b6")
    }
    #[test]
    fn test_opcode_ddcb_b7() {
        test_jsmoo_file("dd cb __ b7")
    }
    #[test]
    fn test_opcode_ddcb_b8() {
        test_jsmoo_file("dd cb __ b8")
    }
    #[test]
    fn test_opcode_ddcb_b9() {
        test_jsmoo_file("dd cb __ b9")
    }
    #[test]
    fn test_opcode_ddcb_ba() {
        test_jsmoo_file("dd cb __ ba")
    }
    #[test]
    fn test_opcode_ddcb_bb() {
        test_jsmoo_file("dd cb __ bb")
    }
    #[test]
    fn test_opcode_ddcb_bc() {
        test_jsmoo_file("dd cb __ bc")
    }
    #[test]
    fn test_opcode_ddcb_bd() {
        test_jsmoo_file("dd cb __ bd")
    }
    #[test]
    fn test_opcode_ddcb_be() {
        test_jsmoo_file("dd cb __ be")
    }
    #[test]
    fn test_opcode_ddcb_bf() {
        test_jsmoo_file("dd cb __ bf")
    }
    #[test]
    fn test_opcode_ddcb_c0() {
        test_jsmoo_file("dd cb __ c0")
    }
    #[test]
    fn test_opcode_ddcb_c1() {
        test_jsmoo_file("dd cb __ c1")
    }
    #[test]
    fn test_opcode_ddcb_c2() {
        test_jsmoo_file("dd cb __ c2")
    }
    #[test]
    fn test_opcode_ddcb_c3() {
        test_jsmoo_file("dd cb __ c3")
    }
    #[test]
    fn test_opcode_ddcb_c4() {
        test_jsmoo_file("dd cb __ c4")
    }
    #[test]
    fn test_opcode_ddcb_c5() {
        test_jsmoo_file("dd cb __ c5")
    }
    #[test]
    fn test_opcode_ddcb_c6() {
        test_jsmoo_file("dd cb __ c6")
    }
    #[test]
    fn test_opcode_ddcb_c7() {
        test_jsmoo_file("dd cb __ c7")
    }
    #[test]
    fn test_opcode_ddcb_c8() {
        test_jsmoo_file("dd cb __ c8")
    }
    #[test]
    fn test_opcode_ddcb_c9() {
        test_jsmoo_file("dd cb __ c9")
    }
    #[test]
    fn test_opcode_ddcb_ca() {
        test_jsmoo_file("dd cb __ ca")
    }
    #[test]
    fn test_opcode_ddcb_cb() {
        test_jsmoo_file("dd cb __ cb")
    }
    #[test]
    fn test_opcode_ddcb_cc() {
        test_jsmoo_file("dd cb __ cc")
    }
    #[test]
    fn test_opcode_ddcb_cd() {
        test_jsmoo_file("dd cb __ cd")
    }
    #[test]
    fn test_opcode_ddcb_ce() {
        test_jsmoo_file("dd cb __ ce")
    }
    #[test]
    fn test_opcode_ddcb_cf() {
        test_jsmoo_file("dd cb __ cf")
    }
    #[test]
    fn test_opcode_ddcb_d0() {
        test_jsmoo_file("dd cb __ d0")
    }
    #[test]
    fn test_opcode_ddcb_d1() {
        test_jsmoo_file("dd cb __ d1")
    }
    #[test]
    fn test_opcode_ddcb_d2() {
        test_jsmoo_file("dd cb __ d2")
    }
    #[test]
    fn test_opcode_ddcb_d3() {
        test_jsmoo_file("dd cb __ d3")
    }
    #[test]
    fn test_opcode_ddcb_d4() {
        test_jsmoo_file("dd cb __ d4")
    }
    #[test]
    fn test_opcode_ddcb_d5() {
        test_jsmoo_file("dd cb __ d5")
    }
    #[test]
    fn test_opcode_ddcb_d6() {
        test_jsmoo_file("dd cb __ d6")
    }
    #[test]
    fn test_opcode_ddcb_d7() {
        test_jsmoo_file("dd cb __ d7")
    }
    #[test]
    fn test_opcode_ddcb_d8() {
        test_jsmoo_file("dd cb __ d8")
    }
    #[test]
    fn test_opcode_ddcb_d9() {
        test_jsmoo_file("dd cb __ d9")
    }
    #[test]
    fn test_opcode_ddcb_da() {
        test_jsmoo_file("dd cb __ da")
    }
    #[test]
    fn test_opcode_ddcb_db() {
        test_jsmoo_file("dd cb __ db")
    }
    #[test]
    fn test_opcode_ddcb_dc() {
        test_jsmoo_file("dd cb __ dc")
    }
    #[test]
    fn test_opcode_ddcb_dd() {
        test_jsmoo_file("dd cb __ dd")
    }
    #[test]
    fn test_opcode_ddcb_de() {
        test_jsmoo_file("dd cb __ de")
    }
    #[test]
    fn test_opcode_ddcb_df() {
        test_jsmoo_file("dd cb __ df")
    }
    #[test]
    fn test_opcode_ddcb_e0() {
        test_jsmoo_file("dd cb __ e0")
    }
    #[test]
    fn test_opcode_ddcb_e1() {
        test_jsmoo_file("dd cb __ e1")
    }
    #[test]
    fn test_opcode_ddcb_e2() {
        test_jsmoo_file("dd cb __ e2")
    }
    #[test]
    fn test_opcode_ddcb_e3() {
        test_jsmoo_file("dd cb __ e3")
    }
    #[test]
    fn test_opcode_ddcb_e4() {
        test_jsmoo_file("dd cb __ e4")
    }
    #[test]
    fn test_opcode_ddcb_e5() {
        test_jsmoo_file("dd cb __ e5")
    }
    #[test]
    fn test_opcode_ddcb_e6() {
        test_jsmoo_file("dd cb __ e6")
    }
    #[test]
    fn test_opcode_ddcb_e7() {
        test_jsmoo_file("dd cb __ e7")
    }
    #[test]
    fn test_opcode_ddcb_e8() {
        test_jsmoo_file("dd cb __ e8")
    }
    #[test]
    fn test_opcode_ddcb_e9() {
        test_jsmoo_file("dd cb __ e9")
    }
    #[test]
    fn test_opcode_ddcb_ea() {
        test_jsmoo_file("dd cb __ ea")
    }
    #[test]
    fn test_opcode_ddcb_eb() {
        test_jsmoo_file("dd cb __ eb")
    }
    #[test]
    fn test_opcode_ddcb_ec() {
        test_jsmoo_file("dd cb __ ec")
    }
    #[test]
    fn test_opcode_ddcb_ed() {
        test_jsmoo_file("dd cb __ ed")
    }
    #[test]
    fn test_opcode_ddcb_ee() {
        test_jsmoo_file("dd cb __ ee")
    }
    #[test]
    fn test_opcode_ddcb_ef() {
        test_jsmoo_file("dd cb __ ef")
    }
    #[test]
    fn test_opcode_ddcb_f0() {
        test_jsmoo_file("dd cb __ f0")
    }
    #[test]
    fn test_opcode_ddcb_f1() {
        test_jsmoo_file("dd cb __ f1")
    }
    #[test]
    fn test_opcode_ddcb_f2() {
        test_jsmoo_file("dd cb __ f2")
    }
    #[test]
    fn test_opcode_ddcb_f3() {
        test_jsmoo_file("dd cb __ f3")
    }
    #[test]
    fn test_opcode_ddcb_f4() {
        test_jsmoo_file("dd cb __ f4")
    }
    #[test]
    fn test_opcode_ddcb_f5() {
        test_jsmoo_file("dd cb __ f5")
    }
    #[test]
    fn test_opcode_ddcb_f6() {
        test_jsmoo_file("dd cb __ f6")
    }
    #[test]
    fn test_opcode_ddcb_f7() {
        test_jsmoo_file("dd cb __ f7")
    }
    #[test]
    fn test_opcode_ddcb_f8() {
        test_jsmoo_file("dd cb __ f8")
    }
    #[test]
    fn test_opcode_ddcb_f9() {
        test_jsmoo_file("dd cb __ f9")
    }
    #[test]
    fn test_opcode_ddcb_fa() {
        test_jsmoo_file("dd cb __ fa")
    }
    #[test]
    fn test_opcode_ddcb_fb() {
        test_jsmoo_file("dd cb __ fb")
    }
    #[test]
    fn test_opcode_ddcb_fc() {
        test_jsmoo_file("dd cb __ fc")
    }
    #[test]
    fn test_opcode_ddcb_fd() {
        test_jsmoo_file("dd cb __ fd")
    }
    #[test]
    fn test_opcode_ddcb_fe() {
        test_jsmoo_file("dd cb __ fe")
    }
    #[test]
    fn test_opcode_ddcb_ff() {
        test_jsmoo_file("dd cb __ ff")
    }

    #[test]
    fn test_opcode_ed40() {
        test_jsmoo_file("ed 40")
    }
    #[test]
    fn test_opcode_ed41() {
        test_jsmoo_file("ed 41")
    }
    #[test]
    fn test_opcode_ed42() {
        test_jsmoo_file("ed 42")
    }
    #[test]
    fn test_opcode_ed43() {
        test_jsmoo_file("ed 43")
    }
    #[test]
    fn test_opcode_ed44() {
        test_jsmoo_file("ed 44")
    }
    #[test]
    fn test_opcode_ed45() {
        test_jsmoo_file("ed 45")
    }
    #[test]
    fn test_opcode_ed46() {
        test_jsmoo_file("ed 46")
    }
    #[test]
    fn test_opcode_ed47() {
        test_jsmoo_file("ed 47")
    }
    #[test]
    fn test_opcode_ed48() {
        test_jsmoo_file("ed 48")
    }
    #[test]
    fn test_opcode_ed49() {
        test_jsmoo_file("ed 49")
    }
    #[test]
    fn test_opcode_ed4a() {
        test_jsmoo_file("ed 4a")
    }
    #[test]
    fn test_opcode_ed4b() {
        test_jsmoo_file("ed 4b")
    }
    #[test]
    fn test_opcode_ed4c() {
        test_jsmoo_file("ed 4c")
    }
    #[test]
    fn test_opcode_ed4d() {
        test_jsmoo_file("ed 4d")
    }
    #[test]
    fn test_opcode_ed4e() {
        test_jsmoo_file("ed 4e")
    }
    #[test]
    fn test_opcode_ed4f() {
        test_jsmoo_file("ed 4f")
    }
    #[test]
    fn test_opcode_ed50() {
        test_jsmoo_file("ed 50")
    }
    #[test]
    fn test_opcode_ed51() {
        test_jsmoo_file("ed 51")
    }
    #[test]
    fn test_opcode_ed52() {
        test_jsmoo_file("ed 52")
    }
    #[test]
    fn test_opcode_ed53() {
        test_jsmoo_file("ed 53")
    }
    #[test]
    fn test_opcode_ed54() {
        test_jsmoo_file("ed 54")
    }
    #[test]
    fn test_opcode_ed55() {
        test_jsmoo_file("ed 55")
    }
    #[test]
    fn test_opcode_ed56() {
        test_jsmoo_file("ed 56")
    }
    #[test]
    fn test_opcode_ed57() {
        test_jsmoo_file("ed 57")
    }
    #[test]
    fn test_opcode_ed58() {
        test_jsmoo_file("ed 58")
    }
    #[test]
    fn test_opcode_ed59() {
        test_jsmoo_file("ed 59")
    }
    #[test]
    fn test_opcode_ed5a() {
        test_jsmoo_file("ed 5a")
    }
    #[test]
    fn test_opcode_ed5b() {
        test_jsmoo_file("ed 5b")
    }
    #[test]
    fn test_opcode_ed5c() {
        test_jsmoo_file("ed 5c")
    }
    #[test]
    fn test_opcode_ed5d() {
        test_jsmoo_file("ed 5d")
    }
    #[test]
    fn test_opcode_ed5e() {
        test_jsmoo_file("ed 5e")
    }
    #[test]
    fn test_opcode_ed5f() {
        test_jsmoo_file("ed 5f")
    }
    #[test]
    fn test_opcode_ed60() {
        test_jsmoo_file("ed 60")
    }
    #[test]
    fn test_opcode_ed61() {
        test_jsmoo_file("ed 61")
    }
    #[test]
    fn test_opcode_ed62() {
        test_jsmoo_file("ed 62")
    }
    #[test]
    fn test_opcode_ed63() {
        test_jsmoo_file("ed 63")
    }
    #[test]
    fn test_opcode_ed64() {
        test_jsmoo_file("ed 64")
    }
    #[test]
    fn test_opcode_ed65() {
        test_jsmoo_file("ed 65")
    }
    #[test]
    fn test_opcode_ed66() {
        test_jsmoo_file("ed 66")
    }
    #[test]
    fn test_opcode_ed67() {
        test_jsmoo_file("ed 67")
    }
    #[test]
    fn test_opcode_ed68() {
        test_jsmoo_file("ed 68")
    }
    #[test]
    fn test_opcode_ed69() {
        test_jsmoo_file("ed 69")
    }
    #[test]
    fn test_opcode_ed6a() {
        test_jsmoo_file("ed 6a")
    }
    #[test]
    fn test_opcode_ed6b() {
        test_jsmoo_file("ed 6b")
    }
    #[test]
    fn test_opcode_ed6c() {
        test_jsmoo_file("ed 6c")
    }
    #[test]
    fn test_opcode_ed6d() {
        test_jsmoo_file("ed 6d")
    }
    #[test]
    fn test_opcode_ed6e() {
        test_jsmoo_file("ed 6e")
    }
    #[test]
    fn test_opcode_ed6f() {
        test_jsmoo_file("ed 6f")
    }
    #[test]
    fn test_opcode_ed70() {
        test_jsmoo_file("ed 70")
    }
    #[test]
    fn test_opcode_ed71() {
        test_jsmoo_file("ed 71")
    }
    #[test]
    fn test_opcode_ed72() {
        test_jsmoo_file("ed 72")
    }
    #[test]
    fn test_opcode_ed73() {
        test_jsmoo_file("ed 73")
    }
    #[test]
    fn test_opcode_ed74() {
        test_jsmoo_file("ed 74")
    }
    #[test]
    fn test_opcode_ed75() {
        test_jsmoo_file("ed 75")
    }
    #[test]
    fn test_opcode_ed76() {
        test_jsmoo_file("ed 76")
    }
    #[test]
    fn test_opcode_ed77() {
        test_jsmoo_file("ed 77")
    }
    #[test]
    fn test_opcode_ed78() {
        test_jsmoo_file("ed 78")
    }
    #[test]
    fn test_opcode_ed79() {
        test_jsmoo_file("ed 79")
    }
    #[test]
    fn test_opcode_ed7a() {
        test_jsmoo_file("ed 7a")
    }
    #[test]
    fn test_opcode_ed7b() {
        test_jsmoo_file("ed 7b")
    }
    #[test]
    fn test_opcode_ed7c() {
        test_jsmoo_file("ed 7c")
    }
    #[test]
    fn test_opcode_ed7d() {
        test_jsmoo_file("ed 7d")
    }
    #[test]
    fn test_opcode_ed7e() {
        test_jsmoo_file("ed 7e")
    }
    #[test]
    fn test_opcode_ed7f() {
        test_jsmoo_file("ed 7f")
    }
    #[test]
    fn test_opcode_eda0() {
        test_jsmoo_file("ed a0")
    }
    #[test]
    fn test_opcode_eda1() {
        test_jsmoo_file("ed a1")
    }
    #[test]
    fn test_opcode_eda2() {
        test_jsmoo_file("ed a2")
    }
    #[test]
    fn test_opcode_eda3() {
        test_jsmoo_file("ed a3")
    }
    #[test]
    fn test_opcode_eda8() {
        test_jsmoo_file("ed a8")
    }
    #[test]
    fn test_opcode_eda9() {
        test_jsmoo_file("ed a9")
    }
    #[test]
    fn test_opcode_edaa() {
        test_jsmoo_file("ed aa")
    }
    #[test]
    fn test_opcode_edab() {
        test_jsmoo_file("ed ab")
    }
    #[test]
    fn test_opcode_edb0() {
        test_jsmoo_file("ed b0")
    }
    #[test]
    fn test_opcode_edb1() {
        test_jsmoo_file("ed b1")
    }
    #[test]
    fn test_opcode_edb2() {
        test_jsmoo_file("ed b2")
    }
    #[test]
    fn test_opcode_edb3() {
        test_jsmoo_file("ed b3")
    }
    #[test]
    fn test_opcode_edb8() {
        test_jsmoo_file("ed b8")
    }
    #[test]
    fn test_opcode_edb9() {
        test_jsmoo_file("ed b9")
    }
    #[test]
    fn test_opcode_edba() {
        test_jsmoo_file("ed ba")
    }
    #[test]
    fn test_opcode_edbb() {
        test_jsmoo_file("ed bb")
    }

    #[test]
    fn test_opcode_fd00() {
        test_jsmoo_file("fd 00")
    }
    #[test]
    fn test_opcode_fd01() {
        test_jsmoo_file("fd 01")
    }
    #[test]
    fn test_opcode_fd02() {
        test_jsmoo_file("fd 02")
    }
    #[test]
    fn test_opcode_fd03() {
        test_jsmoo_file("fd 03")
    }
    #[test]
    fn test_opcode_fd04() {
        test_jsmoo_file("fd 04")
    }
    #[test]
    fn test_opcode_fd05() {
        test_jsmoo_file("fd 05")
    }
    #[test]
    fn test_opcode_fd06() {
        test_jsmoo_file("fd 06")
    }
    #[test]
    fn test_opcode_fd07() {
        test_jsmoo_file("fd 07")
    }
    #[test]
    fn test_opcode_fd08() {
        test_jsmoo_file("fd 08")
    }
    #[test]
    fn test_opcode_fd09() {
        test_jsmoo_file("fd 09")
    }
    #[test]
    fn test_opcode_fd0a() {
        test_jsmoo_file("fd 0a")
    }
    #[test]
    fn test_opcode_fd0b() {
        test_jsmoo_file("fd 0b")
    }
    #[test]
    fn test_opcode_fd0c() {
        test_jsmoo_file("fd 0c")
    }
    #[test]
    fn test_opcode_fd0d() {
        test_jsmoo_file("fd 0d")
    }
    #[test]
    fn test_opcode_fd0e() {
        test_jsmoo_file("fd 0e")
    }
    #[test]
    fn test_opcode_fd0f() {
        test_jsmoo_file("fd 0f")
    }
    #[test]
    fn test_opcode_fd10() {
        test_jsmoo_file("fd 10")
    }
    #[test]
    fn test_opcode_fd11() {
        test_jsmoo_file("fd 11")
    }
    #[test]
    fn test_opcode_fd12() {
        test_jsmoo_file("fd 12")
    }
    #[test]
    fn test_opcode_fd13() {
        test_jsmoo_file("fd 13")
    }
    #[test]
    fn test_opcode_fd14() {
        test_jsmoo_file("fd 14")
    }
    #[test]
    fn test_opcode_fd15() {
        test_jsmoo_file("fd 15")
    }
    #[test]
    fn test_opcode_fd16() {
        test_jsmoo_file("fd 16")
    }
    #[test]
    fn test_opcode_fd17() {
        test_jsmoo_file("fd 17")
    }
    #[test]
    fn test_opcode_fd18() {
        test_jsmoo_file("fd 18")
    }
    #[test]
    fn test_opcode_fd19() {
        test_jsmoo_file("fd 19")
    }
    #[test]
    fn test_opcode_fd1a() {
        test_jsmoo_file("fd 1a")
    }
    #[test]
    fn test_opcode_fd1b() {
        test_jsmoo_file("fd 1b")
    }
    #[test]
    fn test_opcode_fd1c() {
        test_jsmoo_file("fd 1c")
    }
    #[test]
    fn test_opcode_fd1d() {
        test_jsmoo_file("fd 1d")
    }
    #[test]
    fn test_opcode_fd1e() {
        test_jsmoo_file("fd 1e")
    }
    #[test]
    fn test_opcode_fd1f() {
        test_jsmoo_file("fd 1f")
    }
    #[test]
    fn test_opcode_fd20() {
        test_jsmoo_file("fd 20")
    }
    #[test]
    fn test_opcode_fd21() {
        test_jsmoo_file("fd 21")
    }
    #[test]
    fn test_opcode_fd22() {
        test_jsmoo_file("fd 22")
    }
    #[test]
    fn test_opcode_fd23() {
        test_jsmoo_file("fd 23")
    }
    #[test]
    fn test_opcode_fd24() {
        test_jsmoo_file("fd 24")
    }
    #[test]
    fn test_opcode_fd25() {
        test_jsmoo_file("fd 25")
    }
    #[test]
    fn test_opcode_fd26() {
        test_jsmoo_file("fd 26")
    }
    #[test]
    fn test_opcode_fd27() {
        test_jsmoo_file("fd 27")
    }
    #[test]
    fn test_opcode_fd28() {
        test_jsmoo_file("fd 28")
    }
    #[test]
    fn test_opcode_fd29() {
        test_jsmoo_file("fd 29")
    }
    #[test]
    fn test_opcode_fd2a() {
        test_jsmoo_file("fd 2a")
    }
    #[test]
    fn test_opcode_fd2b() {
        test_jsmoo_file("fd 2b")
    }
    #[test]
    fn test_opcode_fd2c() {
        test_jsmoo_file("fd 2c")
    }
    #[test]
    fn test_opcode_fd2d() {
        test_jsmoo_file("fd 2d")
    }
    #[test]
    fn test_opcode_fd2e() {
        test_jsmoo_file("fd 2e")
    }
    #[test]
    fn test_opcode_fd2f() {
        test_jsmoo_file("fd 2f")
    }
    #[test]
    fn test_opcode_fd30() {
        test_jsmoo_file("fd 30")
    }
    #[test]
    fn test_opcode_fd31() {
        test_jsmoo_file("fd 31")
    }
    #[test]
    fn test_opcode_fd32() {
        test_jsmoo_file("fd 32")
    }
    #[test]
    fn test_opcode_fd33() {
        test_jsmoo_file("fd 33")
    }
    #[test]
    fn test_opcode_fd34() {
        test_jsmoo_file("fd 34")
    }
    #[test]
    fn test_opcode_fd35() {
        test_jsmoo_file("fd 35")
    }
    #[test]
    fn test_opcode_fd36() {
        test_jsmoo_file("fd 36")
    }
    #[test]
    fn test_opcode_fd37() {
        test_jsmoo_file("fd 37")
    }
    #[test]
    fn test_opcode_fd38() {
        test_jsmoo_file("fd 38")
    }
    #[test]
    fn test_opcode_fd39() {
        test_jsmoo_file("fd 39")
    }
    #[test]
    fn test_opcode_fd3a() {
        test_jsmoo_file("fd 3a")
    }
    #[test]
    fn test_opcode_fd3b() {
        test_jsmoo_file("fd 3b")
    }
    #[test]
    fn test_opcode_fd3c() {
        test_jsmoo_file("fd 3c")
    }
    #[test]
    fn test_opcode_fd3d() {
        test_jsmoo_file("fd 3d")
    }
    #[test]
    fn test_opcode_fd3e() {
        test_jsmoo_file("fd 3e")
    }
    #[test]
    fn test_opcode_fd3f() {
        test_jsmoo_file("fd 3f")
    }
    #[test]
    fn test_opcode_fd40() {
        test_jsmoo_file("fd 40")
    }
    #[test]
    fn test_opcode_fd41() {
        test_jsmoo_file("fd 41")
    }
    #[test]
    fn test_opcode_fd42() {
        test_jsmoo_file("fd 42")
    }
    #[test]
    fn test_opcode_fd43() {
        test_jsmoo_file("fd 43")
    }
    #[test]
    fn test_opcode_fd44() {
        test_jsmoo_file("fd 44")
    }
    #[test]
    fn test_opcode_fd45() {
        test_jsmoo_file("fd 45")
    }
    #[test]
    fn test_opcode_fd46() {
        test_jsmoo_file("fd 46")
    }
    #[test]
    fn test_opcode_fd47() {
        test_jsmoo_file("fd 47")
    }
    #[test]
    fn test_opcode_fd48() {
        test_jsmoo_file("fd 48")
    }
    #[test]
    fn test_opcode_fd49() {
        test_jsmoo_file("fd 49")
    }
    #[test]
    fn test_opcode_fd4a() {
        test_jsmoo_file("fd 4a")
    }
    #[test]
    fn test_opcode_fd4b() {
        test_jsmoo_file("fd 4b")
    }
    #[test]
    fn test_opcode_fd4c() {
        test_jsmoo_file("fd 4c")
    }
    #[test]
    fn test_opcode_fd4d() {
        test_jsmoo_file("fd 4d")
    }
    #[test]
    fn test_opcode_fd4e() {
        test_jsmoo_file("fd 4e")
    }
    #[test]
    fn test_opcode_fd4f() {
        test_jsmoo_file("fd 4f")
    }
    #[test]
    fn test_opcode_fd50() {
        test_jsmoo_file("fd 50")
    }
    #[test]
    fn test_opcode_fd51() {
        test_jsmoo_file("fd 51")
    }
    #[test]
    fn test_opcode_fd52() {
        test_jsmoo_file("fd 52")
    }
    #[test]
    fn test_opcode_fd53() {
        test_jsmoo_file("fd 53")
    }
    #[test]
    fn test_opcode_fd54() {
        test_jsmoo_file("fd 54")
    }
    #[test]
    fn test_opcode_fd55() {
        test_jsmoo_file("fd 55")
    }
    #[test]
    fn test_opcode_fd56() {
        test_jsmoo_file("fd 56")
    }
    #[test]
    fn test_opcode_fd57() {
        test_jsmoo_file("fd 57")
    }
    #[test]
    fn test_opcode_fd58() {
        test_jsmoo_file("fd 58")
    }
    #[test]
    fn test_opcode_fd59() {
        test_jsmoo_file("fd 59")
    }
    #[test]
    fn test_opcode_fd5a() {
        test_jsmoo_file("fd 5a")
    }
    #[test]
    fn test_opcode_fd5b() {
        test_jsmoo_file("fd 5b")
    }
    #[test]
    fn test_opcode_fd5c() {
        test_jsmoo_file("fd 5c")
    }
    #[test]
    fn test_opcode_fd5d() {
        test_jsmoo_file("fd 5d")
    }
    #[test]
    fn test_opcode_fd5e() {
        test_jsmoo_file("fd 5e")
    }
    #[test]
    fn test_opcode_fd5f() {
        test_jsmoo_file("fd 5f")
    }
    #[test]
    fn test_opcode_fd60() {
        test_jsmoo_file("fd 60")
    }
    #[test]
    fn test_opcode_fd61() {
        test_jsmoo_file("fd 61")
    }
    #[test]
    fn test_opcode_fd62() {
        test_jsmoo_file("fd 62")
    }
    #[test]
    fn test_opcode_fd63() {
        test_jsmoo_file("fd 63")
    }
    #[test]
    fn test_opcode_fd64() {
        test_jsmoo_file("fd 64")
    }
    #[test]
    fn test_opcode_fd65() {
        test_jsmoo_file("fd 65")
    }
    #[test]
    fn test_opcode_fd66() {
        test_jsmoo_file("fd 66")
    }
    #[test]
    fn test_opcode_fd67() {
        test_jsmoo_file("fd 67")
    }
    #[test]
    fn test_opcode_fd68() {
        test_jsmoo_file("fd 68")
    }
    #[test]
    fn test_opcode_fd69() {
        test_jsmoo_file("fd 69")
    }
    #[test]
    fn test_opcode_fd6a() {
        test_jsmoo_file("fd 6a")
    }
    #[test]
    fn test_opcode_fd6b() {
        test_jsmoo_file("fd 6b")
    }
    #[test]
    fn test_opcode_fd6c() {
        test_jsmoo_file("fd 6c")
    }
    #[test]
    fn test_opcode_fd6d() {
        test_jsmoo_file("fd 6d")
    }
    #[test]
    fn test_opcode_fd6e() {
        test_jsmoo_file("fd 6e")
    }
    #[test]
    fn test_opcode_fd6f() {
        test_jsmoo_file("fd 6f")
    }
    #[test]
    fn test_opcode_fd70() {
        test_jsmoo_file("fd 70")
    }
    #[test]
    fn test_opcode_fd71() {
        test_jsmoo_file("fd 71")
    }
    #[test]
    fn test_opcode_fd72() {
        test_jsmoo_file("fd 72")
    }
    #[test]
    fn test_opcode_fd73() {
        test_jsmoo_file("fd 73")
    }
    #[test]
    fn test_opcode_fd74() {
        test_jsmoo_file("fd 74")
    }
    #[test]
    fn test_opcode_fd75() {
        test_jsmoo_file("fd 75")
    }
    #[test]
    fn test_opcode_fd76() {
        test_jsmoo_file("fd 76")
    }
    #[test]
    fn test_opcode_fd77() {
        test_jsmoo_file("fd 77")
    }
    #[test]
    fn test_opcode_fd78() {
        test_jsmoo_file("fd 78")
    }
    #[test]
    fn test_opcode_fd79() {
        test_jsmoo_file("fd 79")
    }
    #[test]
    fn test_opcode_fd7a() {
        test_jsmoo_file("fd 7a")
    }
    #[test]
    fn test_opcode_fd7b() {
        test_jsmoo_file("fd 7b")
    }
    #[test]
    fn test_opcode_fd7c() {
        test_jsmoo_file("fd 7c")
    }
    #[test]
    fn test_opcode_fd7d() {
        test_jsmoo_file("fd 7d")
    }
    #[test]
    fn test_opcode_fd7e() {
        test_jsmoo_file("fd 7e")
    }
    #[test]
    fn test_opcode_fd7f() {
        test_jsmoo_file("fd 7f")
    }
    #[test]
    fn test_opcode_fd80() {
        test_jsmoo_file("fd 80")
    }
    #[test]
    fn test_opcode_fd81() {
        test_jsmoo_file("fd 81")
    }
    #[test]
    fn test_opcode_fd82() {
        test_jsmoo_file("fd 82")
    }
    #[test]
    fn test_opcode_fd83() {
        test_jsmoo_file("fd 83")
    }
    #[test]
    fn test_opcode_fd84() {
        test_jsmoo_file("fd 84")
    }
    #[test]
    fn test_opcode_fd85() {
        test_jsmoo_file("fd 85")
    }
    #[test]
    fn test_opcode_fd86() {
        test_jsmoo_file("fd 86")
    }
    #[test]
    fn test_opcode_fd87() {
        test_jsmoo_file("fd 87")
    }
    #[test]
    fn test_opcode_fd88() {
        test_jsmoo_file("fd 88")
    }
    #[test]
    fn test_opcode_fd89() {
        test_jsmoo_file("fd 89")
    }
    #[test]
    fn test_opcode_fd8a() {
        test_jsmoo_file("fd 8a")
    }
    #[test]
    fn test_opcode_fd8b() {
        test_jsmoo_file("fd 8b")
    }
    #[test]
    fn test_opcode_fd8c() {
        test_jsmoo_file("fd 8c")
    }
    #[test]
    fn test_opcode_fd8d() {
        test_jsmoo_file("fd 8d")
    }
    #[test]
    fn test_opcode_fd8e() {
        test_jsmoo_file("fd 8e")
    }
    #[test]
    fn test_opcode_fd8f() {
        test_jsmoo_file("fd 8f")
    }
    #[test]
    fn test_opcode_fd90() {
        test_jsmoo_file("fd 90")
    }
    #[test]
    fn test_opcode_fd91() {
        test_jsmoo_file("fd 91")
    }
    #[test]
    fn test_opcode_fd92() {
        test_jsmoo_file("fd 92")
    }
    #[test]
    fn test_opcode_fd93() {
        test_jsmoo_file("fd 93")
    }
    #[test]
    fn test_opcode_fd94() {
        test_jsmoo_file("fd 94")
    }
    #[test]
    fn test_opcode_fd95() {
        test_jsmoo_file("fd 95")
    }
    #[test]
    fn test_opcode_fd96() {
        test_jsmoo_file("fd 96")
    }
    #[test]
    fn test_opcode_fd97() {
        test_jsmoo_file("fd 97")
    }
    #[test]
    fn test_opcode_fd98() {
        test_jsmoo_file("fd 98")
    }
    #[test]
    fn test_opcode_fd99() {
        test_jsmoo_file("fd 99")
    }
    #[test]
    fn test_opcode_fd9a() {
        test_jsmoo_file("fd 9a")
    }
    #[test]
    fn test_opcode_fd9b() {
        test_jsmoo_file("fd 9b")
    }
    #[test]
    fn test_opcode_fd9c() {
        test_jsmoo_file("fd 9c")
    }
    #[test]
    fn test_opcode_fd9d() {
        test_jsmoo_file("fd 9d")
    }
    #[test]
    fn test_opcode_fd9e() {
        test_jsmoo_file("fd 9e")
    }
    #[test]
    fn test_opcode_fd9f() {
        test_jsmoo_file("fd 9f")
    }
    #[test]
    fn test_opcode_fda0() {
        test_jsmoo_file("fd a0")
    }
    #[test]
    fn test_opcode_fda1() {
        test_jsmoo_file("fd a1")
    }
    #[test]
    fn test_opcode_fda2() {
        test_jsmoo_file("fd a2")
    }
    #[test]
    fn test_opcode_fda3() {
        test_jsmoo_file("fd a3")
    }
    #[test]
    fn test_opcode_fda4() {
        test_jsmoo_file("fd a4")
    }
    #[test]
    fn test_opcode_fda5() {
        test_jsmoo_file("fd a5")
    }
    #[test]
    fn test_opcode_fda6() {
        test_jsmoo_file("fd a6")
    }
    #[test]
    fn test_opcode_fda7() {
        test_jsmoo_file("fd a7")
    }
    #[test]
    fn test_opcode_fda8() {
        test_jsmoo_file("fd a8")
    }
    #[test]
    fn test_opcode_fda9() {
        test_jsmoo_file("fd a9")
    }
    #[test]
    fn test_opcode_fdaa() {
        test_jsmoo_file("fd aa")
    }
    #[test]
    fn test_opcode_fdab() {
        test_jsmoo_file("fd ab")
    }
    #[test]
    fn test_opcode_fdac() {
        test_jsmoo_file("fd ac")
    }
    #[test]
    fn test_opcode_fdad() {
        test_jsmoo_file("fd ad")
    }
    #[test]
    fn test_opcode_fdae() {
        test_jsmoo_file("fd ae")
    }
    #[test]
    fn test_opcode_fdaf() {
        test_jsmoo_file("fd af")
    }
    #[test]
    fn test_opcode_fdb0() {
        test_jsmoo_file("fd b0")
    }
    #[test]
    fn test_opcode_fdb1() {
        test_jsmoo_file("fd b1")
    }
    #[test]
    fn test_opcode_fdb2() {
        test_jsmoo_file("fd b2")
    }
    #[test]
    fn test_opcode_fdb3() {
        test_jsmoo_file("fd b3")
    }
    #[test]
    fn test_opcode_fdb4() {
        test_jsmoo_file("fd b4")
    }
    #[test]
    fn test_opcode_fdb5() {
        test_jsmoo_file("fd b5")
    }
    #[test]
    fn test_opcode_fdb6() {
        test_jsmoo_file("fd b6")
    }
    #[test]
    fn test_opcode_fdb7() {
        test_jsmoo_file("fd b7")
    }
    #[test]
    fn test_opcode_fdb8() {
        test_jsmoo_file("fd b8")
    }
    #[test]
    fn test_opcode_fdb9() {
        test_jsmoo_file("fd b9")
    }
    #[test]
    fn test_opcode_fdba() {
        test_jsmoo_file("fd ba")
    }
    #[test]
    fn test_opcode_fdbb() {
        test_jsmoo_file("fd bb")
    }
    #[test]
    fn test_opcode_fdbc() {
        test_jsmoo_file("fd bc")
    }
    #[test]
    fn test_opcode_fdbd() {
        test_jsmoo_file("fd bd")
    }
    #[test]
    fn test_opcode_fdbe() {
        test_jsmoo_file("fd be")
    }
    #[test]
    fn test_opcode_fdbf() {
        test_jsmoo_file("fd bf")
    }
    #[test]
    fn test_opcode_fdc0() {
        test_jsmoo_file("fd c0")
    }
    #[test]
    fn test_opcode_fdc1() {
        test_jsmoo_file("fd c1")
    }
    #[test]
    fn test_opcode_fdc2() {
        test_jsmoo_file("fd c2")
    }
    #[test]
    fn test_opcode_fdc3() {
        test_jsmoo_file("fd c3")
    }
    #[test]
    fn test_opcode_fdc4() {
        test_jsmoo_file("fd c4")
    }
    #[test]
    fn test_opcode_fdc5() {
        test_jsmoo_file("fd c5")
    }
    #[test]
    fn test_opcode_fdc6() {
        test_jsmoo_file("fd c6")
    }
    #[test]
    fn test_opcode_fdc7() {
        test_jsmoo_file("fd c7")
    }
    #[test]
    fn test_opcode_fdc8() {
        test_jsmoo_file("fd c8")
    }
    #[test]
    fn test_opcode_fdc9() {
        test_jsmoo_file("fd c9")
    }
    #[test]
    fn test_opcode_fdca() {
        test_jsmoo_file("fd ca")
    }
    #[test]
    fn test_opcode_fdcc() {
        test_jsmoo_file("fd cc")
    }
    #[test]
    fn test_opcode_fdcd() {
        test_jsmoo_file("fd cd")
    }
    #[test]
    fn test_opcode_fdce() {
        test_jsmoo_file("fd ce")
    }
    #[test]
    fn test_opcode_fdcf() {
        test_jsmoo_file("fd cf")
    }
    #[test]
    fn test_opcode_fdd0() {
        test_jsmoo_file("fd d0")
    }
    #[test]
    fn test_opcode_fdd1() {
        test_jsmoo_file("fd d1")
    }
    #[test]
    fn test_opcode_fdd2() {
        test_jsmoo_file("fd d2")
    }
    #[test]
    fn test_opcode_fdd3() {
        test_jsmoo_file("fd d3")
    }
    #[test]
    fn test_opcode_fdd4() {
        test_jsmoo_file("fd d4")
    }
    #[test]
    fn test_opcode_fdd5() {
        test_jsmoo_file("fd d5")
    }
    #[test]
    fn test_opcode_fdd6() {
        test_jsmoo_file("fd d6")
    }
    #[test]
    fn test_opcode_fdd7() {
        test_jsmoo_file("fd d7")
    }
    #[test]
    fn test_opcode_fdd8() {
        test_jsmoo_file("fd d8")
    }
    #[test]
    fn test_opcode_fdd9() {
        test_jsmoo_file("fd d9")
    }
    #[test]
    fn test_opcode_fdda() {
        test_jsmoo_file("fd da")
    }
    #[test]
    fn test_opcode_fddb() {
        test_jsmoo_file("fd db")
    }
    #[test]
    fn test_opcode_fddc() {
        test_jsmoo_file("fd dc")
    }
    #[test]
    fn test_opcode_fdde() {
        test_jsmoo_file("fd de")
    }
    #[test]
    fn test_opcode_fddf() {
        test_jsmoo_file("fd df")
    }
    #[test]
    fn test_opcode_fde0() {
        test_jsmoo_file("fd e0")
    }
    #[test]
    fn test_opcode_fde1() {
        test_jsmoo_file("fd e1")
    }
    #[test]
    fn test_opcode_fde2() {
        test_jsmoo_file("fd e2")
    }
    #[test]
    fn test_opcode_fde3() {
        test_jsmoo_file("fd e3")
    }
    #[test]
    fn test_opcode_fde4() {
        test_jsmoo_file("fd e4")
    }
    #[test]
    fn test_opcode_fde5() {
        test_jsmoo_file("fd e5")
    }
    #[test]
    fn test_opcode_fde6() {
        test_jsmoo_file("fd e6")
    }
    #[test]
    fn test_opcode_fde7() {
        test_jsmoo_file("fd e7")
    }
    #[test]
    fn test_opcode_fde8() {
        test_jsmoo_file("fd e8")
    }
    #[test]
    fn test_opcode_fde9() {
        test_jsmoo_file("fd e9")
    }
    #[test]
    fn test_opcode_fdea() {
        test_jsmoo_file("fd ea")
    }
    #[test]
    fn test_opcode_fdeb() {
        test_jsmoo_file("fd eb")
    }
    #[test]
    fn test_opcode_fdec() {
        test_jsmoo_file("fd ec")
    }
    #[test]
    fn test_opcode_fdee() {
        test_jsmoo_file("fd ee")
    }
    #[test]
    fn test_opcode_fdef() {
        test_jsmoo_file("fd ef")
    }
    #[test]
    fn test_opcode_fdf0() {
        test_jsmoo_file("fd f0")
    }
    #[test]
    fn test_opcode_fdf1() {
        test_jsmoo_file("fd f1")
    }
    #[test]
    fn test_opcode_fdf2() {
        test_jsmoo_file("fd f2")
    }
    #[test]
    fn test_opcode_fdf3() {
        test_jsmoo_file("fd f3")
    }
    #[test]
    fn test_opcode_fdf4() {
        test_jsmoo_file("fd f4")
    }
    #[test]
    fn test_opcode_fdf5() {
        test_jsmoo_file("fd f5")
    }
    #[test]
    fn test_opcode_fdf6() {
        test_jsmoo_file("fd f6")
    }
    #[test]
    fn test_opcode_fdf7() {
        test_jsmoo_file("fd f7")
    }
    #[test]
    fn test_opcode_fdf8() {
        test_jsmoo_file("fd f8")
    }
    #[test]
    fn test_opcode_fdf9() {
        test_jsmoo_file("fd f9")
    }
    #[test]
    fn test_opcode_fdfa() {
        test_jsmoo_file("fd fa")
    }
    #[test]
    fn test_opcode_fdfb() {
        test_jsmoo_file("fd fb")
    }
    #[test]
    fn test_opcode_fdfc() {
        test_jsmoo_file("fd fc")
    }
    #[test]
    fn test_opcode_fdfe() {
        test_jsmoo_file("fd fe")
    }
    #[test]
    fn test_opcode_fdff() {
        test_jsmoo_file("fd ff")
    }
    #[test]
    fn test_opcode_fd100() {
        test_jsmoo_file("fd 100")
    }
    #[test]
    fn test_opcode_fd101() {
        test_jsmoo_file("fd 101")
    }

    #[test]
    fn test_opcode_fdcb_00() {
        test_jsmoo_file("fd cb __ 00")
    }
    #[test]
    fn test_opcode_fdcb_01() {
        test_jsmoo_file("fd cb __ 01")
    }
    #[test]
    fn test_opcode_fdcb_02() {
        test_jsmoo_file("fd cb __ 02")
    }
    #[test]
    fn test_opcode_fdcb_03() {
        test_jsmoo_file("fd cb __ 03")
    }
    #[test]
    fn test_opcode_fdcb_04() {
        test_jsmoo_file("fd cb __ 04")
    }
    #[test]
    fn test_opcode_fdcb_05() {
        test_jsmoo_file("fd cb __ 05")
    }
    #[test]
    fn test_opcode_fdcb_06() {
        test_jsmoo_file("fd cb __ 06")
    }
    #[test]
    fn test_opcode_fdcb_07() {
        test_jsmoo_file("fd cb __ 07")
    }
    #[test]
    fn test_opcode_fdcb_08() {
        test_jsmoo_file("fd cb __ 08")
    }
    #[test]
    fn test_opcode_fdcb_09() {
        test_jsmoo_file("fd cb __ 09")
    }
    #[test]
    fn test_opcode_fdcb_0a() {
        test_jsmoo_file("fd cb __ 0a")
    }
    #[test]
    fn test_opcode_fdcb_0b() {
        test_jsmoo_file("fd cb __ 0b")
    }
    #[test]
    fn test_opcode_fdcb_0c() {
        test_jsmoo_file("fd cb __ 0c")
    }
    #[test]
    fn test_opcode_fdcb_0d() {
        test_jsmoo_file("fd cb __ 0d")
    }
    #[test]
    fn test_opcode_fdcb_0e() {
        test_jsmoo_file("fd cb __ 0e")
    }
    #[test]
    fn test_opcode_fdcb_0f() {
        test_jsmoo_file("fd cb __ 0f")
    }
    #[test]
    fn test_opcode_fdcb_10() {
        test_jsmoo_file("fd cb __ 10")
    }
    #[test]
    fn test_opcode_fdcb_11() {
        test_jsmoo_file("fd cb __ 11")
    }
    #[test]
    fn test_opcode_fdcb_12() {
        test_jsmoo_file("fd cb __ 12")
    }
    #[test]
    fn test_opcode_fdcb_13() {
        test_jsmoo_file("fd cb __ 13")
    }
    #[test]
    fn test_opcode_fdcb_14() {
        test_jsmoo_file("fd cb __ 14")
    }
    #[test]
    fn test_opcode_fdcb_15() {
        test_jsmoo_file("fd cb __ 15")
    }
    #[test]
    fn test_opcode_fdcb_16() {
        test_jsmoo_file("fd cb __ 16")
    }
    #[test]
    fn test_opcode_fdcb_17() {
        test_jsmoo_file("fd cb __ 17")
    }
    #[test]
    fn test_opcode_fdcb_18() {
        test_jsmoo_file("fd cb __ 18")
    }
    #[test]
    fn test_opcode_fdcb_19() {
        test_jsmoo_file("fd cb __ 19")
    }
    #[test]
    fn test_opcode_fdcb_1a() {
        test_jsmoo_file("fd cb __ 1a")
    }
    #[test]
    fn test_opcode_fdcb_1b() {
        test_jsmoo_file("fd cb __ 1b")
    }
    #[test]
    fn test_opcode_fdcb_1c() {
        test_jsmoo_file("fd cb __ 1c")
    }
    #[test]
    fn test_opcode_fdcb_1d() {
        test_jsmoo_file("fd cb __ 1d")
    }
    #[test]
    fn test_opcode_fdcb_1e() {
        test_jsmoo_file("fd cb __ 1e")
    }
    #[test]
    fn test_opcode_fdcb_1f() {
        test_jsmoo_file("fd cb __ 1f")
    }
    #[test]
    fn test_opcode_fdcb_20() {
        test_jsmoo_file("fd cb __ 20")
    }
    #[test]
    fn test_opcode_fdcb_21() {
        test_jsmoo_file("fd cb __ 21")
    }
    #[test]
    fn test_opcode_fdcb_22() {
        test_jsmoo_file("fd cb __ 22")
    }
    #[test]
    fn test_opcode_fdcb_23() {
        test_jsmoo_file("fd cb __ 23")
    }
    #[test]
    fn test_opcode_fdcb_24() {
        test_jsmoo_file("fd cb __ 24")
    }
    #[test]
    fn test_opcode_fdcb_25() {
        test_jsmoo_file("fd cb __ 25")
    }
    #[test]
    fn test_opcode_fdcb_26() {
        test_jsmoo_file("fd cb __ 26")
    }
    #[test]
    fn test_opcode_fdcb_27() {
        test_jsmoo_file("fd cb __ 27")
    }
    #[test]
    fn test_opcode_fdcb_28() {
        test_jsmoo_file("fd cb __ 28")
    }
    #[test]
    fn test_opcode_fdcb_29() {
        test_jsmoo_file("fd cb __ 29")
    }
    #[test]
    fn test_opcode_fdcb_2a() {
        test_jsmoo_file("fd cb __ 2a")
    }
    #[test]
    fn test_opcode_fdcb_2b() {
        test_jsmoo_file("fd cb __ 2b")
    }
    #[test]
    fn test_opcode_fdcb_2c() {
        test_jsmoo_file("fd cb __ 2c")
    }
    #[test]
    fn test_opcode_fdcb_2d() {
        test_jsmoo_file("fd cb __ 2d")
    }
    #[test]
    fn test_opcode_fdcb_2e() {
        test_jsmoo_file("fd cb __ 2e")
    }
    #[test]
    fn test_opcode_fdcb_2f() {
        test_jsmoo_file("fd cb __ 2f")
    }
    #[test]
    fn test_opcode_fdcb_30() {
        test_jsmoo_file("fd cb __ 30")
    }
    #[test]
    fn test_opcode_fdcb_31() {
        test_jsmoo_file("fd cb __ 31")
    }
    #[test]
    fn test_opcode_fdcb_32() {
        test_jsmoo_file("fd cb __ 32")
    }
    #[test]
    fn test_opcode_fdcb_33() {
        test_jsmoo_file("fd cb __ 33")
    }
    #[test]
    fn test_opcode_fdcb_34() {
        test_jsmoo_file("fd cb __ 34")
    }
    #[test]
    fn test_opcode_fdcb_35() {
        test_jsmoo_file("fd cb __ 35")
    }
    #[test]
    fn test_opcode_fdcb_36() {
        test_jsmoo_file("fd cb __ 36")
    }
    #[test]
    fn test_opcode_fdcb_37() {
        test_jsmoo_file("fd cb __ 37")
    }
    #[test]
    fn test_opcode_fdcb_38() {
        test_jsmoo_file("fd cb __ 38")
    }
    #[test]
    fn test_opcode_fdcb_39() {
        test_jsmoo_file("fd cb __ 39")
    }
    #[test]
    fn test_opcode_fdcb_3a() {
        test_jsmoo_file("fd cb __ 3a")
    }
    #[test]
    fn test_opcode_fdcb_3b() {
        test_jsmoo_file("fd cb __ 3b")
    }
    #[test]
    fn test_opcode_fdcb_3c() {
        test_jsmoo_file("fd cb __ 3c")
    }
    #[test]
    fn test_opcode_fdcb_3d() {
        test_jsmoo_file("fd cb __ 3d")
    }
    #[test]
    fn test_opcode_fdcb_3e() {
        test_jsmoo_file("fd cb __ 3e")
    }
    #[test]
    fn test_opcode_fdcb_3f() {
        test_jsmoo_file("fd cb __ 3f")
    }
    #[test]
    fn test_opcode_fdcb_40() {
        test_jsmoo_file("fd cb __ 40")
    }
    #[test]
    fn test_opcode_fdcb_41() {
        test_jsmoo_file("fd cb __ 41")
    }
    #[test]
    fn test_opcode_fdcb_42() {
        test_jsmoo_file("fd cb __ 42")
    }
    #[test]
    fn test_opcode_fdcb_43() {
        test_jsmoo_file("fd cb __ 43")
    }
    #[test]
    fn test_opcode_fdcb_44() {
        test_jsmoo_file("fd cb __ 44")
    }
    #[test]
    fn test_opcode_fdcb_45() {
        test_jsmoo_file("fd cb __ 45")
    }
    #[test]
    fn test_opcode_fdcb_46() {
        test_jsmoo_file("fd cb __ 46")
    }
    #[test]
    fn test_opcode_fdcb_47() {
        test_jsmoo_file("fd cb __ 47")
    }
    #[test]
    fn test_opcode_fdcb_48() {
        test_jsmoo_file("fd cb __ 48")
    }
    #[test]
    fn test_opcode_fdcb_49() {
        test_jsmoo_file("fd cb __ 49")
    }
    #[test]
    fn test_opcode_fdcb_4a() {
        test_jsmoo_file("fd cb __ 4a")
    }
    #[test]
    fn test_opcode_fdcb_4b() {
        test_jsmoo_file("fd cb __ 4b")
    }
    #[test]
    fn test_opcode_fdcb_4c() {
        test_jsmoo_file("fd cb __ 4c")
    }
    #[test]
    fn test_opcode_fdcb_4d() {
        test_jsmoo_file("fd cb __ 4d")
    }
    #[test]
    fn test_opcode_fdcb_4e() {
        test_jsmoo_file("fd cb __ 4e")
    }
    #[test]
    fn test_opcode_fdcb_4f() {
        test_jsmoo_file("fd cb __ 4f")
    }
    #[test]
    fn test_opcode_fdcb_50() {
        test_jsmoo_file("fd cb __ 50")
    }
    #[test]
    fn test_opcode_fdcb_51() {
        test_jsmoo_file("fd cb __ 51")
    }
    #[test]
    fn test_opcode_fdcb_52() {
        test_jsmoo_file("fd cb __ 52")
    }
    #[test]
    fn test_opcode_fdcb_53() {
        test_jsmoo_file("fd cb __ 53")
    }
    #[test]
    fn test_opcode_fdcb_54() {
        test_jsmoo_file("fd cb __ 54")
    }
    #[test]
    fn test_opcode_fdcb_55() {
        test_jsmoo_file("fd cb __ 55")
    }
    #[test]
    fn test_opcode_fdcb_56() {
        test_jsmoo_file("fd cb __ 56")
    }
    #[test]
    fn test_opcode_fdcb_57() {
        test_jsmoo_file("fd cb __ 57")
    }
    #[test]
    fn test_opcode_fdcb_58() {
        test_jsmoo_file("fd cb __ 58")
    }
    #[test]
    fn test_opcode_fdcb_59() {
        test_jsmoo_file("fd cb __ 59")
    }
    #[test]
    fn test_opcode_fdcb_5a() {
        test_jsmoo_file("fd cb __ 5a")
    }
    #[test]
    fn test_opcode_fdcb_5b() {
        test_jsmoo_file("fd cb __ 5b")
    }
    #[test]
    fn test_opcode_fdcb_5c() {
        test_jsmoo_file("fd cb __ 5c")
    }
    #[test]
    fn test_opcode_fdcb_5d() {
        test_jsmoo_file("fd cb __ 5d")
    }
    #[test]
    fn test_opcode_fdcb_5e() {
        test_jsmoo_file("fd cb __ 5e")
    }
    #[test]
    fn test_opcode_fdcb_5f() {
        test_jsmoo_file("fd cb __ 5f")
    }
    #[test]
    fn test_opcode_fdcb_60() {
        test_jsmoo_file("fd cb __ 60")
    }
    #[test]
    fn test_opcode_fdcb_61() {
        test_jsmoo_file("fd cb __ 61")
    }
    #[test]
    fn test_opcode_fdcb_62() {
        test_jsmoo_file("fd cb __ 62")
    }
    #[test]
    fn test_opcode_fdcb_63() {
        test_jsmoo_file("fd cb __ 63")
    }
    #[test]
    fn test_opcode_fdcb_64() {
        test_jsmoo_file("fd cb __ 64")
    }
    #[test]
    fn test_opcode_fdcb_65() {
        test_jsmoo_file("fd cb __ 65")
    }
    #[test]
    fn test_opcode_fdcb_66() {
        test_jsmoo_file("fd cb __ 66")
    }
    #[test]
    fn test_opcode_fdcb_67() {
        test_jsmoo_file("fd cb __ 67")
    }
    #[test]
    fn test_opcode_fdcb_68() {
        test_jsmoo_file("fd cb __ 68")
    }
    #[test]
    fn test_opcode_fdcb_69() {
        test_jsmoo_file("fd cb __ 69")
    }
    #[test]
    fn test_opcode_fdcb_6a() {
        test_jsmoo_file("fd cb __ 6a")
    }
    #[test]
    fn test_opcode_fdcb_6b() {
        test_jsmoo_file("fd cb __ 6b")
    }
    #[test]
    fn test_opcode_fdcb_6c() {
        test_jsmoo_file("fd cb __ 6c")
    }
    #[test]
    fn test_opcode_fdcb_6d() {
        test_jsmoo_file("fd cb __ 6d")
    }
    #[test]
    fn test_opcode_fdcb_6e() {
        test_jsmoo_file("fd cb __ 6e")
    }
    #[test]
    fn test_opcode_fdcb_6f() {
        test_jsmoo_file("fd cb __ 6f")
    }
    #[test]
    fn test_opcode_fdcb_70() {
        test_jsmoo_file("fd cb __ 70")
    }
    #[test]
    fn test_opcode_fdcb_71() {
        test_jsmoo_file("fd cb __ 71")
    }
    #[test]
    fn test_opcode_fdcb_72() {
        test_jsmoo_file("fd cb __ 72")
    }
    #[test]
    fn test_opcode_fdcb_73() {
        test_jsmoo_file("fd cb __ 73")
    }
    #[test]
    fn test_opcode_fdcb_74() {
        test_jsmoo_file("fd cb __ 74")
    }
    #[test]
    fn test_opcode_fdcb_75() {
        test_jsmoo_file("fd cb __ 75")
    }
    #[test]
    fn test_opcode_fdcb_76() {
        test_jsmoo_file("fd cb __ 76")
    }
    #[test]
    fn test_opcode_fdcb_77() {
        test_jsmoo_file("fd cb __ 77")
    }
    #[test]
    fn test_opcode_fdcb_78() {
        test_jsmoo_file("fd cb __ 78")
    }
    #[test]
    fn test_opcode_fdcb_79() {
        test_jsmoo_file("fd cb __ 79")
    }
    #[test]
    fn test_opcode_fdcb_7a() {
        test_jsmoo_file("fd cb __ 7a")
    }
    #[test]
    fn test_opcode_fdcb_7b() {
        test_jsmoo_file("fd cb __ 7b")
    }
    #[test]
    fn test_opcode_fdcb_7c() {
        test_jsmoo_file("fd cb __ 7c")
    }
    #[test]
    fn test_opcode_fdcb_7d() {
        test_jsmoo_file("fd cb __ 7d")
    }
    #[test]
    fn test_opcode_fdcb_7e() {
        test_jsmoo_file("fd cb __ 7e")
    }
    #[test]
    fn test_opcode_fdcb_7f() {
        test_jsmoo_file("fd cb __ 7f")
    }
    #[test]
    fn test_opcode_fdcb_80() {
        test_jsmoo_file("fd cb __ 80")
    }
    #[test]
    fn test_opcode_fdcb_81() {
        test_jsmoo_file("fd cb __ 81")
    }
    #[test]
    fn test_opcode_fdcb_82() {
        test_jsmoo_file("fd cb __ 82")
    }
    #[test]
    fn test_opcode_fdcb_83() {
        test_jsmoo_file("fd cb __ 83")
    }
    #[test]
    fn test_opcode_fdcb_84() {
        test_jsmoo_file("fd cb __ 84")
    }
    #[test]
    fn test_opcode_fdcb_85() {
        test_jsmoo_file("fd cb __ 85")
    }
    #[test]
    fn test_opcode_fdcb_86() {
        test_jsmoo_file("fd cb __ 86")
    }
    #[test]
    fn test_opcode_fdcb_87() {
        test_jsmoo_file("fd cb __ 87")
    }
    #[test]
    fn test_opcode_fdcb_88() {
        test_jsmoo_file("fd cb __ 88")
    }
    #[test]
    fn test_opcode_fdcb_89() {
        test_jsmoo_file("fd cb __ 89")
    }
    #[test]
    fn test_opcode_fdcb_8a() {
        test_jsmoo_file("fd cb __ 8a")
    }
    #[test]
    fn test_opcode_fdcb_8b() {
        test_jsmoo_file("fd cb __ 8b")
    }
    #[test]
    fn test_opcode_fdcb_8c() {
        test_jsmoo_file("fd cb __ 8c")
    }
    #[test]
    fn test_opcode_fdcb_8d() {
        test_jsmoo_file("fd cb __ 8d")
    }
    #[test]
    fn test_opcode_fdcb_8e() {
        test_jsmoo_file("fd cb __ 8e")
    }
    #[test]
    fn test_opcode_fdcb_8f() {
        test_jsmoo_file("fd cb __ 8f")
    }
    #[test]
    fn test_opcode_fdcb_90() {
        test_jsmoo_file("fd cb __ 90")
    }
    #[test]
    fn test_opcode_fdcb_91() {
        test_jsmoo_file("fd cb __ 91")
    }
    #[test]
    fn test_opcode_fdcb_92() {
        test_jsmoo_file("fd cb __ 92")
    }
    #[test]
    fn test_opcode_fdcb_93() {
        test_jsmoo_file("fd cb __ 93")
    }
    #[test]
    fn test_opcode_fdcb_94() {
        test_jsmoo_file("fd cb __ 94")
    }
    #[test]
    fn test_opcode_fdcb_95() {
        test_jsmoo_file("fd cb __ 95")
    }
    #[test]
    fn test_opcode_fdcb_96() {
        test_jsmoo_file("fd cb __ 96")
    }
    #[test]
    fn test_opcode_fdcb_97() {
        test_jsmoo_file("fd cb __ 97")
    }
    #[test]
    fn test_opcode_fdcb_98() {
        test_jsmoo_file("fd cb __ 98")
    }
    #[test]
    fn test_opcode_fdcb_99() {
        test_jsmoo_file("fd cb __ 99")
    }
    #[test]
    fn test_opcode_fdcb_9a() {
        test_jsmoo_file("fd cb __ 9a")
    }
    #[test]
    fn test_opcode_fdcb_9b() {
        test_jsmoo_file("fd cb __ 9b")
    }
    #[test]
    fn test_opcode_fdcb_9c() {
        test_jsmoo_file("fd cb __ 9c")
    }
    #[test]
    fn test_opcode_fdcb_9d() {
        test_jsmoo_file("fd cb __ 9d")
    }
    #[test]
    fn test_opcode_fdcb_9e() {
        test_jsmoo_file("fd cb __ 9e")
    }
    #[test]
    fn test_opcode_fdcb_9f() {
        test_jsmoo_file("fd cb __ 9f")
    }
    #[test]
    fn test_opcode_fdcb_a0() {
        test_jsmoo_file("fd cb __ a0")
    }
    #[test]
    fn test_opcode_fdcb_a1() {
        test_jsmoo_file("fd cb __ a1")
    }
    #[test]
    fn test_opcode_fdcb_a2() {
        test_jsmoo_file("fd cb __ a2")
    }
    #[test]
    fn test_opcode_fdcb_a3() {
        test_jsmoo_file("fd cb __ a3")
    }
    #[test]
    fn test_opcode_fdcb_a4() {
        test_jsmoo_file("fd cb __ a4")
    }
    #[test]
    fn test_opcode_fdcb_a5() {
        test_jsmoo_file("fd cb __ a5")
    }
    #[test]
    fn test_opcode_fdcb_a6() {
        test_jsmoo_file("fd cb __ a6")
    }
    #[test]
    fn test_opcode_fdcb_a7() {
        test_jsmoo_file("fd cb __ a7")
    }
    #[test]
    fn test_opcode_fdcb_a8() {
        test_jsmoo_file("fd cb __ a8")
    }
    #[test]
    fn test_opcode_fdcb_a9() {
        test_jsmoo_file("fd cb __ a9")
    }
    #[test]
    fn test_opcode_fdcb_aa() {
        test_jsmoo_file("fd cb __ aa")
    }
    #[test]
    fn test_opcode_fdcb_ab() {
        test_jsmoo_file("fd cb __ ab")
    }
    #[test]
    fn test_opcode_fdcb_ac() {
        test_jsmoo_file("fd cb __ ac")
    }
    #[test]
    fn test_opcode_fdcb_ad() {
        test_jsmoo_file("fd cb __ ad")
    }
    #[test]
    fn test_opcode_fdcb_ae() {
        test_jsmoo_file("fd cb __ ae")
    }
    #[test]
    fn test_opcode_fdcb_af() {
        test_jsmoo_file("fd cb __ af")
    }
    #[test]
    fn test_opcode_fdcb_b0() {
        test_jsmoo_file("fd cb __ b0")
    }
    #[test]
    fn test_opcode_fdcb_b1() {
        test_jsmoo_file("fd cb __ b1")
    }
    #[test]
    fn test_opcode_fdcb_b2() {
        test_jsmoo_file("fd cb __ b2")
    }
    #[test]
    fn test_opcode_fdcb_b3() {
        test_jsmoo_file("fd cb __ b3")
    }
    #[test]
    fn test_opcode_fdcb_b4() {
        test_jsmoo_file("fd cb __ b4")
    }
    #[test]
    fn test_opcode_fdcb_b5() {
        test_jsmoo_file("fd cb __ b5")
    }
    #[test]
    fn test_opcode_fdcb_b6() {
        test_jsmoo_file("fd cb __ b6")
    }
    #[test]
    fn test_opcode_fdcb_b7() {
        test_jsmoo_file("fd cb __ b7")
    }
    #[test]
    fn test_opcode_fdcb_b8() {
        test_jsmoo_file("fd cb __ b8")
    }
    #[test]
    fn test_opcode_fdcb_b9() {
        test_jsmoo_file("fd cb __ b9")
    }
    #[test]
    fn test_opcode_fdcb_ba() {
        test_jsmoo_file("fd cb __ ba")
    }
    #[test]
    fn test_opcode_fdcb_bb() {
        test_jsmoo_file("fd cb __ bb")
    }
    #[test]
    fn test_opcode_fdcb_bc() {
        test_jsmoo_file("fd cb __ bc")
    }
    #[test]
    fn test_opcode_fdcb_bd() {
        test_jsmoo_file("fd cb __ bd")
    }
    #[test]
    fn test_opcode_fdcb_be() {
        test_jsmoo_file("fd cb __ be")
    }
    #[test]
    fn test_opcode_fdcb_bf() {
        test_jsmoo_file("fd cb __ bf")
    }
    #[test]
    fn test_opcode_fdcb_c0() {
        test_jsmoo_file("fd cb __ c0")
    }
    #[test]
    fn test_opcode_fdcb_c1() {
        test_jsmoo_file("fd cb __ c1")
    }
    #[test]
    fn test_opcode_fdcb_c2() {
        test_jsmoo_file("fd cb __ c2")
    }
    #[test]
    fn test_opcode_fdcb_c3() {
        test_jsmoo_file("fd cb __ c3")
    }
    #[test]
    fn test_opcode_fdcb_c4() {
        test_jsmoo_file("fd cb __ c4")
    }
    #[test]
    fn test_opcode_fdcb_c5() {
        test_jsmoo_file("fd cb __ c5")
    }
    #[test]
    fn test_opcode_fdcb_c6() {
        test_jsmoo_file("fd cb __ c6")
    }
    #[test]
    fn test_opcode_fdcb_c7() {
        test_jsmoo_file("fd cb __ c7")
    }
    #[test]
    fn test_opcode_fdcb_c8() {
        test_jsmoo_file("fd cb __ c8")
    }
    #[test]
    fn test_opcode_fdcb_c9() {
        test_jsmoo_file("fd cb __ c9")
    }
    #[test]
    fn test_opcode_fdcb_ca() {
        test_jsmoo_file("fd cb __ ca")
    }
    #[test]
    fn test_opcode_fdcb_cb() {
        test_jsmoo_file("fd cb __ cb")
    }
    #[test]
    fn test_opcode_fdcb_cc() {
        test_jsmoo_file("fd cb __ cc")
    }
    #[test]
    fn test_opcode_fdcb_cd() {
        test_jsmoo_file("fd cb __ cd")
    }
    #[test]
    fn test_opcode_fdcb_ce() {
        test_jsmoo_file("fd cb __ ce")
    }
    #[test]
    fn test_opcode_fdcb_cf() {
        test_jsmoo_file("fd cb __ cf")
    }
    #[test]
    fn test_opcode_fdcb_d0() {
        test_jsmoo_file("fd cb __ d0")
    }
    #[test]
    fn test_opcode_fdcb_d1() {
        test_jsmoo_file("fd cb __ d1")
    }
    #[test]
    fn test_opcode_fdcb_d2() {
        test_jsmoo_file("fd cb __ d2")
    }
    #[test]
    fn test_opcode_fdcb_d3() {
        test_jsmoo_file("fd cb __ d3")
    }
    #[test]
    fn test_opcode_fdcb_d4() {
        test_jsmoo_file("fd cb __ d4")
    }
    #[test]
    fn test_opcode_fdcb_d5() {
        test_jsmoo_file("fd cb __ d5")
    }
    #[test]
    fn test_opcode_fdcb_d6() {
        test_jsmoo_file("fd cb __ d6")
    }
    #[test]
    fn test_opcode_fdcb_d7() {
        test_jsmoo_file("fd cb __ d7")
    }
    #[test]
    fn test_opcode_fdcb_d8() {
        test_jsmoo_file("fd cb __ d8")
    }
    #[test]
    fn test_opcode_fdcb_d9() {
        test_jsmoo_file("fd cb __ d9")
    }
    #[test]
    fn test_opcode_fdcb_da() {
        test_jsmoo_file("fd cb __ da")
    }
    #[test]
    fn test_opcode_fdcb_db() {
        test_jsmoo_file("fd cb __ db")
    }
    #[test]
    fn test_opcode_fdcb_dc() {
        test_jsmoo_file("fd cb __ dc")
    }
    #[test]
    fn test_opcode_fdcb_dd() {
        test_jsmoo_file("fd cb __ dd")
    }
    #[test]
    fn test_opcode_fdcb_de() {
        test_jsmoo_file("fd cb __ de")
    }
    #[test]
    fn test_opcode_fdcb_df() {
        test_jsmoo_file("fd cb __ df")
    }
    #[test]
    fn test_opcode_fdcb_e0() {
        test_jsmoo_file("fd cb __ e0")
    }
    #[test]
    fn test_opcode_fdcb_e1() {
        test_jsmoo_file("fd cb __ e1")
    }
    #[test]
    fn test_opcode_fdcb_e2() {
        test_jsmoo_file("fd cb __ e2")
    }
    #[test]
    fn test_opcode_fdcb_e3() {
        test_jsmoo_file("fd cb __ e3")
    }
    #[test]
    fn test_opcode_fdcb_e4() {
        test_jsmoo_file("fd cb __ e4")
    }
    #[test]
    fn test_opcode_fdcb_e5() {
        test_jsmoo_file("fd cb __ e5")
    }
    #[test]
    fn test_opcode_fdcb_e6() {
        test_jsmoo_file("fd cb __ e6")
    }
    #[test]
    fn test_opcode_fdcb_e7() {
        test_jsmoo_file("fd cb __ e7")
    }
    #[test]
    fn test_opcode_fdcb_e8() {
        test_jsmoo_file("fd cb __ e8")
    }
    #[test]
    fn test_opcode_fdcb_e9() {
        test_jsmoo_file("fd cb __ e9")
    }
    #[test]
    fn test_opcode_fdcb_ea() {
        test_jsmoo_file("fd cb __ ea")
    }
    #[test]
    fn test_opcode_fdcb_eb() {
        test_jsmoo_file("fd cb __ eb")
    }
    #[test]
    fn test_opcode_fdcb_ec() {
        test_jsmoo_file("fd cb __ ec")
    }
    #[test]
    fn test_opcode_fdcb_ed() {
        test_jsmoo_file("fd cb __ ed")
    }
    #[test]
    fn test_opcode_fdcb_ee() {
        test_jsmoo_file("fd cb __ ee")
    }
    #[test]
    fn test_opcode_fdcb_ef() {
        test_jsmoo_file("fd cb __ ef")
    }
    #[test]
    fn test_opcode_fdcb_f0() {
        test_jsmoo_file("fd cb __ f0")
    }
    #[test]
    fn test_opcode_fdcb_f1() {
        test_jsmoo_file("fd cb __ f1")
    }
    #[test]
    fn test_opcode_fdcb_f2() {
        test_jsmoo_file("fd cb __ f2")
    }
    #[test]
    fn test_opcode_fdcb_f3() {
        test_jsmoo_file("fd cb __ f3")
    }
    #[test]
    fn test_opcode_fdcb_f4() {
        test_jsmoo_file("fd cb __ f4")
    }
    #[test]
    fn test_opcode_fdcb_f5() {
        test_jsmoo_file("fd cb __ f5")
    }
    #[test]
    fn test_opcode_fdcb_f6() {
        test_jsmoo_file("fd cb __ f6")
    }
    #[test]
    fn test_opcode_fdcb_f7() {
        test_jsmoo_file("fd cb __ f7")
    }
    #[test]
    fn test_opcode_fdcb_f8() {
        test_jsmoo_file("fd cb __ f8")
    }
    #[test]
    fn test_opcode_fdcb_f9() {
        test_jsmoo_file("fd cb __ f9")
    }
    #[test]
    fn test_opcode_fdcb_fa() {
        test_jsmoo_file("fd cb __ fa")
    }
    #[test]
    fn test_opcode_fdcb_fb() {
        test_jsmoo_file("fd cb __ fb")
    }
    #[test]
    fn test_opcode_fdcb_fc() {
        test_jsmoo_file("fd cb __ fc")
    }
    #[test]
    fn test_opcode_fdcb_fd() {
        test_jsmoo_file("fd cb __ fd")
    }
    #[test]
    fn test_opcode_fdcb_fe() {
        test_jsmoo_file("fd cb __ fe")
    }
    #[test]
    fn test_opcode_fdcb_ff() {
        test_jsmoo_file("fd cb __ ff")
    }

    fn test_jsmoo_file(fname: &str) {
        struct IO {
            memory: [u8; 0x10000],
            io: [u8; 0x10000],
            ports: Vec<(u16, u8, char)>,
            expected_ports: Vec<(u16, u8, char)>,
        }
        impl Z80IO for IO {
            fn peek_byte(&mut self, addr: u16, _: bool) -> u8 {
                self.memory[addr as usize]
            }

            fn write_byte(&mut self, addr: u16, val: u8, _: bool) {
                self.memory[addr as usize] = val;
            }

            fn peek_io(&mut self, addr: u16, _: bool) -> u8 {
                self.io[addr as usize]
            }

            fn write_io(&mut self, addr: u16, val: u8, _: bool) {
                self.ports.push((addr, val, 'w'));
            }
        }

        let tests = load_file(format!("tests/v1/{}.json", fname));
        for test in &tests {
            println!("Test: {}", test.name);
            let initial = &test.initial;
            let mut io = IO {
                memory: [0; 0x10000],
                io: [0; 0x10000],
                ports: vec![],
                expected_ports: vec![],
            };
            let mut cpu = Z80::new(true);
            cpu.pc = initial.pc;
            cpu.sp = initial.sp;
            cpu.a = initial.a;
            cpu.b = initial.b;
            cpu.c = initial.c;
            cpu.d = initial.d;
            cpu.e = initial.e;
            cpu.f = initial.f;
            cpu.h = initial.h;
            cpu.l = initial.l;
            cpu.i = initial.i;
            cpu.r = initial.r;
            cpu.ei = initial.ei;
            cpu.wz = initial.wz;
            cpu.ix = initial.ix;
            cpu.iy = initial.iy;
            cpu.af_ = initial.af_;
            cpu.bc_ = initial.bc_;
            cpu.de_ = initial.de_;
            cpu.hl_ = initial.hl_;
            cpu.im = initial.im;
            cpu.p = initial.p;
            cpu.q = initial.q;
            cpu.iff1 = initial.iff1;
            cpu.iff2 = initial.iff2;
            for i in 0..initial.ram.len() {
                let (addr, val) = initial.ram[i];
                io.memory[addr as usize] = val;
            }
            match &test.ports {
                None => (),
                Some(ports) => {
                    for i in 0..ports.len() {
                        let (addr, val, rw) = ports[i];
                        if rw == 'r' {
                            io.io[addr as usize] = val;
                        } else {
                            io.expected_ports.push((addr, val, rw));
                        }
                    }
                }
            }

            // Fetch then tick execution
            let mut cycles: Vec<(Option<u16>, Option<u8>, String)> = vec![];
            cpu.tick(&mut io);
            cycles.push((cpu.addr_bus, cpu.data_bus, cpu.pin_state()));
            while cpu.phase != FDEPhase::Init {
                cpu.tick(&mut io);
                let cycle = (cpu.addr_bus, cpu.data_bus, cpu.pin_state());
                cycles.push(cycle);
            }

            // Check against final state
            let ffinal = &test.r#final;
            println!(
                "initial: {:02x} {:02x}, curr: {:02x} {:02x} expected: {:02x} {:02x}",
                initial.a, initial.b, cpu.a, cpu.f, ffinal.a, ffinal.f
            );
            assert_eq!(cycles, test.cycles);
            assert_eq!(cpu.pc, ffinal.pc);
            assert_eq!(cpu.sp, ffinal.sp);
            assert_eq!(cpu.a, ffinal.a);
            assert_eq!(cpu.b, ffinal.b);
            assert_eq!(cpu.c, ffinal.c);
            assert_eq!(cpu.d, ffinal.d);
            assert_eq!(cpu.e, ffinal.e);
            assert_eq!(cpu.f, ffinal.f);
            assert_eq!(cpu.h, ffinal.h);
            assert_eq!(cpu.l, ffinal.l);
            assert_eq!(cpu.i, ffinal.i);
            assert_eq!(cpu.r, ffinal.r);
            assert_eq!(cpu.ei, ffinal.ei);
            assert_eq!(cpu.wz, ffinal.wz);
            assert_eq!(cpu.ix, ffinal.ix);
            assert_eq!(cpu.iy, ffinal.iy);
            assert_eq!(cpu.af_, ffinal.af_);
            assert_eq!(cpu.bc_, ffinal.bc_);
            assert_eq!(cpu.de_, ffinal.de_);
            assert_eq!(cpu.hl_, ffinal.hl_);
            assert_eq!(cpu.im, ffinal.im);
            assert_eq!(cpu.p, ffinal.p);
            assert_eq!(cpu.q, ffinal.q);
            assert_eq!(cpu.iff1, ffinal.iff1);
            assert_eq!(cpu.iff2, ffinal.iff2);
            for i in 0..ffinal.ram.len() {
                let (addr, val) = ffinal.ram[i];
                assert_eq!(io.peek_byte(addr, true), val);
            }
            match &test.ports {
                None => (),
                Some(_) => {
                    assert_eq!(io.ports, io.expected_ports);
                }
            }
        }
    }

    #[test]
    fn test_prelim() {
        test_cpm("prelim.com", "prelim.txt");
    }
    #[test]
    fn test_zexall() {
        test_cpm("zexall.cim", "zexall.txt");
    }
    #[test]
    fn test_zexdoc() {
        test_cpm("zexdoc.cim", "zexdoc.txt");
    }

    fn test_cpm(fname: &str, expected: &str) {
        struct IO {
            memory: [u8; 0x10000],
            done: bool,
            c: u8,
            d: u8,
            e: u8,
            printed_bytes: Vec<u8>,
        }
        impl Z80IO for IO {
            fn peek_byte(&mut self, addr: u16, _: bool) -> u8 {
                self.memory[addr as usize]
            }

            fn write_byte(&mut self, addr: u16, val: u8, _: bool) {
                self.memory[addr as usize] = val;
            }

            fn peek_io(&mut self, _addr: u16, _: bool) -> u8 {
                match self.c {
                    2 => self.printed_bytes.push(self.e),
                    9 => {
                        let mut addr = ((self.d as u16) << 8) | (self.e as u16);
                        loop {
                            let ch = self.memory[addr as usize];
                            addr = addr.wrapping_add(1);
                            if ch as char == '$' {
                                break;
                            }
                            self.printed_bytes.push(ch);
                        }
                    }
                    _ => (),
                }
                0xff
            }

            fn write_io(&mut self, _addr: u16, _val: u8, _: bool) {
                self.done = true;
            }
        }

        let rom = get_file_as_byte_vec(&format!("tests/z80/{}", fname));
        let mut io = IO {
            memory: [0; 0x10000],
            done: false,
            c: 0,
            d: 0,
            e: 0,
            printed_bytes: vec![],
        };
        let mut cpu = Z80::new(true);
        for i in 0..rom.len() {
            io.memory[i + 0x100] = rom[i];
        }
        cpu.pc = 0x100;
        io.memory[0] = 0xd3;
        io.memory[5] = 0xdb;
        io.memory[7] = 0xc9;
        loop {
            cpu.step(&mut io);
            io.c = cpu.c;
            io.d = cpu.d;
            io.e = cpu.e;
            if io.done {
                break;
            }
        }
        let expected_txt = get_file_as_byte_vec(&format!("tests/z80/{}", expected));
        let mut new_expected: Vec<u8> = vec![];
        for b in expected_txt {
            new_expected.push(b);
            if b == 10 {
                new_expected.push(13);
            }
        }
        assert_eq!(io.printed_bytes, new_expected);
        let final_str = new_expected
            .into_iter()
            .map(|val| val as char)
            .collect::<String>();
        println!("{}", final_str);
    }
}
