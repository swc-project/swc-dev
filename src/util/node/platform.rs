use swc_node_arch::PlatformDetail;

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
    .map(|s| s.parse().unwrap())
    .collect()
}
