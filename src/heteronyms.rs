use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static HETERONYMS: Lazy<HashMap<&'static str, Vec<(&'static str, &'static str)>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    m.insert("wind", vec![
        (r"(?i).*\b(blows?|breeze|gust|storm|hurricane|air|weather|north|south|east|west|strong|gentle)\b.*\bwind\b", "wɪnd"),
        (r"(?i).*\bwind\b.*\b(clock|watch|bobbin|spool|tape|up|down|tight|loose|around|the)\b.*", "waɪnd"),
        (r"(?i).*\bwind-?up\b.*", "ˈwaɪnd ʌp"),
        (r"(?i).*\bwind\b.*", "wɪnd"),
    ]);

    m.insert("winds", vec![
        (r"(?i).*\b(windy|blowing|blustery|howling|whistling)\b.*\bwinds\b", "wɪndz"),
        (r"(?i).*\bwinds\b.*\b(clock|watch|bobbin|spool|tape)\b.*", "waɪndz"),
        (r"(?i).*\bwinds\b.*", "wɪndz"),
    ]);

    m.insert("bass", vec![
        (r"(?i).*\b(singer|voice|soprano|alto|tenor|choir|sang|sings)\b.*", "beɪs"),
        (r"(?i).*\b(fish|fishing|lake|river|lure|drum|guitar)\b.*", "bæs"),
        (r"(?i).*\bbass\b.*", "bæs"),
    ]);

    m.insert("live", vec![
        (r"(?i).*\b(reside|dwelling|home|inhabiting)\b.*\blive\b", "lɪv"),
        (r"(?i).*\b(broadcast|performance|show|concert|streaming)\b.*\blive\b", "laɪv"),
        (r"(?i).*\blive\b.*", "lɪv"),
    ]);

    m.insert("lives", vec![
        (r"(?i).*\b(reside|dwelling|home|inhabiting)\b.*\blives\b", "lɪvz"),
        (r"(?i).*\b(broadcast|performance|show|concert|streaming)\b.*\blives\b", "laɪvz"),
        (r"(?i).*\blives\b.*", "lɪvz"),
    ]);

    m.insert("read", vec![
        (r"(?i).*\b(past|read|book|novel|story|yesterday|finished)\b.*\bread\b", "red"),
        (r"(?i).*\b(looking at|studying|examining|now)\b.*\bread\b", "riːd"),
        (r"(?i).*\bread\b.*", "riːd"),
    ]);

    m.insert("tear", vec![
        (r"(?i).*\b(rip|shred|tore|torn|scratch)\b.*\btear\b", "tɛr"),
        (r"(?i).*\b(cry|weep|sad|eye|drop)\b.*\btear\b", "tɪr"),
        (r"(?i).*\btear\b.*", "tɪr"),
    ]);

    m.insert("tears", vec![
        (r"(?i).*\b(rip|shred|tore|torn|scratch)\b.*\btears\b", "tɛrz"),
        (r"(?i).*\b(cry|weep|sad|eye|drop)\b.*\btears\b", "tɪrz"),
        (r"(?i).*\btears\b.*", "tɪrz"),
    ]);

    m.insert("bow", vec![
        (r"(?i).*\b(front|down|curtsy|respect)\b.*\bbow\b", "baʊ"),
        (r"(?i).*\b(arrow|violin|string|ship|stern)\b.*\bbow\b", "boʊ"),
        (r"(?i).*\bbow\b.*", "boʊ"),
    ]);

    m.insert("row", vec![
        (r"(?i).*\b(argument|fight|quarrel|dispute)\b.*\brow\b", "raʊ"),
        (r"(?i).*\b(boat|paddle|oar|river)\b.*\brow\b", "roʊ"),
        (r"(?i).*\brow\b.*", "roʊ"),
    ]);

    m.insert("lead", vec![
        (r"(?i).*\b(guided|followed|directed|managed)\b.*\blead\b", "led"),
        (r"(?i).*\b(metal|pencil|weight|first)\b.*\blead\b", "liːd"),
        (r"(?i).*\blead\b.*", "led"),
    ]);

    m.insert("leads", vec![
        (r"(?i).*\b(guided|followed|directed|managed)\b.*\bleads\b", "ledz"),
        (r"(?i).*\b(metal|pencil|weight|first)\b.*\bleads\b", "liːdz"),
        (r"(?i).*\bleads\b.*", "ledz"),
    ]);

    m.insert("close", vec![
        (r"(?i).*\b(near|proximity|adjacent)\b.*\bclose\b", "kloʊs"),
        (r"(?i).*\b(shut|door|window|lid)\b.*\bclose\b", "kloʊz"),
        (r"(?i).*\bclose\b.*", "kloʊz"),
    ]);

    m.insert("closed", vec![
        (r"(?i).*\b(near|proximity|adjacent)\b.*\bclosed\b", "kloʊst"),
        (r"(?i).*\b(shut|door|window|lid)\b.*\bclosed\b", "kloʊzd"),
        (r"(?i).*\bclosed\b.*", "kloʊzd"),
    ]);

    m.insert("content", vec![
        (r"(?i).*\b(satisfied|pleased|happy|glad)\b.*\bcontent\b", "kənˈtent"),
        (r"(?i).*\b(material|substance|inside|table of)\b.*\bcontent\b", "ˈkɑːntent"),
        (r"(?i).*\bcontent\b.*", "ˈkɑːntent"),
    ]);

    m.insert("record", vec![
        (r"(?i).*\b(document|tape|disc|album|music)\b.*\brecord\b", "ˈrekərd"),
        (r"(?i).*\b(break|achieve|set|history)\b.*\brecord\b", "rɪˈkɔːrd"),
        (r"(?i).*\brecord\b.*", "ˈrekərd"),
    ]);

    m.insert("records", vec![
        (r"(?i).*\b(document|tape|disc|album|music)\b.*\brecords\b", "ˈrekərdz"),
        (r"(?i).*\b(break|achieve|set|history)\b.*\brecords\b", "rɪˈkɔːrdz"),
        (r"(?i).*\brecords\b.*", "ˈrekərdz"),
    ]);

    m.insert("produce", vec![
        (r"(?i).*\b(fruit|vegetable|farm|agriculture)\b.*\bproduce\b", "ˈprɑːdus"),
        (r"(?i).*\b(create|make|generate|result)\b.*\bproduce\b", "prəˈdus"),
        (r"(?i).*\bproduce\b.*", "prəˈdus"),
    ]);

    m.insert("present", vec![
        (r"(?i).*\b(gift|present|package|wrapped)\b.*\bpresent\b", "ˈprezənt"),
        (r"(?i).*\b(now|current|time|here)\b.*\bpresent\b", "prɪˈzent"),
        (r"(?i).*\bpresent\b.*", "prɪˈzent"),
    ]);

    m.insert("presented", vec![
        (r"(?i).*\b(gift|present|package|wrapped)\b.*\bpresented\b", "ˈprezəntɪd"),
        (r"(?i).*\b(now|current|time|here)\b.*\bpresented\b", "prɪˈzentɪd"),
        (r"(?i).*\bpresented\b.*", "prɪˈzentɪd"),
    ]);

    m.insert("object", vec![
        (r"(?i).*\b(thing|item|physical|tangible)\b.*\bobject\b", "ˈɑːbdʒekt"),
        (r"(?i).*\b(oppose|disagree|argue|protest)\b.*\bobject\b", "əbˈdʒekt"),
        (r"(?i).*\bobject\b.*", "ˈɑːbdʒekt"),
    ]);

    m.insert("objected", vec![
        (r"(?i).*\b(thing|item|physical|tangible)\b.*\bobjected\b", "ˈɑːbdʒektɪd"),
        (r"(?i).*\b(oppose|disagree|argue|protest)\b.*\bobjected\b", "əbˈdʒektɪd"),
        (r"(?i).*\bobjected\b.*", "əbˈdʒektɪd"),
    ]);

    m.insert("contract", vec![
        (r"(?i).*\b(agreement|legal|sign|deal)\b.*\bcontract\b", "ˈkɑːntrækt"),
        (r"(?i).*\b(shrink|tighten|muscle|wrinkle)\b.*\bcontract\b", "kənˈtrækt"),
        (r"(?i).*\bcontract\b.*", "ˈkɑːntrækt"),
    ]);

    m.insert("contracts", vec![
        (r"(?i).*\b(agreement|legal|sign|deal)\b.*\bcontracts\b", "ˈkɑːntrækts"),
        (r"(?i).*\b(shrink|tighten|muscle|wrinkle)\b.*\bcontracts\b", "kənˈtrækts"),
        (r"(?i).*\bcontracts\b.*", "ˈkɑːntrækts"),
    ]);

    m.insert("desert", vec![
        (r"(?i).*\b(arid|sand|dune|Sahara)\b.*\bdesert\b", "ˈdezərt"),
        (r"(?i).*\b(abandon|leave|forsake|defect)\b.*\bdesert\b", "dɪˈzɜːrt"),
        (r"(?i).*\bdesert\b.*", "ˈdezərt"),
    ]);

    m.insert("deserted", vec![
        (r"(?i).*\b(arid|sand|dune|Sahara)\b.*\bdeserted\b", "ˈdezərtɪd"),
        (r"(?i).*\b(abandon|leave|forsake|defect)\b.*\bdeserted\b", "dɪˈzɜːrtɪd"),
        (r"(?i).*\bdeserted\b.*", "dɪˈzɜːrtɪd"),
    ]);

    m.insert("minute", vec![
        (r"(?i).*\b(tiny|small|infinitesimal|wee)\b.*\bminute\b", "maɪˈnut"),
        (r"(?i).*\b(60|second|time|hour)\b.*\bminute\b", "ˈmɪnɪt"),
        (r"(?i).*\bminute\b.*", "ˈmɪnɪt"),
    ]);

    m.insert("minutely", vec![
        (r"(?i).*\b(tiny|small|infinitesimal|wee)\b.*\bminutely\b", "maɪˈnutli"),
        (r"(?i).*\b(60|second|time|hour)\b.*\bminutely\b", "ˈmɪnɪtli"),
        (r"(?i).*\bminutely\b.*", "ˈmɪnɪtli"),
    ]);

    m.insert("invalid", vec![
        (r"(?i).*\b(not valid|wrong|incorrect|void)\b.*\binvalid\b", "ɪnˈvælɪd"),
        (r"(?i).*\b(patient|hospital|sick|wheelchair)\b.*\binvalid\b", "ˈɪnvəlɪd"),
        (r"(?i).*\binvalid\b.*", "ɪnˈvælɪd"),
    ]);

    m.insert("use", vec![
        (r"(?i).*\b(utilize|employ|apply|function)\b.*\buse\b", "juːz"),
        (r"(?i).*\b(purpose|reason|benefit|utility)\b.*\buse\b", "juːs"),
        (r"(?i).*\buse\b.*", "juːz"),
    ]);

    m.insert("used", vec![
        (r"(?i).*\b(utilize|employ|apply|function)\b.*\bused\b", "juːzd"),
        (r"(?i).*\b(purpose|reason|benefit|utility)\b.*\bused\b", "juːst"),
        (r"(?i).*\bused\b.*", "juːzd"),
    ]);

    m.insert("abuse", vec![
        (r"(?i).*\b(misuse|mistreat|exploit|wrong)\b.*\babuse\b", "əˈbjuːz"),
        (r"(?i).*\b(misuse|mistreat|exploit|wrong)\b.*\babuse\b", "əˈbjuːs"),
        (r"(?i).*\babuse\b.*", "əˈbjuːz"),
    ]);

    m.insert("abused", vec![
        (r"(?i).*\b(misuse|mistreat|exploit|wrong)\b.*\babused\b", "əˈbjuːzd"),
        (r"(?i).*\b(misuse|mistreat|exploit|wrong)\b.*\babused\b", "əˈbjuːst"),
        (r"(?i).*\babused\b.*", "əˈbjuːzd"),
    ]);

    m.insert("conduct", vec![
        (r"(?i).*\b(behavior|manner|way|ethics)\b.*\bconduct\b", "ˈkɑːndʌkt"),
        (r"(?i).*\b(lead|direct|manage|orchestra)\b.*\bconduct\b", "kənˈdʌkt"),
        (r"(?i).*\bconduct\b.*", "ˈkɑːndʌkt"),
    ]);

    m.insert("conducts", vec![
        (r"(?i).*\b(behavior|manner|way|ethics)\b.*\bconducts\b", "ˈkɑːndʌkts"),
        (r"(?i).*\b(lead|direct|manage|orchestra)\b.*\bconducts\b", "kənˈdʌkts"),
        (r"(?i).*\bconducts\b.*", "ˈkɑːndʌkts"),
    ]);

    m.insert("perfect", vec![
        (r"(?i).*\b(ideal|flawless|excellent|impeccable)\b.*\bperfect\b", "ˈpɜːrfekt"),
        (r"(?i).*\b(complete|finish|make ready)\b.*\bperfect\b", "pərˈfekt"),
        (r"(?i).*\bperfect\b.*", "ˈpɜːrfekt"),
    ]);

    m.insert("perfectly", vec![
        (r"(?i).*\b(ideal|flawless|excellent|impeccable)\b.*\bperfectly\b", "ˈpɜːrfektli"),
        (r"(?i).*\b(complete|finish|make ready)\b.*\bperfectly\b", "pərˈfektli"),
        (r"(?i).*\bperfectly\b.*", "ˈpɜːrfektli"),
    ]);

    m.insert("combine", vec![
        (r"(?i).*\b(machinery|harvester|farm|agriculture)\b.*\bcombine\b", "ˈkɑːmbaɪn"),
        (r"(?i).*\b(join|merge|unite|mix)\b.*\bcombine\b", "kəmˈbaɪn"),
        (r"(?i).*\bcombine\b.*", "kəmˈbaɪn"),
    ]);

    m.insert("combined", vec![
        (r"(?i).*\b(machinery|harvester|farm|agriculture)\b.*\bcombined\b", "ˈkɑːmbaɪnd"),
        (r"(?i).*\b(join|merge|unite|mix)\b.*\bcombined\b", "kəmˈbaɪnd"),
        (r"(?i).*\bcombined\b.*", "kəmˈbaɪnd"),
    ]);

    m.insert("compact", vec![
        (r"(?i).*\b(small|dense|squeeze|tiny)\b.*\bcompact\b", "ˈkɑːmpækt"),
        (r"(?i).*\b(agreement|deal|pact|treaty)\b.*\bcompact\b", "kəmˈpækt"),
        (r"(?i).*\bcompact\b.*", "ˈkɑːmpækt"),
    ]);

    m.insert("compactness", vec![
        (r"(?i).*\b(small|dense|squeeze|tiny)\b.*\bcompactness\b", "ˈkɑːmpæktnəs"),
        (r"(?i).*\b(agreement|deal|pact|treaty)\b.*\bcompactness\b", "kəmˈpæktnəs"),
        (r"(?i).*\bcompactness\b.*", "ˈkɑːmpæktnəs"),
    ]);

    m.insert("project", vec![
        (r"(?i).*\b(plan|scheme|endeavor|venture)\b.*\bproject\b", "ˈprɑːdʒekt"),
        (r"(?i).*\b(extend|jut|protrude|cast)\b.*\bproject\b", "prəˈdʒekt"),
        (r"(?i).*\bproject\b.*", "ˈprɑːdʒekt"),
    ]);

    m.insert("projects", vec![
        (r"(?i).*\b(plan|scheme|endeavor|venture)\b.*\bprojects\b", "ˈprɑːdʒekts"),
        (r"(?i).*\b(extend|jut|protrude|cast)\b.*\bprojects\b", "prəˈdʒekts"),
        (r"(?i).*\bprojects\b.*", "ˈprɑːdʒekts"),
    ]);

    m.insert("subject", vec![
        (r"(?i).*\b(topic|theme|matter|issue)\b.*\bsubject\b", "ˈsʌbdʒekt"),
        (r"(?i).*\b(under|beneath|subordinate|expose)\b.*\bsubject\b", "səbˈdʒekt"),
        (r"(?i).*\bsubject\b.*", "ˈsʌbdʒekt"),
    ]);

    m.insert("subjected", vec![
        (r"(?i).*\b(topic|theme|matter|issue)\b.*\bsubjected\b", "ˈsʌbdʒektɪd"),
        (r"(?i).*\b(under|beneath|subordinate|expose)\b.*\bsubjected\b", "səbˈdʒektɪd"),
        (r"(?i).*\bsubjected\b.*", "səbˈdʒektɪd"),
    ]);

    m.insert("conflict", vec![
        (r"(?i).*\b(war|battle|struggle|fight)\b.*\bconflict\b", "ˈkɑːnflɪkt"),
        (r"(?i).*\b(disagree|clash|oppose|contradict)\b.*\bconflict\b", "kənˈflɪkt"),
        (r"(?i).*\bconflict\b.*", "ˈkɑːnflɪkt"),
    ]);

    m.insert("conflicts", vec![
        (r"(?i).*\b(war|battle|struggle|fight)\b.*\bconflicts\b", "ˈkɑːnflɪkts"),
        (r"(?i).*\b(disagree|clash|oppose|contradict)\b.*\bconflicts\b", "kənˈflɪkts"),
        (r"(?i).*\bconflicts\b.*", "ˈkɑːnflɪkts"),
    ]);

    m.insert("permit", vec![
        (r"(?i).*\b(allow|authorize|license|sanction)\b.*\bpermit\b", "pərˈmɪt"),
        (r"(?i).*\b(license|ticket|authorization|document)\b.*\bpermit\b", "ˈpɜːrmɪt"),
        (r"(?i).*\bpermit\b.*", "pərˈmɪt"),
    ]);

    m.insert("permits", vec![
        (r"(?i).*\b(allow|authorize|license|sanction)\b.*\bpermits\b", "pərˈmɪts"),
        (r"(?i).*\b(license|ticket|authorization|document)\b.*\bpermits\b", "ˈpɜːrmɪts"),
        (r"(?i).*\bpermits\b.*", "ˈpɜːrmɪts"),
    ]);

    m.insert("increase", vec![
        (r"(?i).*\b(grow|expand|enlarge|rise)\b.*\bincrease\b", "ɪnˈkris"),
        (r"(?i).*\b(rise|growth|gain|boost)\b.*\bincrease\b", "ˈɪnkriːs"),
        (r"(?i).*\bincrease\b.*", "ɪnˈkris"),
    ]);

    m.insert("increases", vec![
        (r"(?i).*\b(grow|expand|enlarge|rise)\b.*\bincreases\b", "ɪnˈkrises"),
        (r"(?i).*\b(rise|growth|gain|boost)\b.*\bincreases\b", "ˈɪnkriːsɪz"),
        (r"(?i).*\bincreases\b.*", "ɪnˈkrises"),
    ]);

    m.insert("decrease", vec![
        (r"(?i).*\b(shrink|reduce|diminish|fall)\b.*\bdecrease\b", "diːˈkris"),
        (r"(?i).*\b(fall|drop|decline|reduction)\b.*\bdecrease\b", "ˈdiːkriːs"),
        (r"(?i).*\bdecrease\b.*", "diːˈkris"),
    ]);

    m.insert("decreases", vec![
        (r"(?i).*\b(shrink|reduce|diminish|fall)\b.*\bdecreases\b", "diːˈkrises"),
        (r"(?i).*\b(fall|drop|decline|reduction)\b.*\bdecreases\b", "ˈdiːkriːsɪz"),
        (r"(?i).*\bdecreases\b.*", "diːˈkrises"),
    ]);

    m.insert("insult", vec![
        (r"(?i).*\b(offend|offensive|disrespect|rudeness)\b.*\binsult\b", "ɪnˈsʌlt"),
        (r"(?i).*\b(comment|remark|speak|utterance)\b.*\binsult\b", "ˈɪnsʌlt"),
        (r"(?i).*\binsult\b.*", "ɪnˈsʌlt"),
    ]);

    m.insert("insults", vec![
        (r"(?i).*\b(offend|offensive|disrespect|rudeness)\b.*\binsults\b", "ɪnˈsʌlts"),
        (r"(?i).*\b(comment|remark|speak|utterance)\b.*\binsults\b", "ˈɪnsʌlts"),
        (r"(?i).*\binsults\b.*", "ɪnˈsʌlts"),
    ]);

    m.insert("progress", vec![
        (r"(?i).*\b(advance|development|improvement|headway)\b.*\bprogress\b", "ˈprɑːɡres"),
        (r"(?i).*\b(move|go|walk|travel)\b.*\bprogress\b", "prəˈɡres"),
        (r"(?i).*\bprogress\b.*", "ˈprɑːɡres"),
    ]);

    m.insert("progresses", vec![
        (r"(?i).*\b(advance|development|improvement|headway)\b.*\bprogresses\b", "ˈprɑːɡresɪz"),
        (r"(?i).*\b(move|go|walk|travel)\b.*\bprogresses\b", "prəˈɡresɪz"),
        (r"(?i).*\bprogresses\b.*", "ˈprɑːɡresɪz"),
    ]);

    m.insert("produce", vec![
        (r"(?i).*\b(fruit|vegetable|farm|agriculture)\b.*\bproduce\b", "ˈprɑːdus"),
        (r"(?i).*\b(create|make|generate|result)\b.*\bproduce\b", "prəˈdus"),
        (r"(?i).*\bproduce\b.*", "prəˈdus"),
    ]);

    m.insert("produces", vec![
        (r"(?i).*\b(fruit|vegetable|farm|agriculture)\b.*\bproduces\b", "ˈprɑːdusɪz"),
        (r"(?i).*\b(create|make|generate|result)\b.*\bproduces\b", "prəˈdusɪz"),
        (r"(?i).*\bproduces\b.*", "prəˈdusɪz"),
    ]);

    m.insert("refuse", vec![
        (r"(?i).*\b(garbage|trash|waste|rubbish)\b.*\brefuse\b", "ˈrefjuːs"),
        (r"(?i).*\b(deny|reject|decline|decline)\b.*\brefuse\b", "rɪˈfjuːz"),
        (r"(?i).*\brefuse\b.*", "rɪˈfjuːz"),
    ]);

    m.insert("refuses", vec![
        (r"(?i).*\b(garbage|trash|waste|rubbish)\b.*\brefuses\b", "ˈrefjuːsɪz"),
        (r"(?i).*\b(deny|reject|decline|decline)\b.*\brefuses\b", "rɪˈfjuːzɪz"),
        (r"(?i).*\brefuses\b.*", "rɪˈfjuːzɪz"),
    ]);

    m.insert("separate", vec![
        (r"(?i).*\b(apart|divided|disconnected|individual)\b.*\bseparate\b", "ˈsepəreɪt"),
        (r"(?i).*\b(divide|split|detach|disconnect)\b.*\bseparate\b", "ˈsepəreɪt"),
        (r"(?i).*\bseparate\b.*", "ˈsepəreɪt"),
    ]);

    m.insert("separates", vec![
        (r"(?i).*\b(apart|divided|disconnected|individual)\b.*\bseparates\b", "ˈsepəreɪts"),
        (r"(?i).*\b(divide|split|detach|disconnect)\b.*\bseparates\b", "ˈsepəreɪts"),
        (r"(?i).*\bseparates\b.*", "ˈsepəreɪts"),
    ]);

    m.insert("estimate", vec![
        (r"(?i).*\b(guess|approximate|roughly calculate|guesstimate)\b.*\bestimate\b", "ˈestəmət"),
        (r"(?i).*\b(estimate|assessment|appraisal|evaluation)\b.*\bestimate\b", "ˈestəmət"),
        (r"(?i).*\bestimate\b.*", "ˈestəmət"),
    ]);

    m.insert("estimates", vec![
        (r"(?i).*\b(guess|approximate|roughly calculate|guesstimate)\b.*\bestimates\b", "ˈestəməts"),
        (r"(?i).*\b(estimate|assessment|appraisal|evaluation)\b.*\bestimates\b", "ˈestəməts"),
        (r"(?i).*\bestimates\b.*", "ˈestəməts"),
    ]);

    m
});