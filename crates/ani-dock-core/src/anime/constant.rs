use std::sync::LazyLock;

use regex::Regex;

pub static MIN_EPISODE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\d*\.?\d* *\.?[A-Z,a-z]*(?:電影)?\]").expect("could not parse min episode regex")
});
pub static FULL_EPISODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[.+?\]").expect("could not parse full episode regex"));
pub static WHITESPACES_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+").expect("could not parse whitespaces regex"));

pub static SEASON_FILTER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"第[零一二三四五六七八九十]{1,3}季").expect("could not parse season filter regex")
});
pub static EXTRA_FILTER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(特別篇|中文配音)\]").expect("could not parse extra filter regex")
});
