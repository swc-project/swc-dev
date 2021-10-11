use swc_node_arch::{NodeArch, NodePlatform, PlatformDetail};

fn parse(s: &str) -> PlatformDetail {
    let ss = s.split('-').collect::<Vec<_>>();

    let platform = NodePlatform::from_sys(&ss[0]).unwrap();
    let arch = NodeArch::from_cpu(&ss[1]).unwrap();

    match ss.len() {
        2 => PlatformDetail {
            platform,
            platform_arch_abi: s.to_string(),
            arch,
            raw: s.into(),
            abi: None,
        },

        3 => PlatformDetail {
            platform,
            platform_arch_abi: s.to_string(),
            arch,
            raw: s.into(),
            abi: Some(ss[2].to_string()),
        },
        _ => unreachable!(),
    }
}

pub fn all_node_platforms() -> Vec<PlatformDetail> {
    vec![
        "win32-x64-msvc",
        "darwin-x64",
        "linux-x64-gnu",
        "linux-x64-musl",
        "win32-ia32-msvc",
        "linux-arm64-gnu",
        "linux-arm-gnueabihf",
        "darwin-arm64",
        "android-arm64",
        "freebsd-x64",
        "linux-arm64-musl",
        "win32-arm64-msvc",
    ]
    .into_iter()
    .map(parse)
    .collect()
}
