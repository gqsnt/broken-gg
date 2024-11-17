#[cfg(feature = "ssr")]
use crate::consts::platform_route::PlatformRoute;
#[cfg(feature = "ssr")]
use crate::utils::{parse_summoner_slug, summoner_not_found_url};
use crate::views::summoner_page::Summoner;
use leptos::prelude::*;
use leptos::server;

#[server]
pub async fn get_summoner(
    platform_type: String,
    summoner_slug: String,
) -> Result<Summoner, ServerFnError> {
    //log!("Server::Fetching summoner: {}", summoner_slug);
    let state = expect_context::<crate::ssr::AppState>();
    let db = state.db.clone();

    let platform_route = PlatformRoute::from(platform_type.as_str());
    let (game_name, tag_line) = parse_summoner_slug(summoner_slug.as_str());
    match ssr::find_summoner_by_exact_game_name_tag_line(&db, &platform_route, game_name.as_str(), tag_line.as_str()).await {
        Ok(summoner) => {
            Ok(summoner)
        }
        Err(e) => {
            let (game_name, tag_line) = parse_summoner_slug(summoner_slug.as_str());
            leptos_axum::redirect(summoner_not_found_url(platform_route.to_string().as_str(), game_name.as_str(), tag_line.as_str()).as_str());
            e.as_server_fn_error()
        }
    }
}


#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::backend::ssr::AppResult;
    use crate::consts::platform_route::PlatformRoute;
    use crate::views::summoner_page::Summoner;
    use crate::DATE_FORMAT;
    use chrono::NaiveDateTime;

    pub async fn find_summoner_by_exact_game_name_tag_line(
        db: &sqlx::PgPool,
        platform_route: &PlatformRoute,
        game_name: &str,
        tag_line: &str,
    ) -> AppResult<Summoner> {
        sqlx::query_as::<_, SummonerModel>(
            r#"
            SELECT
                   ss.id              as id,
                   ss.game_name       as game_name,
                   ss.tag_line        as tag_line,
                   ss.platform        as platform,
                   ss.profile_icon_id as profile_icon_id,
                   ss.summoner_level  as summoner_level,
                   ss.puuid           as puuid,
                   ss.updated_at      as updated_at,
                   pp.slug            as pro_slug
            FROM summoners as ss
                     left join (select id, slug from pro_players) as pp on pp.id = ss.pro_player_id
            WHERE
                ss.game_name = $1
                AND ss.tag_line = $2
                AND ss.platform = $3
            LIMIT 1"#
        ).bind(game_name)
            .bind(tag_line)
            .bind(platform_route.to_string())
            .fetch_one(db)
            .await
            .map(|summoner_db| {
                Summoner {
                    id: summoner_db.id,
                    game_name: summoner_db.game_name,
                    tag_line: summoner_db.tag_line,
                    puuid: summoner_db.puuid,
                    platform: PlatformRoute::from(summoner_db.platform.as_str()),
                    updated_at: summoner_db.updated_at.format(DATE_FORMAT).to_string(),
                    summoner_level: summoner_db.summoner_level,
                    profile_icon_id: summoner_db.profile_icon_id as u16,
                    pro_slug: summoner_db.pro_slug,
                }
            })
            .map_err(|e| e.into())
    }


    #[derive(sqlx::FromRow, Debug)]
    pub struct SummonerModel {
        pub id: i32,
        pub game_name: String,
        pub tag_line: String,
        pub puuid: String,
        pub platform: String,
        pub updated_at: NaiveDateTime,
        pub summoner_level: i64,
        pub profile_icon_id: i32,
        pub pro_slug: Option<String>,
    }
}