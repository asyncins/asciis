use std::collections::HashMap;
pub struct Asciis{}
impl Asciis{
    fn init (&self) -> HashMap<String, i32>{
        let contrast: HashMap<String, i32> = [
            (String::from("SOH"), 1),
            (String::from("SOX"), 2),
            (String::from("ETX"), 3),
            (String::from("EOT"), 4),
            (String::from("ENQ"), 5),
            (String::from("ACK"), 6),
            (String::from("BEL"), 7),
            (String::from("BS"), 8),
            (String::from("HT"), 9),
            (String::from("LF"), 10),
            (String::from("VT"), 11),
            (String::from("FF"), 12),
            (String::from("CR"), 13),
            (String::from("SI"), 14),
            (String::from("SO"), 15),
            (String::from("DLE"), 16),
            (String::from("DC1"), 17),
            (String::from("DC2"), 18),
            (String::from("DC3"), 19),
            (String::from("DC4"), 20),
            (String::from("NAK"), 21),
            (String::from("SYN"), 22),
            (String::from("ETB"), 23),
            (String::from("CAN"), 24),
            (String::from("EM"), 25),
            (String::from("SUB"), 26),
            (String::from("ESC"), 27),
            (String::from("FS"), 28),
            (String::from("GS"), 29),
            (String::from("RS"), 30),
            (String::from("US"), 31),
            (String::from("DEL"), 127),
            (String::from(" "), 32), // SP
            (String::from("!"), 33),
            (String::from("\""), 34),
            (String::from("#"), 35),
            (String::from("$"), 36),
            (String::from("%"), 37),
            (String::from("&"), 38),
            (String::from("'"), 39),
            (String::from("("), 40),
            (String::from(")"), 41),
            (String::from("*"), 42),
            (String::from("+"), 43),
            (String::from(","), 44),
            (String::from("-"), 45),
            (String::from("."), 46),
            (String::from("/"), 47),
            (String::from("0"), 48),
            (String::from("1"), 49),
            (String::from("2"), 50),
            (String::from("3"), 51),
            (String::from("4"), 52),
            (String::from("5"), 53),
            (String::from("6"), 54),
            (String::from("7"), 55),
            (String::from("8"), 56),
            (String::from("9"), 57),
            (String::from(":"), 58),
            (String::from(";"), 59),
            (String::from("<"), 60),
            (String::from("="), 61),
            (String::from(">"), 62),
            (String::from("?"), 63),
            (String::from("@"), 64),
            (String::from("A"), 65),
            (String::from("B"), 66),
            (String::from("C"), 67),
            (String::from("D"), 68),
            (String::from("E"), 69),
            (String::from("F"), 70),
            (String::from("G"), 71),
            (String::from("H"), 72),
            (String::from("I"), 73),
            (String::from("J"), 74),
            (String::from("K"), 75),
            (String::from("L"), 76),
            (String::from("M"), 77),
            (String::from("N"), 78),
            (String::from("O"), 79),
            (String::from("P"), 80),
            (String::from("Q"), 81),
            (String::from("R"), 82),
            (String::from("S"), 83),
            (String::from("T"), 84),
            (String::from("U"), 85),
            (String::from("V"), 86),
            (String::from("W"), 87),
            (String::from("X"), 88),
            (String::from("Y"), 89),
            (String::from("Z"), 90),
            (String::from("["), 91),
            (String::from(r"\"), 92),
            (String::from("]"), 93),
            (String::from("^"), 94),
            (String::from("_"), 95),
            (String::from("`"), 96),
            (String::from("a"), 97),
            (String::from("b"), 98),
            (String::from("c"), 99),
            (String::from("d"), 100),
            (String::from("e"), 101),
            (String::from("f"), 102),
            (String::from("g"), 103),
            (String::from("h"), 104),
            (String::from("i"), 105),
            (String::from("j"), 106),
            (String::from("k"), 107),
            (String::from("l"), 108),
            (String::from("m"), 109),
            (String::from("n"), 110),
            (String::from("o"), 111),
            (String::from("p"), 112),
            (String::from("q"), 113),
            (String::from("r"), 114),
            (String::from("s"), 115),
            (String::from("t"), 116),
            (String::from("u"), 117),
            (String::from("v"), 118),
            (String::from("w"), 119),
            (String::from("x"), 120),
            (String::from("y"), 121),
            (String::from("z"), 122),
            (String::from("{"), 123),
            (String::from("|"), 124),
            (String::from("}"), 125),
            (String::from("~"), 126),
        ].iter().cloned().collect();
        contrast
    }

    pub fn ord(&self, value: &str) -> Option<i32>{
        let hmp = self.init();
        let res = hmp.get(value);
        match res{
            Some(n) => Some(*res.unwrap()),
            None => None
        }
    }

    pub fn chr(&self, value: i32) -> Option<String>{
        let hmp = self.init();
        for (key, val) in hmp.iter().enumerate(){
            if value == *val.1{
                return Some(val.0.to_string());
                break;
            }
        }
        None
    }
}


#[cfg(test)]
mod tests{
     use super::Asciis;

     #[test]
     fn ord(){
         let asc = Asciis{};
         let r = asc.ord("s");
         assert_eq!(r, Some(115));
     }

    #[test]
    fn chr(){
        let asc = Asciis{};
        let v = asc.chr(97);
        assert_eq!(v, Some(String::from("a")));
    }
}




