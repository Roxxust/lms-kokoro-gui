pub fn process_vowel_a(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let next_next_next_char_lower = if i + 3 < len { chars[i + 3].to_lowercase().next().unwrap_or(chars[i + 3]) } else { '\0' };
    let mut ipa = String::new();
    let mut increment = 1; // Default increment
    if (next_char_lower == 'i' && (i + 2 >= len || !chars[i + 2].is_alphabetic())) ||
       (next_char_lower == 'y' && (i + 2 >= len || !chars[i + 2].is_alphabetic())) ||
       (next_char_lower == 'i' && next_next_char_lower == 'g' && next_next_next_char_lower == 'h' && (i + 4 >= len || !chars[i + 4].is_alphabetic())) {
        ipa.push_str("eɪ");
        increment = if next_char_lower == 'i' && next_next_char_lower == 'g' { 4 } else { 2 };
    }
    else if (next_char_lower == 'u' || next_char_lower == 'w') ||
            (next_char_lower == 'l' && i + 2 < len && ['f', 'm', 's', 't', 'b', 'd', 'g', 'k', 'p', 'v', 'z'].contains(&next_next_char_lower)) {
         ipa.push_str("ɔː");
         increment = 2;
    }
    else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
         ipa.push_str("ɛə");
         increment = 1; // Only move past 'a', not 'r' or next vowel here
    }
    else if next_char_lower == 'r' && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("ɛə");
         increment = 3;
    }
    else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("eɪ");
         increment = 3;
    }
    else if next_char_lower == 'l' && i + 2 < len && ['m', 'n', 'k'].contains(&next_next_char_lower) {
         ipa.push_str("ɔː");
         increment = 1; // Only move past 'a', not 'l' or next char here
    }
    else {
        ipa.push('æ');
        // increment remains 1
    }
    (increment, ipa)
}

pub fn process_vowel_e(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
    prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let mut ipa = String::new();
    let mut increment = 1;
    if next_char_lower == 'e' || next_char_lower == 'a' || next_char_lower == 'i' || next_char_lower == 'y' {
        if i == 0 && len == 3 && next_char_lower == 'y' && next_next_char_lower == 'e' {
             ipa.push_str("aɪ");
             increment = 3;
        } else {
            ipa.push_str("iː");
            increment = 2;
        }
    }
    else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
         ipa.push_str("ɪə");
         increment = 1;
    }
    else if next_char_lower == 'w' {
         ipa.push_str("juː");
         increment = 2;
    }
    else if i == len - 1 && prev_char_lower != '\0' && is_consonant(prev_char_lower) {
         // Silent 'e' - do not add to IPA, just increment
         ipa.clear(); // Keep empty string
         increment = 1;
    }
    else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("iː");
         increment = 3;
    }
    else {
        ipa.push('ɛ');
        // increment remains 1
    }
    (increment, ipa)
}

pub fn process_vowel_i(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let mut ipa = String::new();
    let mut increment = 1;
    if next_char_lower == 'e' && (i + 2 >= len || !chars[i + 2].is_alphabetic()) {
        ipa.push_str("aɪ");
        increment = 2;
    }
    else if next_char_lower == 'g' && next_next_char_lower == 'h' {
         ipa.push_str("aɪ");
         increment = 3;
    }
    else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
         ipa.push_str("aɪə");
         increment = 1;
    }
    else if next_char_lower == 'r' && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("aɪə");
         increment = 3;
    }
    else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("aɪ");
         increment = 3;
    }
    else {
        ipa.push('ɪ');
        // increment remains 1
    }
    (increment, ipa)
}

pub fn process_vowel_o(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
    _is_liquid_or_nasal: &dyn Fn(char) -> bool, // Prefixed with underscore
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let mut ipa = String::new();
    let mut increment = 1;
    if next_char_lower == 'o' {
        ipa.push_str("uː");
        increment = 2;
    }
    else if (next_char_lower == 'a' || next_char_lower == 'e') ||
            (next_char_lower == 'w' && i + 2 < len) {
         ipa.push_str("əʊ");
         increment = 2;
    }
    else if next_char_lower == 'i' || next_char_lower == 'y' {
         ipa.push_str("ɔɪ");
         increment = 2;
    }
    else if next_char_lower == 'u' {
         ipa.push_str("aʊ");
         increment = 2;
    }
    else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("əʊ");
         increment = 3;
    }
    else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
         ipa.push_str("ɔː");
         increment = 1;
    }
    else if next_char_lower == 'r' && next_next_char_lower == 'e' {
         ipa.push_str("ɔː");
         increment = 3;
    }
    else if next_char_lower == 'o' && next_next_char_lower == 'r' {
         ipa.push_str("ɔː");
         increment = 3;
    }
    else if next_char_lower == 'u' && next_next_char_lower == 'r' {
         ipa.push_str("aʊə");
         increment = 3;
    }
    else if next_char_lower == '\'' {
         ipa.push_str("əʊ");
         increment = 1;
    }
    else if next_char_lower == 'w' && i + 2 == len {
         ipa.push_str("aʊ");
         increment = 2;
    }
    else if next_char_lower == 'n' && i + 2 < len && ['g', 'k'].contains(&next_next_char_lower) {
         ipa.push_str("ɔː");
         increment = 1;
    }
    else {
        ipa.push('ɒ');
        // increment remains 1
    }
    (increment, ipa)
}

pub fn process_vowel_u(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
    _is_liquid_or_nasal: &dyn Fn(char) -> bool, // Prefixed with underscore
    _prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let mut ipa = String::new();
    let mut increment = 1;
    if next_char_lower == 'e' && (i + 2 >= len || !chars[i + 2].is_alphabetic()) {
        ipa.push_str("juː");
        increment = 2;
    }
    else if next_char_lower == 'i' {
         ipa.push_str("juː");
         increment = 2;
    }
    else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
         ipa.push_str("ɜː");
         increment = 1;
    }
    else if next_char_lower == 'r' && next_next_char_lower == 'e' {
         ipa.push_str("jʊə");
         increment = 3;
    }
    else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
         ipa.push_str("juː");
         increment = 3;
    }
    else if next_char_lower == 'i' && next_next_char_lower == 'r' {
         ipa.push_str("aɪə");
         increment = 3;
    }
    else if next_char_lower == 'o' && i + 2 < len && ['l', 'r', 's'].contains(&next_next_char_lower) {
         ipa.push('ʊ');
         increment = 2;
    }
    else {
        ipa.push('ʌ');
        // increment remains 1
    }
    (increment, ipa)
}
