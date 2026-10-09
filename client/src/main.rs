//! `nfrp-client` 命令行入口。
//!
//! 运行时逻辑全部在 `nfrp_client` 库中（`cli_main` / `run` / `run_with_shutdown`），
//! 这里只保留一个薄壳，便于嵌入式宿主直接依赖库而不经过命令行。

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    nfrp_client::cli_main().await
}
