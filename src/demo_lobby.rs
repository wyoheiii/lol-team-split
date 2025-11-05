use crate::domain::{Division, Player, Rank, Role, Tier};
use crate::pipeline::states::Lobby;

fn r(tier: Tier, div: Option<Division>, lp: usize) -> Rank{
  Rank{ tier, division: div, lp }
}

fn p(name: &str, rank: Rank, main: Role, subs: &[Role]) -> Player {
    Player {
        name: name.to_string(),
        rank,
        main_role: main,
        sub_role: subs.to_vec(),
    }
}

fn lobby(id: &'static str, v: Vec<Player>) -> (&'static str, Lobby) {
  (id, Lobby::new(v.try_into().unwrap()))
}

// ===== 10 Lobbies =====
pub fn sample_lobbies() -> [(&'static str, Lobby); 10] {
    use crate::domain::Division::*; use crate::domain::Role::*; use crate::domain::Tier::*;

    // L1: バランス系（各ロール2名ずつメイン）＋広いレート帯
    let l1 = lobby("L1_BALANCED", vec![
        p("L1_ChalMid", r(Challenger, None, 720), Mid, &[Jg, Top]),
        p("L1_GM_Jg",   r(Grandmaster, None, 310), Jg, &[Mid, Top]),
        p("L1_MasTop",  r(Master, None, 140), Top, &[Mid]),
        p("L1_DiaAdc",  r(Diamond, Some(I), 0), Adc, &[Sup, Mid]),
        p("L1_EmeSup",  r(Emerald, Some(II), 0), Sup, &[Adc]),
        p("L1_PlatMid", r(Platinum, Some(I), 0), Mid, &[Top]),
        p("L1_GoldTop", r(Gold, Some(I), 0), Top, &[Jg]),
        p("L1_GoldJg",  r(Gold, Some(II), 0), Jg, &[Top]),
        p("L1_SlvAdc",  r(Silver, Some(I), 0), Adc, &[Sup]),
        p("L1_BrzSup",  r(Bronze, Some(I), 0), Sup, &[Adc]),
    ]);

    // L2: Mid 過多（Midメイン4名）＋上位帯多め
    let l2 = lobby("L2_MID_HEAVY", vec![
        p("L2_ChalMid", r(Challenger, None, 650), Mid, &[Jg]),
        p("L2_GM_Mid",  r(Grandmaster, None, 280), Mid, &[Top]),
        p("L2_MasMid",  r(Master, None, 120), Mid, &[Jg]),
        p("L2_DiaMid",  r(Diamond, Some(I), 0), Mid, &[Top]),
        p("L2_DiaJg",   r(Diamond, Some(II), 0), Jg, &[Mid]),
        p("L2_EmeTop",  r(Emerald, Some(I), 0), Top, &[Jg]),
        p("L2_PlatAdc", r(Platinum, Some(II), 0), Adc, &[Sup]),
        p("L2_GoldSup", r(Gold, Some(I), 0), Sup, &[Adc]),
        p("L2_SlvAdc",  r(Silver, Some(I), 0), Adc, &[Sup]),
        p("L2_BrzSup",  r(Bronze, Some(I), 0), Sup, &[Adc]),
    ]);

    // L3: Jg 不足（Jgメイン1名のみ）＋低～中帯中心
    let l3 = lobby("L3_JG_SCARCE", vec![
        p("L3_DiaTop",  r(Diamond, Some(III), 0), Top, &[Mid]),
        p("L3_PlatTop", r(Platinum, Some(I), 0), Top, &[Jg]),
        p("L3_PlatMid", r(Platinum, Some(II), 0), Mid, &[Top]),
        p("L3_GoldMid", r(Gold, Some(I), 0), Mid, &[Adc]),
        p("L3_GoldAdc", r(Gold, Some(II), 0), Adc, &[Sup]),
        p("L3_GoldSup", r(Gold, Some(III), 0), Sup, &[Adc]),
        p("L3_SlvAdc",  r(Silver, Some(I), 0), Adc, &[Sup]),
        p("L3_SlvSup",  r(Silver, Some(II), 0), Sup, &[Adc]),
        p("L3_BrzTop",  r(Bronze, Some(I), 0), Top, &[Sup]),
        p("L3_EmeJg",   r(Emerald, Some(IV), 0), Jg, &[Top]), // ここだけ Jg メイン
    ]);

    // L4: Sup 過多（Supメイン4名）＋上下ブレ幅大
    let l4 = lobby("L4_SUP_HEAVY", vec![
        p("L4_ChalAdc", r(Challenger, None, 700), Adc, &[Sup]),
        p("L4_GM_Sup",  r(Grandmaster, None, 320), Sup, &[Adc]),
        p("L4_MasSup",  r(Master, None, 160), Sup, &[Adc]),
        p("L4_DiaSup",  r(Diamond, Some(I), 0), Sup, &[Adc]),
        p("L4_EmeSup",  r(Emerald, Some(I), 0), Sup, &[Adc]),
        p("L4_PlatMid", r(Platinum, Some(II), 0), Mid, &[Top]),
        p("L4_GoldJg",  r(Gold, Some(I), 0), Jg, &[Top]),
        p("L4_SlvTop",  r(Silver, Some(I), 0), Top, &[Jg]),
        p("L4_BrzAdc",  r(Bronze, Some(I), 0), Adc, &[Sup]),
        p("L4_BrzTop",  r(Bronze, Some(II), 0), Top, &[Sup]),
    ]);

    // L5: Top 多め（Topメイン4名）＋中帯中心
    let l5 = lobby("L5_TOP_HEAVY", vec![
        p("L5_DiaTop",  r(Diamond, Some(II), 0), Top, &[Mid]),
        p("L5_EmeTop",  r(Emerald, Some(I), 0), Top, &[Jg]),
        p("L5_EmeTop2", r(Emerald, Some(III), 0), Top, &[Adc]),
        p("L5_PlatTop", r(Platinum, Some(I), 0), Top, &[Sup]),
        p("L5_PlatMid", r(Platinum, Some(II), 0), Mid, &[Top]),
        p("L5_GoldMid", r(Gold, Some(I), 0), Mid, &[Top]),
        p("L5_GoldJg",  r(Gold, Some(II), 0), Jg, &[Top]),
        p("L5_GoldAdc", r(Gold, Some(I), 0), Adc, &[Sup]),
        p("L5_SlvSup",  r(Silver, Some(I), 0), Sup, &[Adc]),
        p("L5_BrzAdc",  r(Bronze, Some(I), 0), Adc, &[Sup]),
    ]);

    // L6: Adc 多め（Adcメイン4名）＋LPなし層多め
    let l6 = lobby("L6_ADC_HEAVY", vec![
        p("L6_DiaAdc",  r(Diamond, Some(I), 0), Adc, &[Sup]),
        p("L6_PlatAdc", r(Platinum, Some(I), 0), Adc, &[Sup, Mid]),
        p("L6_PlatAdc2",r(Platinum, Some(III), 0), Adc, &[Sup]),
        p("L6_GoldAdc", r(Gold, Some(I), 0), Adc, &[Sup]),
        p("L6_GoldSup", r(Gold, Some(II), 0), Sup, &[Adc]),
        p("L6_GoldMid", r(Gold, Some(III), 0), Mid, &[Adc]),
        p("L6_SlvTop",  r(Silver, Some(I), 0), Top, &[Jg]),
        p("L6_SlvJg",   r(Silver, Some(II), 0), Jg, &[Top]),
        p("L6_BrzMid",  r(Bronze, Some(I), 0), Mid, &[Adc]),
        p("L6_BrzSup",  r(Bronze, Some(II), 0), Sup, &[Adc]),
    ]);

    // L7: 上位帯密集（Chal/GM/Master 多め）→ 極端な強弱差を含む
    let l7 = lobby("L7_HIGH_TIER", vec![
        p("L7_ChalMid", r(Challenger, None, 680), Mid, &[Jg]),
        p("L7_ChalTop", r(Challenger, None, 640), Top, &[Mid]),
        p("L7_GM_Jg",   r(Grandmaster, None, 300), Jg, &[Mid]),
        p("L7_GM_Sup",  r(Grandmaster, None, 280), Sup, &[Adc]),
        p("L7_MasAdc",  r(Master, None, 180), Adc, &[Sup]),
        p("L7_MasMid",  r(Master, None, 130), Mid, &[Jg]),
        p("L7_DiaTop",  r(Diamond, Some(I), 0), Top, &[Mid]),
        p("L7_PlatSup", r(Platinum, Some(I), 0), Sup, &[Adc]),
        p("L7_GoldJg",  r(Gold, Some(I), 0), Jg, &[Top]),
        p("L7_BrzAdc",  r(Bronze, Some(I), 0), Adc, &[Sup]),
    ]);

    // L8: 下位帯密集（Bronze/Silver/Gold中心）→ ロール偏りもあり
    let l8 = lobby("L8_LOW_TIER", vec![
        p("L8_GoldMid", r(Gold, Some(II), 0), Mid, &[Top]),
        p("L8_GoldTop", r(Gold, Some(III), 0), Top, &[Jg]),
        p("L8_SlvTop",  r(Silver, Some(I), 0), Top, &[Mid]),
        p("L8_SlvMid",  r(Silver, Some(II), 0), Mid, &[Adc]),
        p("L8_SlvAdc",  r(Silver, Some(III), 0), Adc, &[Sup]),
        p("L8_SlvSup",  r(Silver, Some(IV), 0), Sup, &[Adc]),
        p("L8_BrzSup",  r(Bronze, Some(I), 0), Sup, &[Adc]),
        p("L8_BrzAdc",  r(Bronze, Some(II), 0), Adc, &[Sup]),
        p("L8_BrzTop",  r(Bronze, Some(III), 0), Top, &[Sup]),
        p("L8_BrzJg",   r(Bronze, Some(IV), 0), Jg, &[Top]),
    ]);

    // L9: Master 以上の LP 差テスト（LPを幅広く）
    let l9 = lobby("L9_LP_VARIANCE", vec![
        p("L9_Chal700", r(Challenger, None, 700), Mid, &[Jg]),
        p("L9_Chal500", r(Challenger, None, 500), Top, &[Mid]),
        p("L9_GM350",   r(Grandmaster, None, 350), Jg, &[Mid]),
        p("L9_GM180",   r(Grandmaster, None, 180), Sup, &[Adc]),
        p("L9_Mas200",  r(Master, None, 200), Adc, &[Sup]),
        p("L9_Mas120",  r(Master, None, 120), Mid, &[Top]),
        p("L9_DiaI",    r(Diamond, Some(I), 0), Top, &[Mid]),
        p("L9_DiaIII",  r(Diamond, Some(III), 0), Jg, &[Top]),
        p("L9_EmeI",    r(Emerald, Some(I), 0), Sup, &[Adc]),
        p("L9_PlatI",   r(Platinum, Some(I), 0), Adc, &[Sup]),
    ]);

    // L10: 極端な偏り（Challenger 1 + Bronze 9）→ 平均の罠テスト
    let l10 = lobby("L10_EXTREME_SKEW", vec![
        p("L10_Chal",  r(Challenger, None, 800), Mid, &[Jg, Top]),
        p("L10_B1",    r(Bronze, Some(I), 0), Top, &[Sup]),
        p("L10_B2",    r(Bronze, Some(I), 0), Jg,  &[Top]),
        p("L10_B3",    r(Bronze, Some(II), 0), Mid, &[Adc]),
        p("L10_B4",    r(Bronze, Some(II), 0), Adc, &[Sup]),
        p("L10_B5",    r(Bronze, Some(III), 0), Sup, &[Adc]),
        p("L10_B6",    r(Bronze, Some(III), 0), Top, &[Jg]),
        p("L10_B7",    r(Bronze, Some(IV), 0), Mid, &[Top]),
        p("L10_B8",    r(Bronze, Some(IV), 0), Adc, &[Sup]),
        p("L10_B9",    r(Bronze, Some(IV), 0), Sup, &[Adc]),
    ]);

    [l1,l2,l3,l4,l5,l6,l7,l8,l9,l10]
}