//! The portfolio registry -- every public cybercoretech organization repo that's actually
//! part of the Cybercore family (queried live via `gh api
//! users/cybercoretech/repos`, not guessed from memory). Deliberately
//! excludes: the profile README and hub-site repos (not binary tools with
//! releases), and ferraxis/vexlang (real projects, but a separate
//! compiler/language-design line of work, not Cybercore). A private repo
//! here would just make every command report "unknown," so keep this in
//! sync with reality rather than adding speculatively.

const REPOS: &[(&str, &str)] = &[
    ("echo", "cybercoretech/echo"),
    ("cybermeta", "cybercoretech/cybermeta"),
    ("cybervault", "cybercoretech/cybervault"),
    ("keysmith", "cybercoretech/keysmith"),
    ("cyberterm", "cybercoretech/cyberterm"),
    ("sentrygrid", "cybercoretech/sentrygrid"),
    ("wraithflow", "cybercoretech/wraithflow"),
    ("vortexwall", "cybercoretech/vortexwall"),
    ("aetherscope", "cybercoretech/aetherscope"),
    ("ghostport", "cybercoretech/ghostport"),
    ("apexdaemon", "cybercoretech/apexdaemon"),
    ("cybercore", "cybercoretech/cybercore"),
    ("cyberdeck", "cybercoretech/cyberdeck"),
    ("cyberplug", "cybercoretech/cyberplug"),
    ("cyberplug-bar-widget", "cybercoretech/cyberplug-bar-widget"),
    ("diagprint", "cybercoretech/diagprint"),
    ("gateflow", "cybercoretech/gateflow"),
    ("agentforge", "cybercoretech/agentforge"),
];

const FEATURED_TOOLS: &[(&str, &str, &str)] = &[
    (
        "cybercore",
        "The shared design system every tool above is built on.",
        "https://cybercoretech.github.io/cybercore/",
    ),
    (
        "cyberdeck",
        "Systems intelligence framework.",
        "https://cybercoretech.github.io/cyberdeck/",
    ),
    (
        "diagprint",
        "Rust diagnostics lifecycle framework.",
        "https://cybercoretech.github.io/diagprint/",
    ),
    (
        "gateflow",
        "Kernel-sandbox testing with netns, chaos, and veth.",
        "https://cybercoretech.github.io/gateflow/",
    ),
    (
        "cybermeta",
        "TUI EXIF metadata tool.",
        "https://cybercoretech.github.io/cybermeta/",
    ),
];

pub fn resolve(name: &str) -> Option<&'static str> {
    let name = name.trim().to_lowercase();
    REPOS.iter().find(|(n, _)| *n == name).map(|(_, r)| *r)
}

pub fn all() -> impl Iterator<Item = &'static str> {
    REPOS.iter().map(|(_, r)| *r)
}

pub fn all_named() -> impl Iterator<Item = (&'static str, &'static str)> {
    REPOS.iter().copied()
}

pub fn featured() -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
    FEATURED_TOOLS.iter().copied()
}

pub fn names_list() -> String {
    REPOS.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(", ")
}

pub fn description(name: &str) -> &'static str {
    match name {
        "agentforge" => "Deterministic AI-assisted workflow orchestration.",
        "apexdaemon" => "Background automation for theme sync, fleet health, and backups.",
        "aetherscope" => "Packet capture and BPF-filtered network inspection.",
        "cybercore" => "The shared Cybercore design system and foundations.",
        "cyberdeck" => "Systems intelligence framework.",
        "cybermeta" => "Terminal EXIF metadata tooling.",
        "cyberplug" => "Plugin manager for Omarchy.",
        "cyberplug-bar-widget" => "Companion bar widget for Cyberplug.",
        "cyberterm" => "GPU-rendered terminal emulator.",
        "cybervault" => "Encrypted secrets vault using modern authenticated cryptography.",
        "diagprint" => "Rust diagnostics lifecycle framework.",
        "echo" => "Traffic inspector TUI for WraithFlow captures.",
        "gateflow" => "Kernel-sandbox testing with namespaces, chaos, and veth.",
        "ghostport" => "Encrypted NAT-traversing port forwarder.",
        "keysmith" => "Password, passphrase, and hash generation companion.",
        "sentrygrid" => "Network exposure auditor with host and container correlation.",
        "vortexwall" => "Active-blackholing firewall using nftables.",
        "wraithflow" => "Config-driven TCP proxy and traffic-analysis pipelines.",
        _ => "Cybercore project.",
    }
}

pub fn site(name: &str) -> Option<&'static str> {
    FEATURED_TOOLS
        .iter()
        .find(|(project, _, _)| *project == name)
        .map(|(_, _, url)| *url)
}
