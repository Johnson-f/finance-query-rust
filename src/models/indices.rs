use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    #[serde(rename = "US")]
    UnitedStates,
    #[serde(rename = "NA")]
    NorthAmerica,
    #[serde(rename = "SA")]
    SouthAmerica,
    #[serde(rename = "EU")]
    Europe,
    #[serde(rename = "AS")]
    Asia,
    #[serde(rename = "AF")]
    Africa,
    #[serde(rename = "ME")]
    MiddleEast,
    #[serde(rename = "OCE")]
    Oceania,
    #[serde(rename = "global")]
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Index {
    // United States
    #[serde(rename = "snp")]
    Gspc, // S&P 500
    #[serde(rename = "djia")]
    Dji, // Dow Jones Industrial Average
    #[serde(rename = "nasdaq")]
    Ixic, // NASDAQ Composite
    #[serde(rename = "nyse-composite")]
    Nya, // NYSE Composite
    #[serde(rename = "nyse-amex")]
    Xax, // NYSE American Composite
    #[serde(rename = "rut")]
    Rut, // Russell 2000
    #[serde(rename = "vix")]
    Vix, // CBOE Volatility Index

    // North America (excluding US)
    #[serde(rename = "tsx-composite")]
    Gsptse, // Toronto Stock Exchange

    // South America
    #[serde(rename = "ibovespa")]
    Bvsp, // Brazil Bovespa
    #[serde(rename = "ipc-mexico")]
    Mxx, // Mexican IPC
    #[serde(rename = "ipsa")]
    Ipsa, // Chile IPSA
    #[serde(rename = "merval")]
    Merv, // Argentina Merval
    #[serde(rename = "ivbx")]
    Ivbx, // Brazil IVBX
    #[serde(rename = "ibrx-50")]
    Ibrx50, // Brazil IBrX-50

    // Europe
    #[serde(rename = "ftse-100")]
    Ftse, // FTSE 100
    #[serde(rename = "dax")]
    Gdaxi, // German DAX
    #[serde(rename = "cac-40")]
    Fchi, // French CAC 40
    #[serde(rename = "euro-stoxx-50")]
    Stoxx50e, // Euro Stoxx 50
    #[serde(rename = "euronext-100")]
    N100, // Euronext 100
    #[serde(rename = "bel-20")]
    Bfx, // Belgian BEL 20
    #[serde(rename = "moex")]
    MoexMe, // Moscow Exchange
    #[serde(rename = "aex")]
    Aex, // Amsterdam Exchange
    #[serde(rename = "ibex-35")]
    Ibex, // Spanish IBEX 35
    #[serde(rename = "ftse-mib")]
    Ftsemib, // Italian FTSE MIB
    #[serde(rename = "smi")]
    Ssmi, // Swiss Market Index
    #[serde(rename = "psi")]
    Psi, // Portuguese PSI
    #[serde(rename = "atx")]
    Atx, // Austrian ATX
    #[serde(rename = "omxs30")]
    Omxs30, // Stockholm OMX 30
    #[serde(rename = "omxc25")]
    Omxc25, // Copenhagen OMX 25
    #[serde(rename = "wig20")]
    Wig20, // Warsaw WIG 20
    #[serde(rename = "budapest-se")]
    Bux, // Budapest Stock Exchange
    #[serde(rename = "moex-russia")]
    Imoex, // Moscow Exchange Russia
    #[serde(rename = "rtsi")]
    Rtsi, // Russian Trading System

    // Asia
    #[serde(rename = "hang-seng")]
    Hsi, // Hong Kong Hang Seng
    #[serde(rename = "sti")]
    Sti, // Singapore Straits Times
    #[serde(rename = "sensex")]
    Bsesn, // BSE Sensex (India)
    #[serde(rename = "idx-composite")]
    Jkse, // Jakarta Composite
    #[serde(rename = "ftse-bursa")]
    Klse, // FTSE Bursa Malaysia
    #[serde(rename = "kospi")]
    Ks11, // Korea KOSPI
    #[serde(rename = "twse")]
    Twii, // Taiwan TAIEX
    #[serde(rename = "nikkei-225")]
    N225, // Nikkei 225
    #[serde(rename = "shanghai")]
    Shanghai, // Shanghai Composite
    #[serde(rename = "szse-component")]
    Szse, // Shenzhen Component
    #[serde(rename = "set")]
    Set, // Thailand SET
    #[serde(rename = "nifty-50")]
    Nsei, // NSE Nifty 50 (India)
    #[serde(rename = "nifty-200")]
    Cnx200, // NSE Nifty 200
    #[serde(rename = "psei-composite")]
    Psei, // Philippines PSEi Composite
    #[serde(rename = "china-a50")]
    ChinaA50, // FTSE China A50
    #[serde(rename = "dj-shanghai")]
    Djsh, // Dow Jones Shanghai
    #[serde(rename = "india-vix")]
    Indiavix, // India VIX

    // Africa
    #[serde(rename = "egx-30")]
    Case30, // Egypt EGX 30
    #[serde(rename = "jse-40")]
    Jn0uJo, // FTSE JSE Top 40- USD Net TRI
    #[serde(rename = "ftse-jse")]
    Ftsejse, // FTSE/JSE SA Financials Index
    #[serde(rename = "afr-40")]
    Afr40, // All Africa 40 Rand Index
    #[serde(rename = "raf-40")]
    Raf40, // RAFI 40 Index
    #[serde(rename = "sa-40")]
    Sa40, // South Africa Top 40
    #[serde(rename = "alt-15")]
    Alt15, // Alternative 15

    // Middle East
    #[serde(rename = "ta-125")]
    Ta125Ta, // Tel Aviv 125
    #[serde(rename = "ta-35")]
    Ta35, // Tel Aviv 35
    #[serde(rename = "tadawul-all-share")]
    Tasi, // Tadawul All Share
    #[serde(rename = "tamayuz")]
    Tamayuz, // Egyptian Tamayuz
    #[serde(rename = "bist-100")]
    Bist100, // Borsa Istanbul 100

    // Oceania
    #[serde(rename = "asx-200")]
    Axjo, // ASX 200 (Australia)
    #[serde(rename = "all-ordinaries")]
    Aord, // All Ordinaries (Australia)
    #[serde(rename = "nzx-50")]
    Nz50, // NZX 50 (New Zealand)

    // Global/Currency
    #[serde(rename = "usd")]
    DxYNyb, // US Dollar Index
    #[serde(rename = "msci-europe")]
    UsdStrd, // MSCI Europe USD
    #[serde(rename = "gbp")]
    Xdb, // British Pound
    #[serde(rename = "euro")]
    Xde, // Euro
    #[serde(rename = "yen")]
    Xdn, // Japanese Yen
    #[serde(rename = "australian")]
    Xda, // Australian Dollar
    #[serde(rename = "msci-world")]
    MsciWorld, // MSCI World Index
    #[serde(rename = "cboe-uk-100")]
    Buk100p, // CBOE UK 100
}

impl Index {
    pub fn as_str(&self) -> &'static str {
        match self {
            Index::Gspc => "snp",
            Index::Dji => "djia",
            Index::Ixic => "nasdaq",
            Index::Nya => "nyse-composite",
            Index::Xax => "nyse-amex",
            Index::Rut => "rut",
            Index::Vix => "vix",
            Index::Gsptse => "tsx-composite",
            Index::Bvsp => "ibovespa",
            Index::Mxx => "ipc-mexico",
            Index::Ipsa => "ipsa",
            Index::Merv => "merval",
            Index::Ivbx => "ivbx",
            Index::Ibrx50 => "ibrx-50",
            Index::Ftse => "ftse-100",
            Index::Gdaxi => "dax",
            Index::Fchi => "cac-40",
            Index::Stoxx50e => "euro-stoxx-50",
            Index::N100 => "euronext-100",
            Index::Bfx => "bel-20",
            Index::MoexMe => "moex",
            Index::Aex => "aex",
            Index::Ibex => "ibex-35",
            Index::Ftsemib => "ftse-mib",
            Index::Ssmi => "smi",
            Index::Psi => "psi",
            Index::Atx => "atx",
            Index::Omxs30 => "omxs30",
            Index::Omxc25 => "omxc25",
            Index::Wig20 => "wig20",
            Index::Bux => "budapest-se",
            Index::Imoex => "moex-russia",
            Index::Rtsi => "rtsi",
            Index::Hsi => "hang-seng",
            Index::Sti => "sti",
            Index::Bsesn => "sensex",
            Index::Jkse => "idx-composite",
            Index::Klse => "ftse-bursa",
            Index::Ks11 => "kospi",
            Index::Twii => "twse",
            Index::N225 => "nikkei-225",
            Index::Shanghai => "shanghai",
            Index::Szse => "szse-component",
            Index::Set => "set",
            Index::Nsei => "nifty-50",
            Index::Cnx200 => "nifty-200",
            Index::Psei => "psei-composite",
            Index::ChinaA50 => "china-a50",
            Index::Djsh => "dj-shanghai",
            Index::Indiavix => "india-vix",
            Index::Case30 => "egx-30",
            Index::Jn0uJo => "jse-40",
            Index::Ftsejse => "ftse-jse",
            Index::Afr40 => "afr-40",
            Index::Raf40 => "raf-40",
            Index::Sa40 => "sa-40",
            Index::Alt15 => "alt-15",
            Index::Ta125Ta => "ta-125",
            Index::Ta35 => "ta-35",
            Index::Tasi => "tadawul-all-share",
            Index::Tamayuz => "tamayuz",
            Index::Bist100 => "bist-100",
            Index::Axjo => "asx-200",
            Index::Aord => "all-ordinaries",
            Index::Nz50 => "nzx-50",
            Index::DxYNyb => "usd",
            Index::UsdStrd => "msci-europe",
            Index::Xdb => "gbp",
            Index::Xde => "euro",
            Index::Xdn => "yen",
            Index::Xda => "australian",
            Index::MsciWorld => "msci-world",
            Index::Buk100p => "cboe-uk-100",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "snp" => Some(Index::Gspc),
            "djia" => Some(Index::Dji),
            "nasdaq" => Some(Index::Ixic),
            "nyse-composite" => Some(Index::Nya),
            "nyse-amex" => Some(Index::Xax),
            "rut" => Some(Index::Rut),
            "vix" => Some(Index::Vix),
            "tsx-composite" => Some(Index::Gsptse),
            "ibovespa" => Some(Index::Bvsp),
            "ipc-mexico" => Some(Index::Mxx),
            "ipsa" => Some(Index::Ipsa),
            "merval" => Some(Index::Merv),
            "ivbx" => Some(Index::Ivbx),
            "ibrx-50" => Some(Index::Ibrx50),
            "ftse-100" => Some(Index::Ftse),
            "dax" => Some(Index::Gdaxi),
            "cac-40" => Some(Index::Fchi),
            "euro-stoxx-50" => Some(Index::Stoxx50e),
            "euronext-100" => Some(Index::N100),
            "bel-20" => Some(Index::Bfx),
            "moex" => Some(Index::MoexMe),
            "aex" => Some(Index::Aex),
            "ibex-35" => Some(Index::Ibex),
            "ftse-mib" => Some(Index::Ftsemib),
            "smi" => Some(Index::Ssmi),
            "psi" => Some(Index::Psi),
            "atx" => Some(Index::Atx),
            "omxs30" => Some(Index::Omxs30),
            "omxc25" => Some(Index::Omxc25),
            "wig20" => Some(Index::Wig20),
            "budapest-se" => Some(Index::Bux),
            "moex-russia" => Some(Index::Imoex),
            "rtsi" => Some(Index::Rtsi),
            "hang-seng" => Some(Index::Hsi),
            "sti" => Some(Index::Sti),
            "sensex" => Some(Index::Bsesn),
            "idx-composite" => Some(Index::Jkse),
            "ftse-bursa" => Some(Index::Klse),
            "kospi" => Some(Index::Ks11),
            "twse" => Some(Index::Twii),
            "nikkei-225" => Some(Index::N225),
            "shanghai" => Some(Index::Shanghai),
            "szse-component" => Some(Index::Szse),
            "set" => Some(Index::Set),
            "nifty-50" => Some(Index::Nsei),
            "nifty-200" => Some(Index::Cnx200),
            "psei-composite" => Some(Index::Psei),
            "china-a50" => Some(Index::ChinaA50),
            "dj-shanghai" => Some(Index::Djsh),
            "india-vix" => Some(Index::Indiavix),
            "egx-30" => Some(Index::Case30),
            "jse-40" => Some(Index::Jn0uJo),
            "ftse-jse" => Some(Index::Ftsejse),
            "afr-40" => Some(Index::Afr40),
            "raf-40" => Some(Index::Raf40),
            "sa-40" => Some(Index::Sa40),
            "alt-15" => Some(Index::Alt15),
            "ta-125" => Some(Index::Ta125Ta),
            "ta-35" => Some(Index::Ta35),
            "tadawul-all-share" => Some(Index::Tasi),
            "tamayuz" => Some(Index::Tamayuz),
            "bist-100" => Some(Index::Bist100),
            "asx-200" => Some(Index::Axjo),
            "all-ordinaries" => Some(Index::Aord),
            "nzx-50" => Some(Index::Nz50),
            "usd" => Some(Index::DxYNyb),
            "msci-europe" => Some(Index::UsdStrd),
            "gbp" => Some(Index::Xdb),
            "euro" => Some(Index::Xde),
            "yen" => Some(Index::Xdn),
            "australian" => Some(Index::Xda),
            "msci-world" => Some(Index::MsciWorld),
            "cboe-uk-100" => Some(Index::Buk100p),
            _ => None,
        }
    }

    pub fn all() -> Vec<Index> {
        vec![
            Index::Gspc, Index::Dji, Index::Ixic, Index::Nya, Index::Xax, Index::Rut, Index::Vix,
            Index::Gsptse,
            Index::Bvsp, Index::Mxx, Index::Ipsa, Index::Merv, Index::Ivbx, Index::Ibrx50,
            Index::Ftse, Index::Gdaxi, Index::Fchi, Index::Stoxx50e, Index::N100, Index::Bfx,
            Index::MoexMe, Index::Aex, Index::Ibex, Index::Ftsemib, Index::Ssmi, Index::Psi,
            Index::Atx, Index::Omxs30, Index::Omxc25, Index::Wig20, Index::Bux, Index::Imoex,
            Index::Rtsi,
            Index::Hsi, Index::Sti, Index::Bsesn, Index::Jkse, Index::Klse, Index::Ks11,
            Index::Twii, Index::N225, Index::Shanghai, Index::Szse, Index::Set, Index::Nsei,
            Index::Cnx200, Index::Psei, Index::ChinaA50, Index::Djsh, Index::Indiavix,
            Index::Case30, Index::Jn0uJo, Index::Ftsejse, Index::Afr40, Index::Raf40, Index::Sa40,
            Index::Alt15,
            Index::Ta125Ta, Index::Ta35, Index::Tasi, Index::Tamayuz, Index::Bist100,
            Index::Axjo, Index::Aord, Index::Nz50,
            Index::DxYNyb, Index::UsdStrd, Index::Xdb, Index::Xde, Index::Xdn, Index::Xda,
            Index::MsciWorld, Index::Buk100p,
        ]
    }
}

pub fn get_index_regions() -> HashMap<Index, Region> {
    let mut map = HashMap::new();
    // United States
    map.insert(Index::Gspc, Region::UnitedStates);
    map.insert(Index::Dji, Region::UnitedStates);
    map.insert(Index::Ixic, Region::UnitedStates);
    map.insert(Index::Nya, Region::UnitedStates);
    map.insert(Index::Xax, Region::UnitedStates);
    map.insert(Index::Rut, Region::UnitedStates);
    map.insert(Index::Vix, Region::UnitedStates);
    // North America (excluding US)
    map.insert(Index::Gsptse, Region::NorthAmerica);
    // South America
    map.insert(Index::Bvsp, Region::SouthAmerica);
    map.insert(Index::Mxx, Region::SouthAmerica);
    map.insert(Index::Ipsa, Region::SouthAmerica);
    map.insert(Index::Merv, Region::SouthAmerica);
    map.insert(Index::Ivbx, Region::SouthAmerica);
    map.insert(Index::Ibrx50, Region::SouthAmerica);
    // Europe
    map.insert(Index::Ftse, Region::Europe);
    map.insert(Index::Gdaxi, Region::Europe);
    map.insert(Index::Fchi, Region::Europe);
    map.insert(Index::Stoxx50e, Region::Europe);
    map.insert(Index::N100, Region::Europe);
    map.insert(Index::Bfx, Region::Europe);
    map.insert(Index::MoexMe, Region::Europe);
    map.insert(Index::Aex, Region::Europe);
    map.insert(Index::Ibex, Region::Europe);
    map.insert(Index::Ftsemib, Region::Europe);
    map.insert(Index::Ssmi, Region::Europe);
    map.insert(Index::Psi, Region::Europe);
    map.insert(Index::Atx, Region::Europe);
    map.insert(Index::Omxs30, Region::Europe);
    map.insert(Index::Omxc25, Region::Europe);
    map.insert(Index::Wig20, Region::Europe);
    map.insert(Index::Bux, Region::Europe);
    map.insert(Index::Imoex, Region::Europe);
    map.insert(Index::Rtsi, Region::Europe);
    // Asia
    map.insert(Index::Hsi, Region::Asia);
    map.insert(Index::Sti, Region::Asia);
    map.insert(Index::Bsesn, Region::Asia);
    map.insert(Index::Jkse, Region::Asia);
    map.insert(Index::Klse, Region::Asia);
    map.insert(Index::Ks11, Region::Asia);
    map.insert(Index::Twii, Region::Asia);
    map.insert(Index::N225, Region::Asia);
    map.insert(Index::Shanghai, Region::Asia);
    map.insert(Index::Szse, Region::Asia);
    map.insert(Index::Set, Region::Asia);
    map.insert(Index::Nsei, Region::Asia);
    map.insert(Index::Cnx200, Region::Asia);
    map.insert(Index::Psei, Region::Asia);
    map.insert(Index::ChinaA50, Region::Asia);
    map.insert(Index::Djsh, Region::Asia);
    map.insert(Index::Indiavix, Region::Asia);
    // Africa
    map.insert(Index::Case30, Region::Africa);
    map.insert(Index::Jn0uJo, Region::Africa);
    map.insert(Index::Ftsejse, Region::Africa);
    map.insert(Index::Afr40, Region::Africa);
    map.insert(Index::Sa40, Region::Africa);
    map.insert(Index::Raf40, Region::Africa);
    map.insert(Index::Alt15, Region::Africa);
    // Middle East
    map.insert(Index::Ta125Ta, Region::MiddleEast);
    map.insert(Index::Ta35, Region::MiddleEast);
    map.insert(Index::Tasi, Region::MiddleEast);
    map.insert(Index::Tamayuz, Region::MiddleEast);
    map.insert(Index::Bist100, Region::MiddleEast);
    // Oceania
    map.insert(Index::Axjo, Region::Oceania);
    map.insert(Index::Aord, Region::Oceania);
    map.insert(Index::Nz50, Region::Oceania);
    // Global/Currency
    map.insert(Index::DxYNyb, Region::Global);
    map.insert(Index::UsdStrd, Region::Global);
    map.insert(Index::Xdb, Region::Global);
    map.insert(Index::Xde, Region::Global);
    map.insert(Index::Xdn, Region::Global);
    map.insert(Index::Xda, Region::Global);
    map.insert(Index::MsciWorld, Region::Global);
    map.insert(Index::Buk100p, Region::Global);
    map
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarketIndex {
    pub name: String,
    pub value: f64,
    pub change: String,
    #[serde(rename = "percentChange")]
    pub percent_change: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fiveDaysReturn")]
    pub five_days_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oneMonthReturn")]
    pub one_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "threeMonthReturn")]
    pub three_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sixMonthReturn")]
    pub six_month_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ytdReturn")]
    pub ytd_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yearReturn")]
    pub year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "threeYearReturn")]
    pub three_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fiveYearReturn")]
    pub five_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tenYearReturn")]
    pub ten_year_return: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxReturn")]
    pub max_return: Option<String>,
}