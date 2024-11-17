///
pub fn parse_floats(input: &str) -> Option<f64> {
    let mut pre_point = String::new();
    let mut post_point = String::new();
    let mut point_found = false;
    for w in input.split_whitespace().into_iter() {
        if w == "point" {
            point_found = true;
        } else if point_found {
            post_point.push_str(w);
            post_point.push(' ');
        } else {
            pre_point.push_str(w);
            pre_point.push(' ');
        }
    }

    if !point_found {
        if let Some(ans) = parse_numeric_(input) {
            return Some(ans as f64);
        }
        return None;
    } else {
        let n1 = parse_numeric_(&pre_point);
        let n2 = parse_numeric_(&post_point);

        if n1 == None || n2 == None {
            return None;
        }

        let new_str = format!("{}.{}", n1.unwrap(), n2.unwrap());
        if let Ok(ans) = new_str.parse::<f64>() {
            return Some(ans);
        }
        return None;
    }
}

///
pub fn parse_numeric_(input: &str) -> Option<i64> {
    // Takes colloquially-worded string, e.g. "one thousand and sixty four", and trasnforms
    // it into its i32 equivalent (in this case, 1064).
    let words: Vec<&str> = input.split_whitespace().collect();

    let mut number = 0u64;
    let mut partial_sum = 0u64;
    let mut p: i32;
    let mut mult = 1i8;

    let mut pv: Vec<u8> = vec![];

    for (i, &word) in words.iter().enumerate() {
        if i == 0 && (word == "negative" || word == "minus") {
            mult = -1;
            continue;
        }

        match word {
            "and" => continue,
            "zero" => p = 0,
            "one" => p = 1,
            "won" => p = 1,
            "two" => p = 2,
            "to" => p = 2,
            "too" => p = 2,
            "three" => p = 3,
            "tree" => p = 3,
            "four" => p = 4,
            "for" => p = 4,
            "five" => p = 5,
            "six" => p = 6,
            "sex" => p = 6,
            "seven" => p = 7,
            "eight" => p = 8,
            "ate" => p = 8,
            "nine" => p = 9,
            "ten" => p = 10,
            "tin" => p = 10,
            "den" => p = 10,
            "eleven" => p = 11,
            "twelve" => p = 12,
            "thirteen" => p = 13,
            "fourteen" => p = 14,
            "fifteen" => p = 15,
            "sixteen" => p = 16,
            "seventeen" => p = 17,
            "eighteen" => p = 18,
            "nineteen" => p = 19,
            "twenty" => p = 20,
            "thirty" => p = 30,
            "forty" => p = 40,
            "fifty" => p = 50,
            "sixty" => p = 60,
            "seventy" => p = 70,
            "eighty" => p = 80,
            "ninety" => p = 90,
            "hundred" => {
                partial_sum *= 100;
                p = 0;
            }
            "thousand" => {
                number += partial_sum * 1000;
                partial_sum = 0;
                p = 0;
                pv.push(3); // log(1,000)
            }
            "million" => {
                number += partial_sum * 1000000;
                partial_sum = 0;
                p = 0;
                pv.push(6); // log(1,000,000)
            }
            "billion" => {
                number += partial_sum * 1000000000 as u64;
                partial_sum = 0;
                p = 0;
                pv.push(9); // log(1,000,000,000)
            }
            "trillion" => {
                number += partial_sum * 1000000000000 as u64;
                partial_sum = 0;
                p = 0;
                pv.push(12); // log(1,000,000,000,000)
            }
            _ => {
                println!("Invalid word: {}", word);
                return None;
            }
        }

        if p != 0 {
            pv.push(p.ilog10() as u8);
        }

        let n = pv.len();
        assert!(n <= 1 || pv[n - 1] < pv[n - 2] || pv[n - 2] <= 2);

        partial_sum += p as u64;
    }

    if let Some(m) = pv.iter().max() {
        if pv.iter().filter(|&x| *x == *m).count() > 1 {
            return parse_digit_seq_(input);
        }
    }

    Some(mult as i64 * (number + partial_sum) as i64)
}

fn parse_digit_seq_(input: &str) -> Option<i64> {
    let mut ans = String::new();
    for w in input.split_whitespace() {
        let num_word = match w {
            "zero" => "0",
            "one" => "1",
            "won" => "1",
            "two" => "2",
            "to" => "2",
            "too" => "2",
            "three" => "3",
            "tree" => "3",
            "four" => "4",
            "for" => "4",
            "five" => "5",
            "six" => "6",
            "sex" => "6",
            "seven" => "7",
            "eight" => "8",
            "ate" => "8",
            "nine" => "9",
            "ten" => "10",
            "tin" => "10",
            "den" => "10",
            "eleven" => "11",
            "twelve" => "12",
            "thirteen" => "13",
            "fourteen" => "14",
            "fifteen" => "15",
            "sixteen" => "16",
            "seventeen" => "17",
            "eighteen" => "18",
            "nineteen" => "19",
            "twenty" => "20",
            "thirty" => "30",
            "forty" => "40",
            "fifty" => "50",
            "sixty" => "60",
            "seventy" => "70",
            "eighty" => "80",
            "ninety" => "90",
            _ => {
                println!("Invalid word: {}", w);
                return None;
            }
        };

        ans.push_str(num_word);
    }

    if let Ok(p) = ans.parse::<i64>() {
        Some(p)
    } else {
        None
    }
}

/// While this function is not meant to be used publicly, it serves
/// to explain how Rattle boolean values parse (which is fairly
/// self-explanatory).
pub fn _parse_booleans(input: &str) -> Option<bool> {
    match &input.to_lowercase()[..] {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
