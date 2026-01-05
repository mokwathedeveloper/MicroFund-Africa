pub enum Language {
    English,
    Swahili,
}

pub struct Translation {
    pub key: &'static str,
    pub en: &'static str,
    pub sw: &'static str,
}

const TRANSLATIONS: &[Translation] = &[
    Translation { key: "welcome", en: "Financial Freedom for Everyone", sw: "Uhuru wa Kifedha kwa Kila Mtu" },
    Translation { key: "hero_sub", en: "MicroFund Africa provides secure, blockchain-powered microloans and savings for the unbanked across the continent.", sw: "MicroFund Africa hutoa mikopo midogo na akiba salama iliyowezeshwa na blockchain kwa wale wasio na akaunti za benki kote barani." },
    Translation { key: "get_started", en: "Get Started", sw: "Anza Sasa" },
    Translation { key: "login", en: "Login", sw: "Ingia" },
    Translation { key: "register", en: "Register", sw: "Jisajili" },
    Translation { key: "dashboard", en: "Dashboard", sw: "Dashibodi" },
    Translation { key: "logout", en: "Logout", sw: "Ondoka" },
    Translation { key: "trust_score", en: "Trust Score", sw: "Alama ya Imani" },
    Translation { key: "total_impact", en: "Total Impact", sw: "Jumla ya Athari" },
    Translation { key: "marketplace", en: "P2P Marketplace", sw: "Soko la P2P" },
    Translation { key: "microloans", en: "Microloans", sw: "Mikopo Midogo" },
    Translation { key: "savings_goals", en: "Savings Goals", sw: "Malengo ya Akiba" },
    Translation { key: "request_loan", en: "Request Loan", sw: "Omba Mkopo" },
    Translation { key: "create_goal", en: "Create Goal", sw: "Tengeneza Lengo" },
    Translation { key: "repay", en: "Repay", sw: "Lipa" },
    Translation { key: "fund", en: "Fund", sw: "Gharamia" },
];

pub fn t(key: &str, lang: &Language) -> String {
    TRANSLATIONS.iter()
        .find(|t| t.key == key)
        .map(|t| match lang {
            Language::English => t.en.to_string(),
            Language::Swahili => t.sw.to_string(),
        })
        .unwrap_or_else(|| key.to_string())
}
