pub fn convert_under_thousand(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
                "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    if n == 0 {
        return String::new();
    }
    if n < 20 {
        return ones[n as usize].to_string();
    }
    if n < 100 {
        let ten_part = tens[(n / 10) as usize];
        let one_part = if n % 10 != 0 { &ones[(n % 10) as usize] } else { "" };
        if !one_part.is_empty() {
            return format!("{} {}", ten_part, one_part);
        } else {
            return ten_part.to_string();
        }
    }
    let hundreds_digit = n / 100;
    let remainder = n % 100;
    let hundred_part = format!("{} hundred", ones[hundreds_digit as usize]);
    if remainder == 0 {
        return hundred_part;
    } else {
        let under_hundred = convert_under_thousand(remainder);
        return format!("{} and {}", hundred_part, under_hundred);
    }
}

pub fn number_to_words(mut num: u64) -> String {
    if num == 0 {
        return "zero".to_string();
    }
    let scales = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
    let mut parts = Vec::new();
    let mut scale_index = 0;
    while num > 0 {
        let chunk = num % 1000;
        if chunk != 0 {
            let chunk_words = convert_under_thousand(chunk);
            if !scales[scale_index].is_empty() {
                parts.push(format!("{} {}", chunk_words, scales[scale_index]));
            } else {
                parts.push(chunk_words);
            }
        }
        num /= 1000;
        scale_index += 1;
    }
    parts.reverse();
    parts.join(" ")
}

pub fn string_number_to_words(number_str: &str) -> String {
    if let Some(dot_pos) = number_str.find('.') {
        let integer_part = &number_str[..dot_pos];
        let fractional_part = &number_str[dot_pos + 1..];
        let mut result = String::new();
        let cleaned_integer: String = integer_part.chars().filter(|c| c.is_ascii_digit()).collect();
        if !cleaned_integer.is_empty() && cleaned_integer != "0" {
             if let Ok(int_num) = cleaned_integer.parse::<u64>() {
                result.push_str(&number_to_words(int_num));
             } else {
                result.push_str(integer_part);
             }
             result.push_str(" point ");
        } else if cleaned_integer == "0" {
            result.push_str("zero point ");
        } else {
            result.push_str("point ");
        }
        for digit_char in fractional_part.chars() {
            if digit_char.is_ascii_digit() {
                let digit_word = match digit_char {
                    '0' => "zero", '1' => "one", '2' => "two", '3' => "three", '4' => "four",
                    '5' => "five", '6' => "six", '7' => "seven", '8' => "eight", '9' => "nine",
                    _ => "",
                };
                result.push_str(digit_word);
                result.push(' ');
            }
        }
        result.trim_end().to_string()
    } else {
        let cleaned_str: String = number_str.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Ok(num) = cleaned_str.parse::<u64>() {
            number_to_words(num)
        } else {
            eprintln!("Warning: Failed to parse large integer string '{}'", number_str);
            number_str.to_string()
        }
    }
}
