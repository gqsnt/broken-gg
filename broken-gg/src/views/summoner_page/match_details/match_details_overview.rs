use common::consts::champion::Champion;
use common::consts::item::Item;
use common::consts::perk::Perk;
use common::consts::summoner_spell::SummonerSpell;
use common::consts::{HasStaticBgAsset};
use crate::utils::{format_with_spaces, summoner_encounter_url, summoner_url};
use crate::views::summoner_page::match_details::LolMatchParticipantDetails;
use crate::views::summoner_page::Summoner;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use crate::views::{ImgBg, ImgOptBg};

#[component]
pub fn MatchDetailsOverview(
    summoner_id: i32,
    match_details: ReadSignal<Vec<LolMatchParticipantDetails>>,
) -> impl IntoView {
    let details = match_details();
    let (summoner_team, summoner_team_won) = {
        let detail = details
            .iter()
            .find(|participant| participant.summoner_id == summoner_id)
            .expect("Summoner id not found");
        (detail.team_id, detail.won)
    };
    let other_team = if summoner_team == 100 { 200 } else { 100 };
    let first_team = details
        .iter()
        .filter(|participant| participant.team_id == summoner_team)
        .cloned()
        .collect::<Vec<_>>();
    let second_team = details
        .iter()
        .filter(|participant| participant.team_id != summoner_team)
        .cloned()
        .collect::<Vec<_>>();
    view! {
        <div>
            <MatchDetailsOverviewTable
                won=summoner_team_won
                team_id=summoner_team
                participants=first_team
            />
            <MatchDetailsOverviewTable
                won=!summoner_team_won
                team_id=other_team
                participants=second_team
            />

        </div>
    }
}

#[component]
pub fn MatchDetailsOverviewTable(
    won: bool,
    team_id: u16,
    participants: Vec<LolMatchParticipantDetails>,
) -> impl IntoView {
    let summoner = expect_context::<Summoner>();
    view! {
        <table class="table-fixed text-xs w-full border-collapse">
            <colgroup>
                <col width="44" />
                <col width="18" />
                <col width="18" />
                <col />
                <col width="98" />
                <col width="120" />
                <col width="48" />
                <col width="56" />
                <col width="175" />
            </colgroup>
            <thead>
                <tr>
                    <th colspan="4">
                        <span>{if won { "Victory" } else { "Defeat" }}</span>
                        {if team_id == 100 { "(Blue Team)" } else { "(Red Team)" }}
                    </th>
                    <th>KDA</th>
                    <th>Damage</th>
                    <th>Wards</th>
                    <th>CS</th>
                    <th>Item</th>
                </tr>
            </thead>
            <tbody>
                {participants
                    .into_iter()
                    .map(|participant| {
                        let champion = Champion::from(participant.champion_id);
                        let summoner_spell1 = SummonerSpell::from(participant.summoner_spell1_id);
                        let summoner_spell2 = SummonerSpell::from(participant.summoner_spell2_id);
                        let primary_perk_selection = Perk::from(
                            participant.perk_primary_selection_id,
                        );
                        let sub_perk_style = Perk::from(participant.perk_sub_style_id);
                        let item0 = Item::try_from(participant.item0_id).ok();
                        let item1 = Item::try_from(participant.item1_id).ok();
                        let item2 = Item::try_from(participant.item2_id).ok();
                        let item3 = Item::try_from(participant.item3_id).ok();
                        let item4 = Item::try_from(participant.item4_id).ok();
                        let item5 = Item::try_from(participant.item5_id).ok();
                        let item6 = Item::try_from(participant.item6_id).ok();
                        let is_pro_player = participant.summoner_pro_player_slug.is_some();
                        let summoner_game_name_clone = summoner.game_name.clone();
                        let summoner_tag_line_clone = summoner.tag_line.clone();
                        let participant_game_name_clone = participant.game_name.clone();
                        let participant_tag_line_clone = participant.tag_line.clone();

                        view! {
                            <tr
                                class=("bg-red-900", !won && participant.summoner_id != summoner.id)
                                class=("bg-blue-900", won && participant.summoner_id != summoner.id)
                                class=("bg-red-800", !won && participant.summoner_id == summoner.id)
                                class=("bg-blue-800", won && participant.summoner_id == summoner.id)
                            >
                                <td class="pl-2.5 py-1">
                                    <div class="sprite-wrapper w-8 h-8">
                                        <ImgBg
                                            alt=champion.to_str().to_string()
                                            class=format!(
                                                "sprite-inner rounded-full block scale-66 {}",
                                                champion.get_class_name(),
                                            )
                                        >
                                            <span class="absolute left-[-3px] bottom-[-3px] w-[15px] h-[15px] bg-gray-600 rounded-full text-[10px] text-center">
                                                {participant.champ_level}
                                            </span>
                                        </ImgBg>
                                    </div>

                                </td>
                                <td class="py-1">
                                    <div class="w-4 h-4 sprite-wrapper">
                                        <ImgBg
                                            alt=summoner_spell1.to_string()
                                            class=format!(
                                                "sprite-inner scale-72 rounded {}",
                                                summoner_spell1.get_class_name(),
                                            )
                                        />
                                    </div>
                                    <div class="w-4 h-4 sprite-wrapper">
                                        <ImgBg
                                            alt=summoner_spell2.to_string()
                                            class=format!(
                                                "sprite-inner scale-72 rounded {}",
                                                summoner_spell2.get_class_name(),
                                            )
                                        />
                                    </div>

                                </td>
                                <td class="py-1">
                                    <div class="w-4 h-4 sprite-wrapper">
                                        <ImgBg
                                            alt=primary_perk_selection.to_string()
                                            class=format!(
                                                "sprite-inner scale-57 rounded {}",
                                                primary_perk_selection.get_class_name(),
                                            )
                                        />
                                    </div>
                                    <div class="w-4 h-4 sprite-wrapper">
                                        <ImgBg
                                            alt=sub_perk_style.to_string()
                                            class=format!(
                                                "sprite-inner scale-57 rounded {}",
                                                sub_perk_style.get_class_name(),
                                            )
                                        />
                                    </div>
                                </td>
                                <td class="pl-[5px] py-1 text-ellipsis overflow-hidden text-left">
                                    <div class="flex items-center gap-1">
                                        <Show when=move || (participant.encounter_count > 1)>
                                            <a
                                                href=summoner_encounter_url(
                                                    summoner.platform.as_ref(),
                                                    summoner_game_name_clone.as_str(),
                                                    summoner_tag_line_clone.as_str(),
                                                    participant.platform.as_ref(),
                                                    participant_game_name_clone.as_str(),
                                                    participant_tag_line_clone.as_str(),
                                                )
                                                class="text-xs bg-green-800 rounded px-0.5 text-center"
                                            >
                                                {participant.encounter_count}
                                            </a>
                                        </Show>
                                        <Show when=move || is_pro_player>
                                            <a
                                                target="_blank"
                                                href=format!(
                                                    "https://lolpros.gg/player/{}",
                                                    participant.summoner_pro_player_slug.unwrap().as_ref(),
                                                )
                                                class="text-xs bg-purple-800 rounded px-0.5 text-center"
                                            >
                                                pro
                                            </a>
                                        </Show>
                                        <a
                                            target="_blank"
                                            href=summoner_url(
                                                participant.platform.as_ref(),
                                                participant.game_name.as_str(),
                                                participant.tag_line.as_str(),
                                            )
                                        >
                                            {participant.game_name.clone()}
                                        </a>
                                    </div>
                                    <span class="text-[11px]">
                                        Lvl. {participant.summoner_level}
                                    </span>
                                </td>
                                <td class="py-1 text-center">
                                    <div class="flex  justify-center">
                                        {participant.kills}/{participant.deaths}/
                                        {participant.assists}
                                        <div class="ml-1 relative">
                                            {participant.kill_participation}%
                                        </div>
                                    </div>
                                </td>
                                <td class="py-1">
                                    <div class="flex justify-center space-x-1">
                                        <div>
                                            {format_with_spaces(participant.damage_dealt_to_champions)}
                                        </div>
                                        <span>-</span>

                                        <div>{format_with_spaces(participant.damage_taken)}</div>
                                    </div>
                                </td>
                                <td class="py-1">
                                    <div class="flex justify-center">
                                        <div>{participant.wards_placed}</div>
                                    </div>
                                </td>
                                <td class="py-1">
                                    <div class="flex justify-center">
                                        <div>{participant.cs}</div>
                                    </div>
                                </td>
                                <td class="py-1">
                                    <div class="flex gap-0.5">
                                        <ImgOptBg
                                            when=move || item0.is_some()
                                            alt=item0.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item0.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item1.is_some()
                                            alt=item1.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item1.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item2.is_some()
                                            alt=item2.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item2.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item3.is_some()
                                            alt=item3.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item3.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item4.is_some()
                                            alt=item4.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item4.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item5.is_some()
                                            alt=item5.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item5.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                        <ImgOptBg
                                            when=move || item6.is_some()
                                            alt=item6.map(|i| i.to_string()).unwrap_or_default()
                                            class=format!(
                                                "w-[22px] w-[22px] rounded {}",
                                                item6.map(|i| i.get_class_name()).unwrap_or_default(),
                                            )
                                        />
                                    </div>
                                </td>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>()}
            </tbody>
        </table>
    }
}