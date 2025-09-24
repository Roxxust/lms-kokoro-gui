use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static HETERONYMS: Lazy<HashMap<&'static str, Vec<(&'static str, &'static str)>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // Wind (noun: air movement) vs. Wind (verb: to twist/turn)
    // Pattern order: specific context for "wind" as noun first, then as verb, then default to noun
    m.insert("wind", vec![
        // Wind as noun (air movement)
        (r"(?i)\b(?:blow|breeze|gust|storm|hurricane|air|weather|north|south|east|west|strong|gentle|cold|hot|fierce|light|fresh|prevailing)\b.*?\bwind\b", "wɪnd"),
        // Wind as verb (to twist/turn)
        (r"(?i)\bwind\b.*?\b(?:clock|watch|bobbin|spool|tape|up|down|tight|loose|around|the|it|him|her|them)\b", "waɪnd"),
        // Special case: "wind up" (to conclude or to tighten)
        (r"(?i)\bwind-?up\b", "ˈwaɪnd ʌp"),
        // Default to noun pronunciation
        (r"(?i)\bwind\b", "wɪnd"),
    ]);

    // Winds (plural of wind)
    m.insert("winds", vec![
        // Winds as air movement
        (r"(?i)\b(?:windy|blowing|blustery|howling|whistling|prevailing|trade|solar|westerly|easterly)\b.*?\bwinds\b", "wɪndz"),
        // Winds as verb (less common plural)
        (r"(?i)\bwinds\b.*?\b(?:clock|watch|bobbin|spool|tape)\b", "waɪndz"),
        // Default to air movement
        (r"(?i)\bwinds\b", "wɪndz"),
    ]);

    // Bass (fish/instrument) vs. Bass (low frequency sound)
    m.insert("bass", vec![
        // Bass as low frequency sound
        (r"(?i)\b(?:singer|voice|soprano|alto|tenor|choir|sang|sings|frequency|sound|audio|amplifier|guitar|amplification)\b.*?\bbass\b", "beɪs"),
        // Bass as fish or instrument
        (r"(?i)\b(?:fish|fishing|lake|river|lure|bass|guitar|instrument|bassoon|bassoonist)\b", "bæs"),
        // Default to fish/instrument
        (r"(?i)\bbass\b", "bæs"),
    ]);

    // Live (reside) vs. Live (broadcast)
    m.insert("live", vec![
        // Live as verb (to reside)
        (r"(?i)\b(?:reside|dwelling|home|inhabiting|living|where|currently|living|dwells|dwelling|dwelt)\b.*?\blive\b", "lɪv"),
        // Live as adjective (broadcast)
        (r"(?i)\blive\b.*?\b(?:broadcast|performance|show|concert|streaming|event|tv|television|video|feed)\b", "laɪv"),
        // Default to verb (reside)
        (r"(?i)\blive\b", "lɪv"),
    ]);

    // Lives (plural of live)
    m.insert("lives", vec![
        // Lives as verb (resides)
        (r"(?i)\b(?:reside|dwelling|home|inhabiting|living|where|currently|living|dwells|dwelling|dwelt)\b.*?\blives\b", "lɪvz"),
        // Lives as plural noun (broadcasts)
        (r"(?i)\blives\b.*?\b(?:broadcast|performance|show|concert|streaming|event|tv|television)\b", "laɪvz"),
        // Default to verb
        (r"(?i)\blives\b", "lɪvz"),
    ]);

    // Read (past tense) vs. Read (present tense)
    m.insert("read", vec![
        // Read as past tense
        (r"(?i)\b(?:past|read|book|novel|story|yesterday|finished|already|just|had|has|have)\b.*?\bread\b", "red"),
        // Read as present tense
        (r"(?i)\b(?:looking|studying|examining|now|currently|reading|can|please|let|me)\b.*?\bread\b", "riːd"),
        // Default to present tense
        (r"(?i)\bread\b", "riːd"),
    ]);

    // Tear (rip) vs. Tear (cry)
    m.insert("tear", vec![
        // Tear as verb (to rip)
        (r"(?i)\b(?:rip|shred|tore|torn|scratch|paper|fabric|cloth|cloth|fabric|material)\b.*?\btear\b", "tɛr"),
        // Tear as noun (from eye)
        (r"(?i)\b(?:cry|weep|sad|eye|drop|shed|shedding|emotional|tearful|water)\b.*?\btear\b", "tɪr"),
        // Default to tear as noun
        (r"(?i)\btear\b", "tɪr"),
    ]);

    // Tears (plural)
    m.insert("tears", vec![
        // Tears as verb (ripping)
        (r"(?i)\b(?:rip|shred|tore|torn|scratch)\b.*?\btears\b", "tɛrz"),
        // Tears as noun (from eyes)
        (r"(?i)\b(?:cry|weep|sad|eye|drop|shed|shedding)\b.*?\btears\b", "tɪrz"),
        // Default to tears as noun
        (r"(?i)\btears\b", "tɪrz"),
    ]);

    // Bow (front of ship) vs. Bow (weapon)
    m.insert("bow", vec![
        // Bow as front of ship
        (r"(?i)\b(?:front|down|curtsy|respect|ship|vessel|boat|nautical|sail|sailing)\b.*?\bbow\b", "baʊ"),
        // Bow as weapon or violin
        (r"(?i)\b(?:arrow|violin|string|ship|stern|weapon|archery|archer|shoot)\b.*?\bbow\b", "boʊ"),
        // Default to bow as weapon
        (r"(?i)\bbow\b", "boʊ"),
    ]);

    // Row (argument) vs. Row (boat)
    m.insert("row", vec![
        // Row as argument
        (r"(?i)\b(?:argument|fight|quarrel|dispute|verbal|angry|heated|shouting)\b.*?\brow\b", "raʊ"),
        // Row as boat movement
        (r"(?i)\b(?:boat|paddle|oar|river|rowing|crew|regatta|water)\b.*?\brow\b", "roʊ"),
        // Default to boat movement
        (r"(?i)\brow\b", "roʊ"),
    ]);

    // Lead (metal) vs. Lead (to guide)
    m.insert("lead", vec![
        // Lead as verb (to guide)
        (r"(?i)\b(?:guided|followed|directed|managed|guide|guiding|direction|path|example)\b.*?\blead\b", "led"),
        // Lead as metal or pencil
        (r"(?i)\b(?:metal|pencil|weight|first|primary|main|chief|lead|leaded|pipe)\b.*?\blead\b", "liːd"),
        // Default to verb (to guide)
        (r"(?i)\blead\b", "led"),
    ]);

    // Leads (plural)
    m.insert("leads", vec![
        // Leads as verb (guides)
        (r"(?i)\b(?:guided|followed|directed|managed|guide|guiding|direction|path)\b.*?\bleads\b", "ledz"),
        // Leads as plural noun (metal)
        (r"(?i)\b(?:metal|pencil|weight|first|primary|main|chief)\b.*?\bleads\b", "liːdz"),
        // Default to verb
        (r"(?i)\bleads\b", "ledz"),
    ]);

    // Close (near) vs. Close (shut)
    m.insert("close", vec![
        // Close as adjective (near)
        (r"(?i)\b(?:near|proximity|adjacent|very|quite|relatively|approximately|distance)\b.*?\bclose\b", "kloʊs"),
        // Close as verb (shut)
        (r"(?i)\b(?:shut|door|window|lid|end|business|deal|transaction|company)\b.*?\bclose\b", "kloʊz"),
        // Default to verb (shut)
        (r"(?i)\bclose\b", "kloʊz"),
    ]);

    // Closed (past tense)
    m.insert("closed", vec![
        // Closed as adjective (near)
        (r"(?i)\b(?:near|proximity|adjacent|very|quite)\b.*?\bclosed\b", "kloʊst"),
        // Closed as verb (shut)
        (r"(?i)\b(?:shut|door|window|lid|end|business|deal)\b.*?\bclosed\b", "kloʊzd"),
        // Default to verb
        (r"(?i)\bclosed\b", "kloʊzd"),
    ]);

    // Content (satisfied) vs. Content (material)
    m.insert("content", vec![
        // Content as adjective (satisfied)
        (r"(?i)\b(?:satisfied|pleased|happy|glad|contented|joyful|delighted)\b.*?\bcontent\b", "kənˈtent"),
        // Content as noun (material)
        (r"(?i)\b(?:material|substance|inside|table|of|contents|web|digital|written)\b.*?\bcontent\b", "ˈkɑːntent"),
        // Default to material
        (r"(?i)\bcontent\b", "ˈkɑːntent"),
    ]);

    // Record (document) vs. Record (to create)
    m.insert("record", vec![
        // Record as noun (document)
        (r"(?i)\b(?:document|tape|disc|album|music|vinyl|CD|audio|video|play|collection)\b.*?\brecord\b", "ˈrekərd"),
        // Record as verb (to create)
        (r"(?i)\b(?:break|achieve|set|history|create|make|generate|document|capture)\b.*?\brecord\b", "rɪˈkɔːrd"),
        // Default to noun
        (r"(?i)\brecord\b", "ˈrekərd"),
    ]);

    // Records (plural)
    m.insert("records", vec![
        // Records as noun (documents)
        (r"(?i)\b(?:document|tape|disc|album|music|vinyl|CD|audio|video)\b.*?\brecords\b", "ˈrekərdz"),
        // Records as verb (to create)
        (r"(?i)\b(?:break|achieve|set|history|create|make|generate)\b.*?\brecords\b", "rɪˈkɔːrdz"),
        // Default to noun
        (r"(?i)\brecords\b", "ˈrekərdz"),
    ]);

    // Produce (agricultural) vs. Produce (to create)
    m.insert("produce", vec![
        // Produce as noun (agricultural)
        (r"(?i)\b(?:fruit|vegetable|farm|agriculture|fresh|organic|market|local|harvest)\b.*?\bproduce\b", "ˈprɑːdus"),
        // Produce as verb (to create)
        (r"(?i)\b(?:create|make|generate|result|develop|build|construct|produce|production)\b.*?\bproduce\b", "prəˈdus"),
        // Default to verb
        (r"(?i)\bproduce\b", "prəˈdus"),
    ]);

    // Produces (plural)
    m.insert("produces", vec![
        // Produces as noun (agricultural)
        (r"(?i)\b(?:fruit|vegetable|farm|agriculture|fresh|organic)\b.*?\bproduces\b", "ˈprɑːdusɪz"),
        // Produces as verb (to create)
        (r"(?i)\b(?:create|make|generate|result|develop)\b.*?\bproduces\b", "prəˈdusɪz"),
        // Default to verb
        (r"(?i)\bproduces\b", "prəˈdusɪz"),
    ]);

    // Present (gift) vs. Present (current)
    m.insert("present", vec![
        // Present as noun (gift)
        (r"(?i)\b(?:gift|present|package|wrapped|birthday|christmas|holiday|surprise|opening)\b.*?\bpresent\b", "ˈprezənt"),
        // Present as adjective (current)
        (r"(?i)\b(?:now|current|time|here|present|moment|day|age|situation|tense)\b.*?\bpresent\b", "prɪˈzent"),
        // Default to adjective
        (r"(?i)\bpresent\b", "prɪˈzent"),
    ]);

    // Presented (past tense)
    m.insert("presented", vec![
        // Presented as noun (gift)
        (r"(?i)\b(?:gift|present|package|wrapped|birthday|christmas)\b.*?\bpresented\b", "ˈprezəntɪd"),
        // Presented as verb (current)
        (r"(?i)\b(?:now|current|time|here|present|moment|day)\b.*?\bpresented\b", "prɪˈzentɪd"),
        // Default to verb
        (r"(?i)\bpresented\b", "prɪˈzentɪd"),
    ]);

    // Object (thing) vs. Object (to oppose)
    m.insert("object", vec![
        // Object as noun (thing)
        (r"(?i)\b(?:thing|item|physical|tangible|material|concrete|visible|object|material|solid)\b.*?\bobject\b", "ˈɑːbdʒekt"),
        // Object as verb (to oppose)
        (r"(?i)\b(?:oppose|disagree|argue|protest|complain|object|objection|raise)\b.*?\bobject\b", "əbˈdʒekt"),
        // Default to noun
        (r"(?i)\bobject\b", "ˈɑːbdʒekt"),
    ]);

    // Objected (past tense)
    m.insert("objected", vec![
        // Objected as noun (thing)
        (r"(?i)\b(?:thing|item|physical|tangible)\b.*?\bobjected\b", "ˈɑːbdʒektɪd"),
        // Objected as verb (to oppose)
        (r"(?i)\b(?:oppose|disagree|argue|protest)\b.*?\bobjected\b", "əbˈdʒektɪd"),
        // Default to verb
        (r"(?i)\bobjected\b", "əbˈdʒektɪd"),
    ]);

    // Contract (agreement) vs. Contract (shrink)
    m.insert("contract", vec![
        // Contract as noun (agreement)
        (r"(?i)\b(?:agreement|legal|sign|deal|business|written|verbal|contract|agreement|signed)\b.*?\bcontract\b", "ˈkɑːntrækt"),
        // Contract as verb (shrink)
        (r"(?i)\b(?:shrink|tighten|muscle|wrinkle|reduce|contract|contraction|physiology)\b.*?\bcontract\b", "kənˈtrækt"),
        // Default to noun
        (r"(?i)\bcontract\b", "ˈkɑːntrækt"),
    ]);

    // Contracts (plural)
    m.insert("contracts", vec![
        // Contracts as noun (agreements)
        (r"(?i)\b(?:agreement|legal|sign|deal|business|written)\b.*?\bcontracts\b", "ˈkɑːntrækts"),
        // Contracts as verb (shrink)
        (r"(?i)\b(?:shrink|tighten|muscle|wrinkle)\b.*?\bcontracts\b", "kənˈtrækts"),
        // Default to noun
        (r"(?i)\bcontracts\b", "ˈkɑːntrækts"),
    ]);

    // Desert (arid area) vs. Desert (to abandon)
    m.insert("desert", vec![
        // Desert as noun (arid area)
        (r"(?i)\b(?:arid|sand|dune|Sahara|Gobi|Kalahari|desert|dry|hot|cactus)\b.*?\bdesert\b", "ˈdezərt"),
        // Desert as verb (to abandon)
        (r"(?i)\b(?:abandon|leave|forsake|defect|desert|deserter|military|duty|responsibility)\b.*?\bdesert\b", "dɪˈzɜːrt"),
        // Default to noun
        (r"(?i)\bdesert\b", "ˈdezərt"),
    ]);

    // Deserted (past tense)
    m.insert("deserted", vec![
        // Deserted as noun (arid area)
        (r"(?i)\b(?:arid|sand|dune|Sahara)\b.*?\bdeserted\b", "ˈdezərtɪd"),
        // Deserted as verb (abandoned)
        (r"(?i)\b(?:abandon|leave|forsake|defect)\b.*?\bdeserted\b", "dɪˈzɜːrtɪd"),
        // Default to verb
        (r"(?i)\bdeserted\b", "dɪˈzɜːrtɪd"),
    ]);

    // Minute (tiny) vs. Minute (time unit)
    m.insert("minute", vec![
        // Minute as adjective (tiny)
        (r"(?i)\b(?:tiny|small|infinitesimal|wee|minute|microscopic|imperceptible)\b.*?\bminute\b", "maɪˈnut"),
        // Minute as noun (time unit)
        (r"(?i)\b(?:60|second|time|hour|minute|clock|watch|timer|duration)\b.*?\bminute\b", "ˈmɪnɪt"),
        // Default to time unit
        (r"(?i)\bminute\b", "ˈmɪnɪt"),
    ]);

    // Minutely (adverb)
    m.insert("minutely", vec![
        // Minutely as adverb (in tiny detail)
        (r"(?i)\b(?:tiny|small|infinitesimal|wee|minute|microscopic)\b.*?\bminutely\b", "maɪˈnutli"),
        // Minutely as adverb (by the minute)
        (r"(?i)\b(?:60|second|time|hour|minute|clock|watch)\b.*?\bminutely\b", "ˈmɪnɪtli"),
        // Default to time-related
        (r"(?i)\bminutely\b", "ˈmɪnɪtli"),
    ]);

    // Invalid (not valid) vs. Invalid (sick person)
    m.insert("invalid", vec![
        // Invalid as adjective (not valid)
        (r"(?i)\b(?:not valid|wrong|incorrect|void|invalid|illegal|unacceptable)\b.*?\binvalid\b", "ɪnˈvælɪd"),
        // Invalid as noun (sick person)
        (r"(?i)\b(?:patient|hospital|sick|wheelchair|disabled|infirm|invalid|care)\b.*?\binvalid\b", "ˈɪnvəlɪd"),
        // Default to not valid
        (r"(?i)\binvalid\b", "ɪnˈvælɪd"),
    ]);

    // Use (utilize) vs. Use (purpose)
    m.insert("use", vec![
        // Use as verb (utilize)
        (r"(?i)\b(?:utilize|employ|apply|function|use|used|using|user)\b.*?\buse\b", "juːz"),
        // Use as noun (purpose)
        (r"(?i)\b(?:purpose|reason|benefit|utility|use|uses|usage)\b.*?\buse\b", "juːs"),
        // Default to verb
        (r"(?i)\buse\b", "juːz"),
    ]);

    // Used (past tense)
    m.insert("used", vec![
        // Used as verb (utilized)
        (r"(?i)\b(?:utilize|employ|apply|function|use|used|using)\b.*?\bused\b", "juːzd"),
        // Used as adjective (purpose/benefit)
        (r"(?i)\b(?:purpose|reason|benefit|utility|use|uses)\b.*?\bused\b", "juːst"),
        // Default to verb
        (r"(?i)\bused\b", "juːzd"),
    ]);

    // Abuse (misuse) vs. Abuse (verb)
    m.insert("abuse", vec![
        // Abuse as noun (misuse)
        (r"(?i)\b(?:misuse|mistreat|exploit|wrong|abuse|verbal|physical|drug|substance)\b.*?\babuse\b", "əˈbjuːz"),
        // Abuse as verb (to mistreat)
        (r"(?i)\b(?:misuse|mistreat|exploit|wrong|abuse|verb|verbally|physically)\b.*?\babuse\b", "əˈbjuːs"),
        // Default to noun
        (r"(?i)\babuse\b", "əˈbjuːz"),
    ]);

    // Abused (past tense)
    m.insert("abused", vec![
        // Abused as noun (misuse)
        (r"(?i)\b(?:misuse|mistreat|exploit|wrong)\b.*?\babused\b", "əˈbjuːzd"),
        // Abused as verb (mistreated)
        (r"(?i)\b(?:misuse|mistreat|exploit|wrong)\b.*?\babused\b", "əˈbjuːst"),
        // Default to verb
        (r"(?i)\babused\b", "əˈbjuːzd"),
    ]);

    // Conduct (behavior) vs. Conduct (lead)
    m.insert("conduct", vec![
        // Conduct as noun (behavior)
        (r"(?i)\b(?:behavior|manner|way|ethics|professional|personal|code|conduct)\b.*?\bconduct\b", "ˈkɑːndʌkt"),
        // Conduct as verb (lead/direct)
        (r"(?i)\b(?:lead|direct|manage|orchestra|conduct|conductor|research|experiment)\b.*?\bconduct\b", "kənˈdʌkt"),
        // Default to noun
        (r"(?i)\bconduct\b", "ˈkɑːndʌkt"),
    ]);

    // Conducts (plural)
    m.insert("conducts", vec![
        // Conducts as noun (behavior)
        (r"(?i)\b(?:behavior|manner|way|ethics)\b.*?\bconducts\b", "ˈkɑːndʌkts"),
        // Conducts as verb (leads)
        (r"(?i)\b(?:lead|direct|manage|orchestra)\b.*?\bconducts\b", "kənˈdʌkts"),
        // Default to noun
        (r"(?i)\bconducts\b", "ˈkɑːndʌkts"),
    ]);

    // Perfect (ideal) vs. Perfect (to complete)
    m.insert("perfect", vec![
        // Perfect as adjective (ideal)
        (r"(?i)\b(?:ideal|flawless|excellent|impeccable|perfect|perfectly|ideal)\b.*?\bperfect\b", "ˈpɜːrfekt"),
        // Perfect as verb (to complete)
        (r"(?i)\b(?:complete|finish|make|ready|perfect|perfection|skill|practice)\b.*?\bperfect\b", "pərˈfekt"),
        // Default to adjective
        (r"(?i)\bperfect\b", "ˈpɜːrfekt"),
    ]);

    // Perfectly (adverb)
    m.insert("perfectly", vec![
        // Perfectly as adverb (ideally)
        (r"(?i)\b(?:ideal|flawless|excellent|impeccable|perfect|ideal)\b.*?\bperfectly\b", "ˈpɜːrfektli"),
        // Perfectly as adverb (completely)
        (r"(?i)\b(?:complete|finish|make|ready|perfect|perfection)\b.*?\bperfectly\b", "pərˈfektli"),
        // Default to ideal
        (r"(?i)\bperfectly\b", "ˈpɜːrfektli"),
    ]);

    // Combine (harvester) vs. Combine (to join)
    m.insert("combine", vec![
        // Combine as noun (harvester)
        (r"(?i)\b(?:machinery|harvester|farm|agriculture|combine|harvest|field|crop)\b.*?\bcombine\b", "ˈkɑːmbaɪn"),
        // Combine as verb (to join)
        (r"(?i)\b(?:join|merge|unite|mix|combine|combination|together|blend)\b.*?\bcombine\b", "kəmˈbaɪn"),
        // Default to verb
        (r"(?i)\bcombine\b", "kəmˈbaɪn"),
    ]);

    // Combined (past tense)
    m.insert("combined", vec![
        // Combined as noun (harvester)
        (r"(?i)\b(?:machinery|harvester|farm|agriculture)\b.*?\bcombined\b", "ˈkɑːmbaɪnd"),
        // Combined as verb (joined)
        (r"(?i)\b(?:join|merge|unite|mix)\b.*?\bcombined\b", "kəmˈbaɪnd"),
        // Default to verb
        (r"(?i)\bcombined\b", "kəmˈbaɪnd"),
    ]);

    // Compact (small) vs. Compact (agreement)
    m.insert("compact", vec![
        // Compact as adjective (small)
        (r"(?i)\b(?:small|dense|squeeze|tiny|compact|space-saving|portable|miniature)\b.*?\bcompact\b", "ˈkɑːmpækt"),
        // Compact as noun (agreement)
        (r"(?i)\b(?:agreement|deal|pact|treaty|compact|contract|formal|written)\b.*?\bcompact\b", "kəmˈpækt"),
        // Default to adjective
        (r"(?i)\bcompact\b", "ˈkɑːmpækt"),
    ]);

    // Compactness (noun form)
    m.insert("compactness", vec![
        // Compactness as noun (small size)
        (r"(?i)\b(?:small|dense|squeeze|tiny)\b.*?\bcompactness\b", "ˈkɑːmpæktnəs"),
        // Compactness as noun (agreement quality)
        (r"(?i)\b(?:agreement|deal|pact|treaty)\b.*?\bcompactness\b", "kəmˈpæktnəs"),
        // Default to small size
        (r"(?i)\bcompactness\b", "ˈkɑːmpæktnəs"),
    ]);

    // Project (plan) vs. Project (to extend)
    m.insert("project", vec![
        // Project as noun (plan)
        (r"(?i)\b(?:plan|scheme|endeavor|venture|project|initiative|proposal|research)\b.*?\bproject\b", "ˈprɑːdʒekt"),
        // Project as verb (to extend)
        (r"(?i)\b(?:extend|jut|protrude|cast|project|projection|screen|display)\b.*?\bproject\b", "prəˈdʒekt"),
        // Default to noun
        (r"(?i)\bproject\b", "ˈprɑːdʒekt"),
    ]);

    // Projects (plural)
    m.insert("projects", vec![
        // Projects as noun (plans)
        (r"(?i)\b(?:plan|scheme|endeavor|venture)\b.*?\bprojects\b", "ˈprɑːdʒekts"),
        // Projects as verb (extends)
        (r"(?i)\b(?:extend|jut|protrude|cast)\b.*?\bprojects\b", "prəˈdʒekts"),
        // Default to noun
        (r"(?i)\bprojects\b", "ˈprɑːdʒekts"),
    ]);

    // Subject (topic) vs. Subject (to expose)
    m.insert("subject", vec![
        // Subject as noun (topic)
        (r"(?i)\b(?:topic|theme|matter|issue|subject|discussion|academic|school)\b.*?\bsubject\b", "ˈsʌbdʒekt"),
        // Subject as verb (to expose)
        (r"(?i)\b(?:under|beneath|subordinate|expose|subject|subjects|subjection)\b.*?\bsubject\b", "səbˈdʒekt"),
        // Default to noun
        (r"(?i)\bsubject\b", "ˈsʌbdʒekt"),
    ]);

    // Subjected (past tense)
    m.insert("subjected", vec![
        // Subjected as noun (topic)
        (r"(?i)\b(?:topic|theme|matter|issue)\b.*?\bsubjected\b", "ˈsʌbdʒektɪd"),
        // Subjected as verb (exposed)
        (r"(?i)\b(?:under|beneath|subordinate|expose)\b.*?\bsubjected\b", "səbˈdʒektɪd"),
        // Default to verb
        (r"(?i)\bsubjected\b", "səbˈdʒektɪd"),
    ]);

    // Conflict (war) vs. Conflict (to disagree)
    m.insert("conflict", vec![
        // Conflict as noun (war)
        (r"(?i)\b(?:war|battle|struggle|fight|conflict|armed|international|violent)\b.*?\bconflict\b", "ˈkɑːnflɪkt"),
        // Conflict as verb (to disagree)
        (r"(?i)\b(?:disagree|clash|oppose|contradict|conflict|conflicting|values|interests)\b.*?\bconflict\b", "kənˈflɪkt"),
        // Default to noun
        (r"(?i)\bconflict\b", "ˈkɑːnflɪkt"),
    ]);

    // Conflicts (plural)
    m.insert("conflicts", vec![
        // Conflicts as noun (wars)
        (r"(?i)\b(?:war|battle|struggle|fight)\b.*?\bconflicts\b", "ˈkɑːnflɪkts"),
        // Conflicts as verb (disagrees)
        (r"(?i)\b(?:disagree|clash|oppose|contradict)\b.*?\bconflicts\b", "kənˈflɪkts"),
        // Default to noun
        (r"(?i)\bconflicts\b", "ˈkɑːnflɪkts"),
    ]);

    // Permit (allow) vs. Permit (document)
    m.insert("permit", vec![
        // Permit as verb (allow)
        (r"(?i)\b(?:allow|authorize|license|sanction|permit|permits|permitted|permission)\b.*?\bpermit\b", "pərˈmɪt"),
        // Permit as noun (document)
        (r"(?i)\b(?:license|ticket|authorization|document|permit|permits|building|parking)\b.*?\bpermit\b", "ˈpɜːrmɪt"),
        // Default to verb
        (r"(?i)\bpermit\b", "pərˈmɪt"),
    ]);

    // Permits (plural)
    m.insert("permits", vec![
        // Permits as verb (allows)
        (r"(?i)\b(?:allow|authorize|license|sanction)\b.*?\bpermits\b", "pərˈmɪts"),
        // Permits as noun (documents)
        (r"(?i)\b(?:license|ticket|authorization|document)\b.*?\bpermits\b", "ˈpɜːrmɪts"),
        // Default to noun
        (r"(?i)\bpermits\b", "ˈpɜːrmɪts"),
    ]);

    // Increase (grow) vs. Increase (rise)
    m.insert("increase", vec![
        // Increase as verb (grow)
        (r"(?i)\b(?:grow|expand|enlarge|rise|increase|increases|increasing|growth)\b.*?\bincrease\b", "ɪnˈkris"),
        // Increase as noun (rise)
        (r"(?i)\b(?:rise|growth|gain|boost|increase|increases|increment|amount)\b.*?\bincrease\b", "ˈɪnkriːs"),
        // Default to verb
        (r"(?i)\bincrease\b", "ɪnˈkris"),
    ]);

    // Increases (plural)
    m.insert("increases", vec![
        // Increases as verb (grows)
        (r"(?i)\b(?:grow|expand|enlarge|rise)\b.*?\bincreases\b", "ɪnˈkrises"),
        // Increases as noun (rises)
        (r"(?i)\b(?:rise|growth|gain|boost)\b.*?\bincreases\b", "ˈɪnkriːsɪz"),
        // Default to verb
        (r"(?i)\bincreases\b", "ɪnˈkrises"),
    ]);

    // Decrease (shrink) vs. Decrease (fall)
    m.insert("decrease", vec![
        // Decrease as verb (shrink)
        (r"(?i)\b(?:shrink|reduce|diminish|fall|decrease|decreases|decreasing|reduction)\b.*?\bdecrease\b", "diːˈkris"),
        // Decrease as noun (fall)
        (r"(?i)\b(?:fall|drop|decline|reduction|decrease|decreases|amount|rate)\b.*?\bdecrease\b", "ˈdiːkriːs"),
        // Default to verb
        (r"(?i)\bdecrease\b", "diːˈkris"),
    ]);

    // Decreases (plural)
    m.insert("decreases", vec![
        // Decreases as verb (shrinks)
        (r"(?i)\b(?:shrink|reduce|diminish|fall)\b.*?\bdecreases\b", "diːˈkrises"),
        // Decreases as noun (falls)
        (r"(?i)\b(?:fall|drop|decline|reduction)\b.*?\bdecreases\b", "ˈdiːkriːsɪz"),
        // Default to verb
        (r"(?i)\bdecreases\b", "diːˈkrises"),
    ]);

    // Insult (offend) vs. Insult (remark)
    m.insert("insult", vec![
        // Insult as verb (offend)
        (r"(?i)\b(?:offend|offensive|disrespect|rudeness|insult|insults|insulting|verbal)\b.*?\binsult\b", "ɪnˈsʌlt"),
        // Insult as noun (remark)
        (r"(?i)\b(?:comment|remark|speak|utterance|insult|insults|verbal|abuse)\b.*?\binsult\b", "ˈɪnsʌlt"),
        // Default to verb
        (r"(?i)\binsult\b", "ɪnˈsʌlt"),
    ]);

    // Insults (plural)
    m.insert("insults", vec![
        // Insults as verb (offends)
        (r"(?i)\b(?:offend|offensive|disrespect|rudeness)\b.*?\binsults\b", "ɪnˈsʌlts"),
        // Insults as noun (remarks)
        (r"(?i)\b(?:comment|remark|speak|utterance)\b.*?\binsults\b", "ˈɪnsʌlts"),
        // Default to verb
        (r"(?i)\binsults\b", "ɪnˈsʌlts"),
    ]);

    // Progress (advance) vs. Progress (to move)
    m.insert("progress", vec![
        // Progress as noun (advance)
        (r"(?i)\b(?:advance|development|improvement|headway|progress|progression|positive|significant)\b.*?\bprogress\b", "ˈprɑːɡres"),
        // Progress as verb (to move)
        (r"(?i)\b(?:move|go|walk|travel|progress|progressing|slowly|steadily)\b.*?\bprogress\b", "prəˈɡres"),
        // Default to noun
        (r"(?i)\bprogress\b", "ˈprɑːɡres"),
    ]);

    // Progresses (plural)
    m.insert("progresses", vec![
        // Progresses as noun (advances)
        (r"(?i)\b(?:advance|development|improvement|headway)\b.*?\bprogresses\b", "ˈprɑːɡresɪz"),
        // Progresses as verb (moves)
        (r"(?i)\b(?:move|go|walk|travel)\b.*?\bprogresses\b", "prəˈɡresɪz"),
        // Default to noun
        (r"(?i)\bprogresses\b", "ˈprɑːɡresɪz"),
    ]);

    // Refuse (garbage) vs. Refuse (deny)
    m.insert("refuse", vec![
        // Refuse as noun (garbage)
        (r"(?i)\b(?:garbage|trash|waste|rubbish|refuse|refuses|disposal|collection)\b.*?\brefuse\b", "ˈrefjuːs"),
        // Refuse as verb (deny)
        (r"(?i)\b(?:deny|reject|decline|refuse|refuses|refusing|application|request)\b.*?\brefuse\b", "rɪˈfjuːz"),
        // Default to verb
        (r"(?i)\brefuse\b", "rɪˈfjuːz"),
    ]);

    // Refuses (plural)
    m.insert("refuses", vec![
        // Refuses as noun (garbage)
        (r"(?i)\b(?:garbage|trash|waste|rubbish)\b.*?\brefuses\b", "ˈrefjuːsɪz"),
        // Refuses as verb (denies)
        (r"(?i)\b(?:deny|reject|decline|decline)\b.*?\brefuses\b", "rɪˈfjuːzɪz"),
        // Default to verb
        (r"(?i)\brefuses\b", "rɪˈfjuːzɪz"),
    ]);

    // Separate (apart) vs. Separate (to divide)
    m.insert("separate", vec![
        // Separate as adjective (apart)
        (r"(?i)\b(?:apart|divided|disconnected|individual|separate|distinct|different|unique)\b.*?\bseparate\b", "ˈsepəreɪt"),
        // Separate as verb (to divide)
        (r"(?i)\b(?:divide|split|detach|disconnect|separate|separates|separating|carefully)\b.*?\bseparate\b", "ˈsepəreɪt"),
        // Default to adjective
        (r"(?i)\bseparate\b", "ˈsepəreɪt"),
    ]);

    // Separates (plural)
    m.insert("separates", vec![
        // Separates as adjective (apart)
        (r"(?i)\b(?:apart|divided|disconnected|individual)\b.*?\bseparates\b", "ˈsepəreɪts"),
        // Separates as verb (divides)
        (r"(?i)\b(?:divide|split|detach|disconnect)\b.*?\bseparates\b", "ˈsepəreɪts"),
        // Default to verb
        (r"(?i)\bseparates\b", "ˈsepəreɪts"),
    ]);

    // Estimate (guess) vs. Estimate (assessment)
    m.insert("estimate", vec![
        // Estimate as verb (guess)
        (r"(?i)\b(?:guess|approximate|roughly|calculate|guesstimate|estimate|estimates|estimating)\b.*?\bestimate\b", "ˈestəmət"),
        // Estimate as noun (assessment)
        (r"(?i)\b(?:estimate|assessment|appraisal|evaluation|estimate|estimates|cost|time)\b.*?\bestimate\b", "ˈestəmət"),
        // Default to verb
        (r"(?i)\bestimate\b", "ˈestəmət"),
    ]);

    // Estimates (plural)
    m.insert("estimates", vec![
        // Estimates as verb (guesses)
        (r"(?i)\b(?:guess|approximate|roughly|calculate|guesstimate)\b.*?\bestimates\b", "ˈestəməts"),
        // Estimates as noun (assessments)
        (r"(?i)\b(?:estimate|assessment|appraisal|evaluation)\b.*?\bestimates\b", "ˈestəməts"),
        // Default to verb
        (r"(?i)\bestimates\b", "ˈestəməts"),
    ]);

    // Resume (continue) vs. Resume (CV)
    m.insert("resume", vec![
        // Resume as verb (continue)
        (r"(?i)\b(?:continue|restart|recommence|proceed|resume|resumes|resuming|after|break)\b.*?\bresume\b", "rɪˈzuːm"),
        // Resume as noun (CV)
        (r"(?i)\b(?:cv|curriculum|vitae|job|application|professional|resume|resumes|document)\b.*?\bresume\b", "ˈrezəmeɪ"),
        // Default to verb
        (r"(?i)\bresume\b", "rɪˈzuːm"),
    ]);

    // Resumes (plural)
    m.insert("resumes", vec![
        // Resumes as verb (continues)
        (r"(?i)\b(?:continue|restart|recommence|proceed)\b.*?\bresumes\b", "rɪˈzuːmz"),
        // Resumes as noun (CVs)
        (r"(?i)\b(?:cv|curriculum|vitae|job|application)\b.*?\bresumes\b", "ˈrezəmeɪz"),
        // Default to verb
        (r"(?i)\bresumes\b", "rɪˈzuːmz"),
    ]);

    m
});
