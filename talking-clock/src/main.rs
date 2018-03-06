fn main() {
    alarm_clock("00:00");
}

fn alarm_clock(time: &str) -> String {
    let times: Vec<&str> = time.split(":").collect();
    let hr = times[0].parse().unwrap();
    let min = times[1];

    let am_pm = am_pm(hr);
    let hr_format = get_val(hr);
    let mut res = String::new();

    if min == "00" {
        res.push_str(format!("{} {}", hr_format, am_pm).as_str());
    } else {
        let min_format = get_min(min);
        res.push_str(format!("{} {} {}", hr_format, min_format, am_pm).as_str());
    }

    res
}

fn get_val<'a>(time: usize) -> &'a str {
    let mut adjusted_time = time;
    if time > 12 {
        adjusted_time = time - 12;
    }
    match adjusted_time {
        0 => "twelve",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        _ => "none",
    }
}

fn teens(min: &str) -> &str {
    match min {
        "0" => "ten",
        "1" => "eleven",
        "2" => "twelve",
        "3" => "thirteen",
        "4" => "fourteen",
        "5" => "fifteen",
        "6" => "sixteen",
        "7" => "seventeen",
        "8" => "eighteen",
        "9" => "nineteen",
        _ => "none"
    }
}

fn tens(min: &str) -> &str {
    match min {
        "2" => "twenty",
        "3" => "thirty",
        "4" => "forty",
        "5" => "fifty",
        _ => "none",
    }
}

fn get_min(min: &str) -> String {
    let first = min.get(0..1).unwrap();
    let last = min.get(1..).unwrap();

    match first {
        "0" => {
            if last == "0" {
                String::new()
            } else {
                format!("{}", get_val(last.parse().unwrap()))
            }
        },
        "1" => {
            format!("{}", teens(last))
        }
        _ => {
            let tens = tens(first);
            let ones = get_val(last.parse().unwrap());
            format!("{} {}", tens, ones)
        }
    }
}

fn am_pm<'a >(hr: usize) -> &'a str {
    if hr < 12 {
        "am"
    } else {
        "pm"
    }
}

#[test]
fn test_alarm_clock() {
   assert_eq!(alarm_clock("00:00"), "twelve am");
   assert_eq!(alarm_clock("01:00"), "one am");
   assert_eq!(alarm_clock("01:15"), "one fifteen am");
   assert_eq!(alarm_clock("13:25"), "one twenty five pm");
   assert_eq!(alarm_clock("18:00"), "six pm");
}
