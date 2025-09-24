use super::*;
use crate::tts::word2ipa_with_context;
use crate::tts::g2p;

pub fn word2ipa(word: &str) -> String {
    pub static CONTRACTIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        use std::collections::HashMap;
        let entries = [
    // ─── Contractions ───
    //ai converted.
    ("they're", "/ðeɪr/"), ("who're", "/hʊr/"), ("what're", "/wʌtr/"),
    ("where're", "/wɛr/"), ("when're", "/wɛnr/"), ("why're", "/waɪr/"),
    ("i've", "/aɪv/"), ("we've", "/wiv/"), ("they've", "/ðev/"),
    ("who've", "/hʊv/"), ("what've", "/wʌtv/"), ("where've", "/wɛrv/"),
    ("when've", "/wɛnv/"), ("why've", "/waɪv/"), ("how've", "/haʊv/"),
    ("i'll", "/aɪl/"), ("we'll", "/wil/"), ("they'll", "/ðeɪl/"),
    ("he'll", "/hel/"), ("she'll", "/ʃel/"), ("it'll", "/ɪtl/"), ("who'll", "/hul/"),
    ("what'll", "/wʌtl/"), ("where'll", "/wɛrl/"), ("when'll", "/wɛnl/"),
    ("why'll", "/waɪl/"), ("how'll", "/haʊl/"), ("i'ma", "/ˈaɪəmə/"),
    ("i'd", "/aɪd/"), ("you'd", "/juːd/"), ("we'd", "/wɪd/"), ("they'd", "/ðed/"),
    ("he'd", "/hed/"), ("she'd", "/ʃed/"), ("it'd", "/ɪtd/"),
    ("who'd", "/hud/"), ("what'd", "/wʌtd/"), ("where'd", "/wɛrd/"),
    ("when'd", "/wɛnd/"), ("why'd", "/waɪd/"), ("how'd", "/haʊd/"),
    ("i'd've", "/ˈaɪdəv/"), ("you'd've", "/ˈjudəv/"),
    ("we'd've", "/ˈwɪdəv/"), ("they'd've", "/ˈðedəv/"),
    ("he'd've", "/ˈhedəv/"), ("she'd've", "/ˈʃedəv/"),
    ("it'd've", "/ˈɪtdəv/"), ("how're", "/haʊr/"),
    ("he's", "/hiːz/"), ("she's", "/ʃiz/"), ("it's", "/ɪts/"), ("here's", "/hɪrz/"),
    ("there's", "/ðɛrz/"), ("that's", "/ðæts/"), ("this's", "/ðɪs ɪz/"),
    ("when's", "/wɛnz/"), ("why's", "/waɪz/"), ("how's", "/haʊz/"), ("where's", "/wɛrz/"),
    ("you're", "yo.ʊr"),
    //you're   you've   you'll
    //proper
    ("i'm", "/aɪm/"), ("we're", "/wɚ/"), 
    ("what's","/wɑːts/"), ("who's","/huːz/"),
    ("isn't","/ɪsnət/"), ("aren't","/ɑːrnt/"), ("wasn't","/ˈwɑː.zənt/"), ("weren't","/wɝːnt/"),
    ("haven't","/havənt/"), ("hasn't","/ˈhæz.ənt/"), ("hadn't","hádənt"),
    ("won't","/woʊnt/"), ("couldn't","/ˈkʊd.ənt/"),
    ("mightn't","/ˈmaɪ.tənt/"), ("mustn't","/ˈmʌs.ənt/"),
    ("shan't","shant"), ("needn't","/ˈniː.dənt/"), ("daren't","darent"),
    ("let's","/lets/"), ("o'clock","/əˈklɒk/"), ("'em","em"),
    ("don't","/doʊnt/"), ("won't've", "/wəʊnt əv/"),
    ("woudn't","/wʊldn̩t/"), ("shouldn't","ʃʊdn̩t/"),
    ("you've", "yewːve/"),  ("you'll", "/yewːll/"),
    // ─── Pronunciation Helpers ───
    ("read","riːd"), ("tear","tɪə"), ("AI","A.I"),
    ("lives","/laɪvz/"), ("every","/ˈevri"), ("liberating","/ˈlɪb.ə.reɪ.t̬ɪŋ/"),
    ("for","/fɔːr/"), ("further","/ˈfɝː.ðɚ/"), ("forever","/fɔːˈrev.ɚ/"),
    ("was","/wəz/"), ("been","/bɪn/"), ("clothes","/kloʊðz/"),
    ("often","/ˈɔf(ə)n/"), ("either","/ˈiːðər/"), ("neither","/ˈniːðər/"),
    ("route","/ruːt/"), ("suite","swiːt"), ("aisle","aɪl"),
    ("bass","/beɪs/"), ("bow","/baʊ/"), ("buff","/bʌf/"),
    ("cache","/kæʃ/"), ("caught","/kɔːt/"), ("chaos","/ˈkeɪ.ɑs/"),
    ("chaise","ʃeɪz"), ("colonel","/ˈkɜrnəl/"), ("comfortable","/ˈkʌmftəbəl/"),
    ("cupboard","/ˈkʌbərd/"), ("debt","/det/"), ("design","dɪˈzaɪn"),
    ("diabetes","/ˌdaɪəˈbiːtiːz/"), ("doubt","/daʊt/"), ("ensemble","ɑnˈsɑmbəl"),
    ("eye","aɪ"), ("fuchsia","/ˈfjuːʃə/"), ("genre","ʒɑnrə"),
    ("gourmet","ɡʊrˈmeɪ"), ("hierarchy","/ˈhaɪərɑrki/"), ("hour","aʊər"),
    ("hyperbole","haɪˈpɜrbəli"), ("indict","ɪnˈdaɪt"), ("iron","aɪərn"),
    ("jaguar","/ˈdʒæɡjuːɑr/"), ("knight","naɪt"), ("know","noʊ"),
    ("leisure","/ˈliːʒər/"), ("liaison","/ˈliːəzɑn/"), ("light","laɪt"),
    ("lose","luːz"), ("margarine","/ˌmɑrdʒəˈriːn/"), ("mischievous","/ˈmɪtʃəvəs/"),
    ("naive","naɪˈiːv"), ("naked","/ˈneɪkɪd/"), ("niche","niːʃ"),
    ("obese","oʊˈbiːs"), ("once","wʌns"), ("one","wʌn"),
    ("phlegm","/flem/"), ("pint","paɪnt"), ("queue","kjuː"),
    ("raspberry","/ˈræzˌbɛri/"), ("rarely","/ˈrɛrli/"), ("receipt","rɪˈsiːt"),
    ("schedule","/ˈskɛdʒuːl/"), ("scissors","/ˈsɪzərz/"), ("segue","/ˈseɡweɪ/"),
    ("sergeant","/ˈsɑrdʒənt/"), ("sherbet","/ˈʃɜrbət/"), ("sign","saɪn"),
    ("subtle","/ˈsʌtəl/"), ("suite","swiːt"), ("sword","sɔrd"),
    ("thyme","taɪm"), ("tongue","tʌŋ"), ("toward","tɔrd"),
    ("trouser","/ˈtraʊzər/"), ("vehicle","/ˈviːɪkəl/"), ("whole","/hoʊl/"),
    ("world","/wɝːld/"), ("worse","wɜrs"), ("yolk","joʊk"),
    ("elongating","/ɪˈlɑːŋ.ɡeɪt/"), ("bloodlust","/ˈblʌd ˌlʌst/"),
    ("teenager","/ˈtiːnˌeɪ.dʒɚ/"), ("towering","/ˈtaʊ.ɚ.ɪŋ/"),
    ("labyrinth","/ˈlæb.ə.rɪnθ/"), ("labyrinthine","/ˌlæb.əˈrɪnˌθaɪn/."),
    ("history","/ˈhɪs.t̬ɚ.i/"), ("lover","/ˈlʌv.ɚ/"),
    ("discoveries","/dɪˈskʌv.ə.riz/"), ("read","/riːd/"),
    ("concert","/ˈkɑːn.sɚt/"), ("rephrasing","/riˈfreɪzɪŋ/"),
        ];
        HashMap::from(entries)
    });
    let lower_word = word.to_lowercase();
    if let Some(&expanded_phrase) = CONTRACTIONS.get(lower_word.as_str()) {
        // Optimized IPA character sets - only includes characters relevant for English phonetics
        let ipa_like_chars = vec![
            // Basic vowels
            'a', 'e', 'i', 'o', 'u', 'æ', 'ɑ', 'ɔ', 'ə', 'ɛ', 'ɪ', 'ʊ', 'ʌ', 'ɜ', 'ɒ',
            // Length marker
            'ː',
            // Consonants
            'b', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'z',
            // Special consonants
            'ʃ', 'ʒ', 'θ', 'ð', 'ŋ', 'ɹ', 'ɦ',
            // R-colored vowels
            'ɚ', 'ɝ',
            // Suprasegmentals
            'ˈ', 'ˌ',
            // Space and syllable break
            ' ', '.',
            // Common diacritics
            '̩', '̃', '̪', 'ʰ', '̯'
        ];
        
        let ipa_like_strings = vec![
            // Diphthongs
            "aɪ", "aʊ", "eɪ", "oʊ", "ɔɪ", "eə", "ɪə", "ʊə",
            // Triphthongs
            "aʊə", "aɪə", "ɔɪə",
            // Affricates
            "tʃ", "dʒ",
            // Syllabic consonants
            "n̩", "l̩", "r̩", "m̩",
            // Dental consonants
            "t̪", "d̪",
            // Aspirated consonants
            "pʰ", "tʰ", "kʰ",
            // Lengthened vowels
            "iː", "uː", "ɜː", "ɑː", "ɔː", "eː", "ʌː",
            // Common combinations
            "tʃʰ", "dʒʰ", "θ̞", "ð̞"
        ];

        let is_phoneme_string = expanded_phrase.chars().all(|c| {
            c.is_ascii_lowercase() ||
            ipa_like_chars.contains(&c) ||
            ipa_like_strings.iter().any(|&s| expanded_phrase.contains(s))
        });
        
        if is_phoneme_string {
            // Clean up the IPA string - remove slashes and dots
            return expanded_phrase.to_string()
                .replace("/", "")
                .replace(".", "");
        } else {
            // If it's not a phoneme string, process it through G2P
            return g2p(expanded_phrase);
        }
    }
    
    // If not a contraction, use context-aware IPA conversion
    word2ipa_with_context(word, "", "")
}
