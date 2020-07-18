struct ValueLabel {
    value: i64,
    label: &'static str,
}

// HARUS diurutkan dari yang terbesar ke yang terkecil
static DENOMINASI: [ValueLabel; 6] = [
    ValueLabel {
        value: 1_000_000_000_000,
        label: "triliun",
    },
    ValueLabel {
        value: 1_000_000_000,
        label: "milyar",
    },
    ValueLabel {
        value: 1_000_000,
        label: "juta",
    },
    ValueLabel {
        value: 1_000,
        label: "ribu",
    },
    ValueLabel {
        value: 100,
        label: "ratus",
    },
    ValueLabel {
        value: 10,
        label: "puluh",
    },
];

static SATUAN: [ValueLabel; 12] = [
    ValueLabel {
        value: 0,
        label: "nol",
    },
    ValueLabel {
        value: 1,
        label: "satu",
    },
    ValueLabel {
        value: 2,
        label: "dua",
    },
    ValueLabel {
        value: 3,
        label: "tiga",
    },
    ValueLabel {
        value: 4,
        label: "empat",
    },
    ValueLabel {
        value: 5,
        label: "lima",
    },
    ValueLabel {
        value: 6,
        label: "enam",
    },
    ValueLabel {
        value: 7,
        label: "tujuh",
    },
    ValueLabel {
        value: 8,
        label: "delapan",
    },
    ValueLabel {
        value: 9,
        label: "sembilan",
    },
    ValueLabel {
        value: 10,
        label: "sepuluh",
    },
    ValueLabel {
        value: 11,
        label: "sebelas",
    },
];

fn satuan(number: &i64) -> Result<String, String> {
    match SATUAN.iter().find(|x| x.value == *number) {
        Some(n) => Ok(String::from(n.label)),
        None => Err(format!("weird number found: {}", number)),
    }
}

fn belasan(number: &i64) -> Result<String, String> {
    Ok(format!("{} belas", terbilang_helper(&(number % 10))?))
}

fn other(number: &i64) -> Result<String, String> {
    for denom in DENOMINASI.iter() {
        if *number >= denom.value {
            let s = format!(
                "{} {}",
                terbilang_helper(&(number / denom.value))?,
                denom.label
            );
            let s = match *number % denom.value == 0 {
                true => s,
                false => format!("{} {}", s, terbilang_helper(&(number % denom.value))?),
            };

            return Ok(s
                .replace("satu ratus", "seratus")
                .replace("satu ribu", "seribu"));
        }
    }
    Err("number out of range".to_string())
}

fn terbilang_helper(number: &i64) -> Result<String, String> {
    match *number {
        0..=11 => satuan(&number),
        12..=19 => belasan(&number),
        _ => other(&number),
    }
}

pub fn terbilang(number: &i64) -> Result<String, String> {
    let s = terbilang_helper(&(number.abs()))?;
    let s = match *number < 0 {
        true => format!("negatif {}", s),
        false => s,
    };
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terbilang_it_works() {
        let tests = [
            (-1_123_456_789_123, "negatif satu triliun seratus dua puluh tiga milyar empat ratus lima puluh enam juta tujuh ratus delapan puluh sembilan ribu seratus dua puluh tiga"),
            (-1_000, "negatif seribu"),
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
            (110, "seratus sepuluh"),
            (111, "seratus sebelas"),
            (119, "seratus sembilan belas"),
            (220, "dua ratus dua puluh"),
            (1000, "seribu"),
            (1019, "seribu sembilan belas"),
            (1119, "seribu seratus sembilan belas"),
            (2210, "dua ribu dua ratus sepuluh"),
            (2220, "dua ribu dua ratus dua puluh"),
            (10000, "sepuluh ribu"),
            (12220, "dua belas ribu dua ratus dua puluh"),
            (22220, "dua puluh dua ribu dua ratus dua puluh"),
            (222220, "dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (2222220, "dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (22222220, "dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (222222220, "dua ratus dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (987654321, "sembilan ratus delapan puluh tujuh juta enam ratus lima puluh empat ribu tiga ratus dua puluh satu"),
            (1234567890, "satu milyar dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"),
            (2222222220, "dua milyar dua ratus dua puluh dua juta dua ratus dua puluh dua ribu dua ratus dua puluh"),
            (1231234567890, "satu triliun dua ratus tiga puluh satu milyar dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"),
            (451231234567890, "empat ratus lima puluh satu triliun dua ratus tiga puluh satu milyar dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"),
            (4561231234567890, "empat ribu lima ratus enam puluh satu triliun dua ratus tiga puluh satu milyar dua ratus tiga puluh empat juta lima ratus enam puluh tujuh ribu delapan ratus sembilan puluh"),
            ];

        for test in tests.iter() {
            assert_eq!(terbilang(&test.0), Ok(String::from(test.1)));
        }
    }
}
