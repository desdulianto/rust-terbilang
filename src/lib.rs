type Terbilang = Result<String, String>;

struct ValueLabel{value: u32, label: &'static str}

// HARUS diurutkan dari yang terbesar ke yang terkecil
static DENOMINASI: [ValueLabel; 5] = [
    ValueLabel{value: 1_000_000_000, label: "milyar"},
    ValueLabel{value: 1_000_000, label: "juta"},
    ValueLabel{value: 1_000, label: "ribu"},
    ValueLabel{value: 100, label: "ratus"},
    ValueLabel{value: 10, label: "puluh"},
];

static SATUAN: [ValueLabel; 10] = [
    ValueLabel{value: 0, label: "nol"},
    ValueLabel{value: 1, label: "satu"},
    ValueLabel{value: 2, label: "dua"},
    ValueLabel{value: 3, label: "tiga"},
    ValueLabel{value: 4, label: "empat"},
    ValueLabel{value: 5, label: "lima"},
    ValueLabel{value: 6, label: "enam"},
    ValueLabel{value: 7, label: "tujuh"},
    ValueLabel{value: 8, label: "delapan"},
    ValueLabel{value: 9, label: "sembilan"},
];

fn satuan(number: &u32) -> Terbilang {
    match SATUAN.iter().find(|x| x.value == *number) {
        Some(n) => Ok(String::from(n.label)),
        None => Err(String::from("satuan digit not found")),
    }
}

fn belasan(number: &u32) -> Terbilang {
    let mut s = terbilang(&(number % 10))?;
    s.push_str(" belas");
    Ok(s.replace("satu belas", "sebelas"))
}

fn other(number: &u32) -> Terbilang {
    for denom in DENOMINASI.iter() {
        if *number >= denom.value {
            let mut s = terbilang(&(number / denom.value))?;
            s.push_str(" ");
            s.push_str(denom.label);
            if *number % denom.value != 0 {
                s.push_str(" ");
                let s1 = terbilang(&(number % denom.value))?;
                s.push_str(s1.as_str());
            }

            return Ok(s.replace("satu ribu", "seribu")
                .replace("satu ratus", "seratus")
                .replace("satu puluh", "sepuluh"));
        }
    }
    Err(String::from("number out of range"))
}

pub fn terbilang(number: &u32) -> Terbilang {
    match *number {
        0..=9 => satuan(&number),
        11..=19 => belasan(&number),
        _ => other(&number),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terbilang_it_works() {
        let tests = [
            (0, "nol"),
            (1, "satu"),
            (2, "dua"),
            (3, "tiga"),
            (4, "empat"),
            (5, "lima"),
            (6, "enam"),
            (7, "tujuh"),
            (8, "delapan"),
            (9, "sembilan"),
            (10, "sepuluh"),
            (11, "sebelas"),
            (12, "dua belas"),
            (13, "tiga belas"),
            (14, "empat belas"),
            (15, "lima belas"),
            (16, "enam belas"),
            (17, "tujuh belas"),
            (18, "delapan belas"),
            (19, "sembilan belas"),
            (20, "dua puluh"),
            (29, "dua puluh sembilan"),
            (99, "sembilan puluh sembilan"),
            (100, "seratus"),
            (111, "seratus sebelas"),
            (119, "seratus sembilan belas"),
            (220, "dua ratus dua puluh"),
            (1000, "seribu"),
            (1019, "seribu sembilan belas"),
            (1119, "seribu seratus sembilan belas"),
            (2220, "dua ribu dua ratus dua puluh"),
            (12220, "dua belas ribu dua ratus dua puluh"),
            (22220, "dua puluh dua ribu dua ratus dua puluh"),
            (222220, "dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (2222220, "dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (22222220, "dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (222222220, "dua ratus dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (987654321, "sembilan ratus delapan puluh tujuh juta enam ratus lima puluh empat ribu tiga ratus dua puluh satu"),
            (2222222220, "dua milyar dua ratus dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (1234567890, "satu milyar dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"),
            ];
        
        for test in tests.iter() {
            assert_eq!(terbilang(&test.0), Ok(String::from(test.1)));
        }
    }
}
