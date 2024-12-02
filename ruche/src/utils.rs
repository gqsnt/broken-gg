use rkyv::{Archive, Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize, Archive)]
pub struct FixedSizeString<const N: usize>([u8; N]);

impl<const N: usize> FixedSizeString<N> {
    pub fn new(value: &str) -> Self {
        let mut result = [0u8; N];
        let bytes = value.as_bytes();
        let len = bytes.len().min(N);
        result[..len].copy_from_slice(&bytes[..len]);
        FixedSizeString(result)
    }

    fn trim_end_zeros(&self) -> &[u8] {
        let end = self
            .0
            .iter()
            .rposition(|&b| b != 0)
            .map_or(0, |pos| pos + 1);
        &self.0[..end]
    }
}

impl<const N: usize> AsRef<str> for FixedSizeString<N> {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(self.trim_end_zeros()).unwrap()
    }
}

impl<const N: usize> std::fmt::Display for FixedSizeString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.trim_end_zeros()))
    }
}

pub type Puuid = FixedSizeString<78>;
pub type ProPlayerSlug = FixedSizeString<20>;

pub type RiotMatchId = FixedSizeString<17>;
pub type DurationSince = FixedSizeString<14>;

pub fn format_duration(seconds: Option<i32>) -> String {
    let seconds = seconds.unwrap_or(0);
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn calculate_loss_and_win_rate<T: Into<f32>>(wins: T, total: T) -> (f32, f32) {
    let wins = wins.into();
    let total = total.into();

    if total == 0.0 {
        return (0.0, 0.0);
    }
    let loses = total - wins;
    let win_rate = (wins / total) * 100.0;
    (loses, win_rate)
}

pub fn calculate_and_format_kda<T: Into<f32>>(kills: T, deaths: T, assists: T) -> String {
    let kda = calculate_kda(kills, deaths, assists);
    format!("{:.2}", kda)
}

pub fn calculate_kda<T: Into<f32>>(kills: T, deaths: T, assists: T) -> f32 {
    let kills = kills.into();
    let deaths = deaths.into();
    let assists = assists.into();

    if deaths == 0.0 {
        if kills == 0.0 {
            return assists;
        }
        return kills + assists;
    }
    (kills + assists) / deaths
}

pub fn version_to_major_minor(version: &str) -> String {
    let mut split = version.split(".");
    if split.clone().count() < 2 {
        panic!("version_to_major_minor: version: {}", version);
    }
    let major = split.next().unwrap();
    let minor = split.next().unwrap();
    format!("{}.{}", major, minor)
}

pub fn format_with_spaces(number: u32) -> String {
    // convert 4978521 -> 4 978 521

    let mut num_str = number.to_string();
    let mut result = String::new();

    while num_str.len() > 3 {
        let split_at = num_str.len() - 3;
        result.insert_str(0, &format!(" {}", &num_str[split_at..]));
        num_str.truncate(split_at);
    }
    result.insert_str(0, &num_str);

    result
}

pub fn summoner_to_slug(game_name: &str, tag_line: &str) -> String {
    format!(
        "{}-{}",
        urlencoding::encode(game_name),
        urlencoding::encode(tag_line)
    )
}

pub fn parse_summoner_slug(slug: &str) -> (String, String) {
    let parts: Vec<&str> = slug.split('-').collect();
    let len = parts.len();
    let game_name = urlencoding::decode(parts[0]).ok().unwrap().into_owned();
    if len == 2 {
        return (
            game_name,
            urlencoding::decode(parts[1]).ok().unwrap().into_owned(),
        );
    }
    (game_name, String::new())
}

pub fn summoner_url(platform: &str, game_name: &str, tag_line: &str) -> String {
    format!(
        "/platform/{}/summoners/{}",
        platform,
        summoner_to_slug(game_name, tag_line)
    )
}

pub fn summoner_not_found_url(platform: &str, game_name: &str, tag_line: &str) -> String {
    format!(
        "/platform/{}?game_name={}&tag_line={}",
        platform, game_name, tag_line
    )
}

pub fn summoner_encounter_url(
    platform: &str,
    game_name: &str,
    tag_line: &str,
    encounter_platform: &str,
    encounter_game_name: &str,
    encounter_tag_line: &str,
) -> String {
    format!(
        "/platform/{}/summoners/{}?tab=encounter&encounter_slug={}&encounter_platform={}",
        platform,
        summoner_to_slug(game_name, tag_line),
        summoner_to_slug(encounter_game_name, encounter_tag_line),
        encounter_platform
    )
}

pub fn round_to_2_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn format_float_to_2digits(value: f32) -> String {
    let value = (value * 100.0).round() / 100.0;
    value.to_string()
}