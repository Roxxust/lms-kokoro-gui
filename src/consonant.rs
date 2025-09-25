pub fn process_consonant_c(
    i: usize,
    len: usize,
    chars: &[char],
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let ipa = if ['e', 'i', 'y'].contains(&next_char_lower) {
        's'
    } else {
        'k'
    };
    (1, ipa.to_string()) // Always increment by 1 for 'c'
}

pub fn process_consonant_g(
    i: usize,
    len: usize,
    chars: &[char],
    prev_char_lower: char,
) -> (usize, String) {
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    // Handle 'gh' as a special case FIRST
    if next_char_lower == 'h' {
        // 'gh' at the end of a word is silent (knight, though)
        if i + 2 == len {
            return (2, String::new());
        }
        // 'gh' before 't', 's', 'u' is silent (knight, thought, through)
        else if ['t', 's', 'u'].contains(&next_next_char_lower) {
            return (2, String::new());
        }
        // 'gh' before vowels often makes /f/ sound (cough, laugh, enough)
        else if ['a', 'e', 'i', 'o', 'u'].contains(&next_next_char_lower) {
            return (2, "f".to_string());
        }
        // Default to silent for other 'gh' cases
        else {
            return (2, String::new());
        }
    }
    // Handle 'g' followed by 'e', 'i', or 'y' (soft g)
    else if ['e', 'i', 'y'].contains(&next_char_lower) && !(i > 0 && prev_char_lower == 'g') {
        return (1, "dʒ".to_string());
    }
    // Default to hard 'g'
    else {
        return (1, "ɡ".to_string());
    }
}

pub fn process_consonant_h(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
    prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let ipa = if i == 0 && is_vowel(next_char_lower) {
        'h'
    } else if i > 0 && is_vowel(prev_char_lower) && is_vowel(next_char_lower) {
        'h'
    } else if i > 0 && is_consonant(prev_char_lower) {
        // Silent 'h' after consonant
        '\0' // Use null character as a placeholder for silent
    } else {
        'h'
    };
    let result = if ipa == '\0' { String::new() } else { ipa.to_string() };
    (1, result) // Always increment by 1 for 'h'
}

pub fn process_consonant_t(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    is_consonant: &dyn Fn(char) -> bool,
    _is_liquid_or_nasal: &dyn Fn(char) -> bool, // Prefixed with underscore
    prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let next_next_next_char_lower = if i + 3 < len { chars[i + 3].to_lowercase().next().unwrap_or(chars[i + 3]) } else { '\0' };
    if next_char_lower == 'h' {
        let is_voiced = if i + 2 < len && next_next_char_lower != '\0' {
            is_vowel(next_next_char_lower) && (is_vowel(prev_char_lower) || _is_liquid_or_nasal(prev_char_lower)) // Use the prefixed parameter
        } else {
            false
        };
        return (2, if is_voiced { "ð".to_string() } else { "θ".to_string() }); // Increment by 2 for 'th'
    }
    else if next_char_lower == 'i' && next_next_char_lower == 'o' && i + 3 < len && next_next_next_char_lower == 'n' {
         let ipa = if prev_char_lower == 's' || (prev_char_lower == 'l' || prev_char_lower == 'n') {
             "ʒ"
         }
         else if prev_char_lower != '\0' && is_consonant(prev_char_lower) {
             "ʃ"
         } else {
             return (1, "t".to_string()); // If no special rule, just 't'
         };
         let mut result = ipa.to_string();
         result.push_str("ən");
         return (4, result); // Increment by 4 for 'tion'
    }
    else if next_char_lower == 'c' && next_next_char_lower == 'h' {
         return (3, "tʃ".to_string()); // Increment by 3 for 'tch'
    }
    else if next_char_lower == 't' && next_next_char_lower == 'l' && next_next_next_char_lower == 'e' && (i + 4 >= len || !chars[i + 4].is_alphabetic()) {
         return (4, "təl".to_string()); // Increment by 4 for 'ttle'
    }
    (1, "t".to_string()) // Default for 't'
}

pub fn process_consonant_s(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
    prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
    let next_next_next_char_lower = if i + 3 < len { chars[i + 3].to_lowercase().next().unwrap_or(chars[i + 3]) } else { '\0' };
    let is_z_sound = if i + 1 < len && next_char_lower != '\0' {
        is_vowel(prev_char_lower) && is_vowel(next_char_lower)
    } else {
         is_vowel(prev_char_lower)
    };
    if is_z_sound {
        (1, "z".to_string())
    }
    else if next_char_lower == 'h' {
         (2, "ʃ".to_string()) // Increment by 2 for 'sh'
    }
    else if next_char_lower == 'i' && next_next_char_lower == 'o' && i + 3 < len && next_next_next_char_lower == 'n' {
         let result = "ʒən".to_string(); // Removed 'mut' as it's not modified after creation
         (4, result) // Increment by 4 for 'sion' producing 'ʒən'
    }
    else if next_char_lower == 's' {
         (2, "s".to_string()) // Increment by 2 for 'ss'
    }
    else {
        (1, "s".to_string()) // Default for 's'
    }
}

pub fn process_consonant_or_vowel_y(
    i: usize,
    len: usize,
    chars: &[char],
    is_vowel: &dyn Fn(char) -> bool,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    let ipa_str = if i == 0 && next_char_lower != '\0' {
        "j" // Consonant 'y' at beginning - String literal
    }
    else if i == len - 1 {
        if is_vowel(chars.get(i.saturating_sub(1)).copied().unwrap_or('\0')) {
            "i" // Vowel 'y' at end after vowel - String literal
        } else {
            "aɪ" // Vowel 'y' at end after consonant - String literal
        }
    }
    else {
        "i" // Vowel 'y' in middle - String literal
    };
    let result = ipa_str.to_string(); // Convert string literal to String
    (1, result) // Always increment by 1 for 'y'
}

pub fn process_consonant_q(
    i: usize,
    len: usize,
    chars: &[char],
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    if next_char_lower == 'u' {
        (2, "kw".to_string()) // Increment by 2 for 'qu'
    } else {
        (1, "k".to_string()) // Default for 'q'
    }
}

pub fn process_consonant_w(
    i: usize,
    len: usize,
    chars: &[char],
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    if next_char_lower == 'h' {
        (2, "ʍ".to_string()) // Increment by 2 for 'wh'
    } else {
        (1, "w".to_string()) // Default for 'w'
    }
}

pub fn process_consonant_k(
    i: usize,
    len: usize,
    chars: &[char],
    prev_char_lower: char,
) -> (usize, String) {
    // Removed unused variable: let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
    let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
    if next_char_lower == 'n' && i == 0 {
        (1, String::new()) // Silent 'k' at beginning before 'n'
    }
    else if i > 0 && prev_char_lower == 'c' && next_char_lower == 'n' {
         (1, "k".to_string()) // 'k' in 'ckn' like 'knock'
    }
    else {
        (1, "k".to_string()) // Default for 'k'
    }
}
