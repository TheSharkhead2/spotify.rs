use crate::Error;

/// Object representing a market (ISO 3166-1 alpha-2 codes)
/// See: <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
pub enum Market {
    AD, // Andorra
    AE, // United Arab Emirates
    AF, // Afghanistan
    AG, // Antigua and Barbuda
    AI, // Anguilla
    AL, // Albania
    AM, // Armenia
    AO, // Angola
    AQ, // Antarctica
    AR, // Argentina
    AS, // American Somoa
    AT, // Austria
    AU, // Australia
    AW, // Aruba
    AX, // Aland Islands
    AZ, // Azerbaijan
    BA, // Bosnia and Herzegovina
    BB, // Barbados
    BD, // Bangladesh
    BE, // Belgium
    BF, // Burkina Faso
    BG, // Bulgaria
    BH, // Bahrain
    BI, // Burundi
    BJ, // Benin
    BL, // Saint Barthelemy
    BM, // Bermuda
    BN, // Brunei Darussalam
    BO, // Bolivia
    BQ, // Bonaire, Sint Eustatius and Saba
    BR, // Brazil
    BS, // Bahamas
    BT, // Bhutan
    BV, // Bouvet Island
    BW, // Botswana
    BY, // Belarus
    BZ, // Belize
    CA, // Canada
    CC, // Cocos Islands
    CD, // Democratic Republic of the Congo
    CF, // Central African Republic
    CG, // Congo
    CH, // Switzerland
    CI, // Cote dlvoire
    CK, // Cook Islands
    CL, // Chile
    CM, // Cameroon
    CN, // China
    CO, // Colombia
    CR, // Costa Rica
    CU, // Cuba
    CV, // Cabo Verde
    CW, // Curacao
    CX, // Christmas Island
    CY, // Cyprus
    CZ, // Czechia
    DE, // Germany
    DJ, // Djibouti
    DK, // Denmark
    DM, // Dominica
    DO, // Dominican Republic
    DZ, // Algeria
    EC, // Ecuador
    EE, // Estonia
    EG, // Egypt
    EH, // Western Sahara
    ER, // Eritrea
    ES, // Spain
    ET, // Ethiopia
    FI, // Finland
    FJ, // Fiji
    FK, // Falkland Islands
    FM, // Micronesia
    FO, // Faroe Islands
    FR, // France
    GA, // Gabon
    GB, // United Kingdom
    GD, // Grenada
    GE, // Georgia
    GF, // French Guiana
    GG, // Guernsey
    GH, // Ghana
    GI, // Gibraltar
    GL, // Greenland
    GM, // Gambia
    GN, // Guinea
    GP, // Guadeloupe
    GQ, // Equatorial Guinea
    GR, // Greece
    GS, // South Georgia
    GT, // Guatemala
    GU, // Guam
    GW, // Guinea-Bissau
    GY, // Guyana
    HK, // Hong Kong
    HM, // Heard Island
    HN, // Honduras
    HR, // Croatia
    HT, // Haiti
    HU, // Hungary
    ID, // Indonesia
    IE, // Ireland
    IL, // Israel
    IM, // Isle of Man
    IN, // India
    IO, // British Indian Ocean Territory
    IQ, // Iraq
    IR, // Iran
    IS, // Iceland
    IT, // Italy
    JE, // Jersey
    JM, // Jamaica
    JO, // Jordan
    JP, // Japan
    KE, // Kenya
    KG, // Kyrgyzstan
    KH, // Cambodia
    KI, // Kiribati
    KM, // Comoros
    KN, // Saint Kitts
    KP, // Democratic Peoples Republic of Korea (North Korea)
    KR, // Republic of Korea (South Korea)
    KW, // Kuwait
    KY, // Cayman Islands
    KZ, // Kazakhstan
    LA, // Lao Peoples Democratic Republic
    LB, // Lebanon
    LC, // Saint Lucia
    LI, // Liechtenstein
    LK, // Sri Lanka
    LR, // Liberia
    LS, // Lesotho
    LT, // Lithuania
    LU, // Luxembourg
    LV, // Latvia
    LY, // Libya
    MA, // Morocco
    MC, // Monaco
    MD, // Moldova
    ME, // Montenegro
    MF, // Saint Martin
    MG, // Madagascar
    MH, // Marshall Islands
    MK, // North Macedonia
    ML, // Mali
    MM, // Myanmar
    MN, // Mongolia
    MO, // Macao
    MP, // Northern Mariana Islands
    MQ, // Martinique
    MR, // Mauritania
    MS, // Montserrat
    MT, // Malta
    MU, // Mauritius
    MV, // Maldives
    MW, // Malawi
    MX, // Mexico
    MY, // Malaysia
    MZ, // Mozambique
    NA, // Namibia
    NC, // New Caledonia
    NE, // Niger
    NF, // Norfolk Island
    NG, // Nigeria
    NI, // Nicaragua
    NL, // Netherlands
    NO, // Norway
    NP, // Nepal
    NR, // Nauru
    NU, // Niue
    NZ, // New Zealand
    OM, // Oman
    PA, // Panama
    PE, // Peru
    PF, // French Polynesia
    PG, // Papua New Guinea
    PH, // Philippines
    PK, // Pakistan
    PL, // Poland
    PM, // Saint Pierre
    PN, // Pitcairn
    PR, // Puerto Rico
    PS, // Palestine
    PT, // Portugal
    PW, // Palau
    PY, // Paraguay
    QA, // Qatar
    RE, // Reunion
    RO, // Romania
    RS, // Serbia
    RU, // Russian Federation
    RW, // Rwanda
    SA, // Saudi Arabia
    SB, // Solomon Islands
    SC, // Seychelles
    SD, // Sudan
    SE, // Sweden
    SG, // Singapore
    SH, // Saint Helena
    SI, // Slovenia
    SJ, // Svalbard
    SK, // Slovakia
    SL, // Sierra Leone
    SM, // San Marino
    SN, // Senegal
    SO, // Somalia
    SR, // Suriname
    SS, // South Sudan
    ST, // Sao Tome
    SV, // El Salvador
    SX, // Sint Maarten
    SY, // Syrian Arab Republic
    SZ, // Eswatini
    TC, // Turks
    TD, // Chad
    TF, // French Southern Territories
    TG, // Togo
    TH, // Thailand
    TJ, // Tajikistan
    TK, // Tokelau
    TL, // Timor-Leste
    TM, // Turkmenistan
    TN, // Tunisia
    TO, // Tonga
    TR, // Turkiye
    TT, // Trinidad
    TV, // Tuvalu
    TW, // Taiwan
    TZ, // Tanzania
    UA, // Ukraine
    UG, // Uganda
    UM, // United States Minor Outlying Islands
    US, // United States of America
    UY, // Uruguay
    UZ, // Uzbekistan
    VA, // Holy See
    VC, // Saint Vincent
    VE, // Venezuela
    VG, // British Virgin Islands
    VI, // U.S. Virgin Islands
    VN, // Viet Nam
    VU, // Vanuatu
    WF, // Wallis
    WS, // Samoa
    YE, // Yemen
    YT, // Mayotte
    ZA, // South Africa
    ZM, // Zambia
    ZW, // Zimbabwe
}

impl TryFrom<&str> for Market {
    type Error = crate::Error;

    /// Converts &str to market code
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match &value.to_lowercase()[..] {
            "ad" => Ok(Market::AD),
            "ae" => Ok(Market::AE),
            "af" => Ok(Market::AF),
            "ag" => Ok(Market::AG),
            "ai" => Ok(Market::AI),
            "al" => Ok(Market::AL),
            "am" => Ok(Market::AM),
            "ao" => Ok(Market::AO),
            "aq" => Ok(Market::AQ),
            "ar" => Ok(Market::AR),
            "as" => Ok(Market::AS),
            "at" => Ok(Market::AT),
            "au" => Ok(Market::AU),
            "aw" => Ok(Market::AW),
            "ax" => Ok(Market::AX),
            "az" => Ok(Market::AZ),
            "ba" => Ok(Market::BA),
            "bb" => Ok(Market::BB),
            "bd" => Ok(Market::BD),
            "be" => Ok(Market::BE),
            "bf" => Ok(Market::BF),
            "bg" => Ok(Market::BG),
            "bh" => Ok(Market::BH),
            "bi" => Ok(Market::BI),
            "bj" => Ok(Market::BJ),
            "bl" => Ok(Market::BL),
            "bm" => Ok(Market::BM),
            "bn" => Ok(Market::BN),
            "bo" => Ok(Market::BO),
            "bq" => Ok(Market::BQ),
            "br" => Ok(Market::BR),
            "bs" => Ok(Market::BS),
            "bt" => Ok(Market::BT),
            "bv" => Ok(Market::BV),
            "bw" => Ok(Market::BW),
            "by" => Ok(Market::BY),
            "bz" => Ok(Market::BZ),
            "ca" => Ok(Market::CA),
            "cc" => Ok(Market::CC),
            "cd" => Ok(Market::CD),
            "cf" => Ok(Market::CF),
            "cg" => Ok(Market::CG),
            "ch" => Ok(Market::CH),
            "ci" => Ok(Market::CI),
            "ck" => Ok(Market::CK),
            "cl" => Ok(Market::CL),
            "cm" => Ok(Market::CM),
            "cn" => Ok(Market::CN),
            "co" => Ok(Market::CO),
            "cr" => Ok(Market::CR),
            "cu" => Ok(Market::CU),
            "cv" => Ok(Market::CV),
            "cw" => Ok(Market::CW),
            "cx" => Ok(Market::CX),
            "cy" => Ok(Market::CY),
            "cz" => Ok(Market::CZ),
            "de" => Ok(Market::DE),
            "dj" => Ok(Market::DJ),
            "dk" => Ok(Market::DK),
            "dm" => Ok(Market::DM),
            "do" => Ok(Market::DO),
            "dz" => Ok(Market::DZ),
            "ec" => Ok(Market::EC),
            "ee" => Ok(Market::EE),
            "eg" => Ok(Market::EG),
            "eh" => Ok(Market::EH),
            "er" => Ok(Market::ER),
            "es" => Ok(Market::ES),
            "et" => Ok(Market::ET),
            "fi" => Ok(Market::FI),
            "fj" => Ok(Market::FJ),
            "fk" => Ok(Market::FK),
            "fm" => Ok(Market::FM),
            "fo" => Ok(Market::FO),
            "fr" => Ok(Market::FR),
            "ga" => Ok(Market::GA),
            "gb" => Ok(Market::GB),
            "gd" => Ok(Market::GD),
            "ge" => Ok(Market::GE),
            "gf" => Ok(Market::GF),
            "gg" => Ok(Market::GG),
            "gh" => Ok(Market::GH),
            "gi" => Ok(Market::GI),
            "gl" => Ok(Market::GL),
            "gm" => Ok(Market::GM),
            "gn" => Ok(Market::GN),
            "gp" => Ok(Market::GP),
            "gq" => Ok(Market::GQ),
            "gr" => Ok(Market::GR),
            "gs" => Ok(Market::GS),
            "gt" => Ok(Market::GT),
            "gu" => Ok(Market::GU),
            "gw" => Ok(Market::GW),
            "gy" => Ok(Market::GY),
            "hk" => Ok(Market::HK),
            "hm" => Ok(Market::HM),
            "hn" => Ok(Market::HN),
            "hr" => Ok(Market::HR),
            "ht" => Ok(Market::HT),
            "hu" => Ok(Market::HU),
            "id" => Ok(Market::ID),
            "ie" => Ok(Market::IE),
            "il" => Ok(Market::IL),
            "im" => Ok(Market::IM),
            "in" => Ok(Market::IN),
            "io" => Ok(Market::IO),
            "iq" => Ok(Market::IQ),
            "ir" => Ok(Market::IR),
            "is" => Ok(Market::IS),
            "it" => Ok(Market::IT),
            "je" => Ok(Market::JE),
            "jm" => Ok(Market::JM),
            "jo" => Ok(Market::JO),
            "jp" => Ok(Market::JP),
            "ke" => Ok(Market::KE),
            "kg" => Ok(Market::KG),
            "kh" => Ok(Market::KH),
            "ki" => Ok(Market::KI),
            "km" => Ok(Market::KM),
            "kn" => Ok(Market::KN),
            "kp" => Ok(Market::KP),
            "kr" => Ok(Market::KR),
            "kw" => Ok(Market::KW),
            "ky" => Ok(Market::KY),
            "kz" => Ok(Market::KZ),
            "la" => Ok(Market::LA),
            "lb" => Ok(Market::LB),
            "lc" => Ok(Market::LC),
            "li" => Ok(Market::LI),
            "lk" => Ok(Market::LK),
            "lr" => Ok(Market::LR),
            "ls" => Ok(Market::LS),
            "lt" => Ok(Market::LT),
            "lu" => Ok(Market::LU),
            "lv" => Ok(Market::LV),
            "ly" => Ok(Market::LY),
            "ma" => Ok(Market::MA),
            "mc" => Ok(Market::MC),
            "md" => Ok(Market::MD),
            "me" => Ok(Market::ME),
            "mf" => Ok(Market::MF),
            "mg" => Ok(Market::MG),
            "mh" => Ok(Market::MH),
            "mk" => Ok(Market::MK),
            "ml" => Ok(Market::ML),
            "mm" => Ok(Market::MM),
            "mn" => Ok(Market::MN),
            "mo" => Ok(Market::MO),
            "mp" => Ok(Market::MP),
            "mq" => Ok(Market::MQ),
            "mr" => Ok(Market::MR),
            "ms" => Ok(Market::MS),
            "mt" => Ok(Market::MT),
            "mu" => Ok(Market::MU),
            "mv" => Ok(Market::MV),
            "mw" => Ok(Market::MW),
            "mx" => Ok(Market::MX),
            "my" => Ok(Market::MY),
            "mz" => Ok(Market::MZ),
            "na" => Ok(Market::NA),
            "nc" => Ok(Market::NC),
            "ne" => Ok(Market::NE),
            "nf" => Ok(Market::NF),
            "ng" => Ok(Market::NG),
            "ni" => Ok(Market::NI),
            "nl" => Ok(Market::NL),
            "no" => Ok(Market::NO),
            "np" => Ok(Market::NP),
            "nr" => Ok(Market::NR),
            "nu" => Ok(Market::NU),
            "nz" => Ok(Market::NZ),
            "om" => Ok(Market::OM),
            "pa" => Ok(Market::PA),
            "pe" => Ok(Market::PE),
            "pf" => Ok(Market::PF),
            "pg" => Ok(Market::PG),
            "ph" => Ok(Market::PH),
            "pk" => Ok(Market::PK),
            "pl" => Ok(Market::PL),
            "pm" => Ok(Market::PM),
            "pn" => Ok(Market::PN),
            "pr" => Ok(Market::PR),
            "ps" => Ok(Market::PS),
            "pt" => Ok(Market::PT),
            "pw" => Ok(Market::PW),
            "py" => Ok(Market::PY),
            "qa" => Ok(Market::QA),
            "re" => Ok(Market::RE),
            "ro" => Ok(Market::RO),
            "rs" => Ok(Market::RS),
            "ru" => Ok(Market::RU),
            "rw" => Ok(Market::RW),
            "sa" => Ok(Market::SA),
            "sb" => Ok(Market::SB),
            "sc" => Ok(Market::SC),
            "sd" => Ok(Market::SD),
            "se" => Ok(Market::SE),
            "sg" => Ok(Market::SG),
            "sh" => Ok(Market::SH),
            "si" => Ok(Market::SI),
            "sj" => Ok(Market::SJ),
            "sk" => Ok(Market::SK),
            "sl" => Ok(Market::SL),
            "sm" => Ok(Market::SM),
            "sn" => Ok(Market::SN),
            "so" => Ok(Market::SO),
            "sr" => Ok(Market::SR),
            "ss" => Ok(Market::SS),
            "st" => Ok(Market::ST),
            "sv" => Ok(Market::SV),
            "sx" => Ok(Market::SX),
            "sy" => Ok(Market::SY),
            "sz" => Ok(Market::SZ),
            "tc" => Ok(Market::TC),
            "td" => Ok(Market::TD),
            "tf" => Ok(Market::TF),
            "tg" => Ok(Market::TG),
            "th" => Ok(Market::TH),
            "tj" => Ok(Market::TJ),
            "tk" => Ok(Market::TK),
            "tl" => Ok(Market::TL),
            "tm" => Ok(Market::TM),
            "tn" => Ok(Market::TN),
            "to" => Ok(Market::TO),
            "tr" => Ok(Market::TR),
            "tt" => Ok(Market::TT),
            "tv" => Ok(Market::TV),
            "tw" => Ok(Market::TW),
            "tz" => Ok(Market::TZ),
            "ua" => Ok(Market::UA),
            "ug" => Ok(Market::UG),
            "um" => Ok(Market::UM),
            "us" => Ok(Market::US),
            "uy" => Ok(Market::UY),
            "uz" => Ok(Market::UZ),
            "va" => Ok(Market::VA),
            "vc" => Ok(Market::VC),
            "ve" => Ok(Market::VE),
            "vg" => Ok(Market::VG),
            "vi" => Ok(Market::VI),
            "vn" => Ok(Market::VN),
            "vu" => Ok(Market::VU),
            "wf" => Ok(Market::WF),
            "ws" => Ok(Market::WS),
            "ye" => Ok(Market::YE),
            "yt" => Ok(Market::YT),
            "za" => Ok(Market::ZA),
            "zm" => Ok(Market::ZM),
            "zw" => Ok(Market::ZW),
            market => Err(Error::InvalidMarket(market.to_owned())),
        }
    }
}

impl TryFrom<String> for Market {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // call &str logic
        (&value[..]).try_into()
    }
}
