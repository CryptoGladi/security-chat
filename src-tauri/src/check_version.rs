//! Module for checking version

use const_format::formatcp;
use log::debug;
use version_compare::Cmp;

const GITHUB_URL: &str = formatcp!("{}/tags", env!("CARGO_PKG_REPOSITORY"));
const VERSION_APP: &str = env!("CARGO_PKG_VERSION");

/// Raw function check version
///
/// Use only [`smart`]
///
/// # Panics
///
/// If you are not connected to the Internet, there will be panic
#[deprecated(note = "use check_version::smart()")]
pub async fn raw(github_url: &str, app_version: &str) -> bool {
    debug!(
        "run check_version with url: {}; app_version: {}",
        github_url, app_version
    );

    let body = reqwest::get(github_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();

    let last_version: String = dom
        .query_selector("h2")
        .unwrap()
        .map(|x| x.get(dom.parser()).unwrap())
        .filter(|x| x.as_tag().unwrap().attributes().class().unwrap() == "f4 d-inline")
        .map(|x| x.as_tag().unwrap().inner_text(dom.parser()))
        .next()
        .unwrap()
        .to_string();

    debug!("last version: {}", last_version);

    version_compare::compare(last_version, app_version).unwrap() == Cmp::Eq
}

/// Check app version
///
/// # Panics
///
/// If you are not connected to the Internet, there will be panic
pub async fn smart() -> bool {
    #[allow(deprecated)]
    raw(GITHUB_URL, VERSION_APP).await
}

#[cfg(test)]
mod tests {
    use test_log::test;

    #[allow(deprecated)]
    #[test(tokio::test)]
    async fn raw_check_version() {
        assert!(super::raw(super::GITHUB_URL, super::VERSION_APP).await);
    }

    #[test(tokio::test)]
    async fn smart_check_version() {
        assert!(super::smart().await);
    }

    #[test(tokio::test)]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Builder, source: RelativeUrlWithoutBase }"
    )]
    #[allow(deprecated)]
    async fn raw_check_version_with_break_url() {
        assert!(super::raw("INCORRECT URL!", super::VERSION_APP).await);
    }
}
