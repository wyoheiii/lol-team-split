use crate::domain::{Division, Player, Rank, Role, TierBelowMaster, MasterLeague};
use crate::domain::states::Lobby;

fn br(tier: TierBelowMaster, div: Division) -> Rank{
  Rank::BelowMaster { tier, division: div }
}

fn mr(tier: MasterLeague, lp: u32) -> Rank {
    Rank::MasterLeague { tier, lp }
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
//pub fn sample_lobbies() -> [(&'static str, Lobby); 10] {
pub fn sample_lobbies() -> [(&'static str, Lobby); 3] {
    use crate::domain::Division::*; use crate::domain::Role::*; use crate::domain::TierBelowMaster::*;
    use crate::domain::MasterLeague::*;

    // L1: バランス系（各ロール2名ずつメイン）＋広いレート帯
    let l1 = lobby("L1_BALANCED", vec![
        p("L1_ChalMid", mr(Challenger, 720), Mid, &[Jg, Top]),
        p("L1_GM_Jg",   mr(Grandmaster, 310), Jg, &[Mid, Top]),
        p("L1_MasTop",  mr(Master, 140), Top, &[Mid]),
        p("L1_DiaAdc",  br(Diamond, I), Adc, &[Sup, Mid]),
        p("L1_EmeSup",  br(Emerald, I), Sup, &[Adc]),
        p("L1_PlatMid", br(Platinum, I), Mid, &[Top]),
        p("L1_GoldTop", br(Gold, I), Top, &[Jg]),
    p("L1_GoldJg",  br(Gold, II), Jg, &[Top]),
    p("L1_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    p("L1_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    ]);

    // L2: Mid 過多（Midメイン4名）＋上位帯多め
    let l2 = lobby("L2_MID_HEAVY", vec![
    p("L2_ChalMid", mr(Challenger, 650), Mid, &[Jg]),
    p("L2_GM_Mid",  mr(Grandmaster, 280), Mid, &[Top]),
    p("L2_MasMid",  mr(Master, 120), Mid, &[Jg]),
    p("L2_DiaMid",  br(Diamond, I), Mid, &[Top]),
    p("L2_DiaJg",   br(Diamond, II), Jg, &[Mid]),
    p("L2_EmeTop",  br(Emerald, I), Top, &[Jg]),
    p("L2_PlatAdc", br(Platinum, II), Adc, &[Sup]),
    p("L2_GoldSup", br(Gold, I), Sup, &[Adc]),
    p("L2_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    p("L2_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    ]);

    // L3: Jg 不足（Jgメイン1名のみ）＋低～中帯中心
    let l3 = lobby("L3_JG_SCARCE", vec![
    p("L3_DiaTop",  br(Diamond, III), Top, &[Mid]),
    p("L3_PlatTop", br(Platinum, I), Top, &[Jg]),
    p("L3_PlatMid", br(Platinum, II), Mid, &[Top]),
    p("L3_GoldMid", br(Gold, I), Mid, &[Adc]),
    p("L3_GoldAdc", br(Gold, II), Adc, &[Sup]),
    p("L3_GoldSup", br(Gold, III), Sup, &[Adc]),
    p("L3_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    p("L3_SlvSup",  br(Silver, II), Sup, &[Adc]),
    p("L3_BrzTop",  br(Bronze, I), Top, &[Sup]),
    p("L3_EmeJg",   br(Emerald, IV), Jg, &[Top]), // ここだけ Jg メイン
    ]);

    // // L4: Sup 過多（Supメイン4名）＋上下ブレ幅大
    // let l4 = lobby("L4_SUP_HEAVY", vec![
    // p("L4_ChalAdc", mr(Challenger, 700), Adc, &[Sup]),
    // p("L4_GM_Sup",  mr(Grandmaster, 320), Sup, &[Adc]),
    // p("L4_MasSup",  mr(Master, 160), Sup, &[Adc]),
    // p("L4_DiaSup",  br(Diamond, I), Sup, &[Adc]),
    // p("L4_EmeSup",  br(Emerald, I), Sup, &[Adc]),
    // p("L4_PlatMid", br(Platinum, II), Mid, &[Top]),
    // p("L4_GoldJg",  br(Gold, I), Jg, &[Top]),
    // p("L4_SlvTop",  br(Silver, I), Top, &[Jg]),
    // p("L4_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // p("L4_BrzTop",  br(Bronze, II), Top, &[Sup]),
    // ]);

    // // L5: Top 多め（Topメイン4名）＋中帯中心
    // let l5 = lobby("L5_TOP_HEAVY", vec![
    // p("L5_DiaTop",  br(Diamond, II), Top, &[Mid]),
    // p("L5_EmeTop",  br(Emerald, I), Top, &[Jg]),
    // p("L5_EmeTop2", br(Emerald, III), Top, &[Adc]),
    // p("L5_PlatTop", br(Platinum, I), Top, &[Sup]),
    // p("L5_PlatMid", br(Platinum, II), Mid, &[Top]),
    // p("L5_GoldMid", br(Gold, I), Mid, &[Top]),
    // p("L5_GoldJg",  br(Gold, II), Jg, &[Top]),
    // p("L5_GoldAdc", br(Gold, I), Adc, &[Sup]),
    // p("L5_SlvSup",  br(Silver, I), Sup, &[Adc]),
    // p("L5_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // ]);

    // // L6: Adc 多め（Adcメイン4名）＋LPなし層多め
    // let l6 = lobby("L6_ADC_HEAVY", vec![
    // p("L6_DiaAdc",  br(Diamond, I), Adc, &[Sup]),
    // p("L6_PlatAdc", br(Platinum, I), Adc, &[Sup, Mid]),
    // p("L6_PlatAdc2",br(Platinum, III), Adc, &[Sup]),
    // p("L6_GoldAdc", br(Gold, I), Adc, &[Sup]),
    // p("L6_GoldSup", br(Gold, II), Sup, &[Adc]),
    // p("L6_GoldMid", br(Gold, III), Mid, &[Adc]),
    // p("L6_SlvTop",  br(Silver, I), Top, &[Jg]),
    // p("L6_SlvJg",   br(Silver, II), Jg, &[Top]),
    // p("L6_BrzMid",  br(Bronze, I), Mid, &[Adc]),
    // p("L6_BrzSup",  br(Bronze, II), Sup, &[Adc]),
    // ]);

    // // L7: 上位帯密集（Chal/GM/Master 多め）→ 極端な強弱差を含む
    // let l7 = lobby("L7_HIGH_TIER", vec![
    // p("L7_ChalMid", mr(Challenger, 680), Mid, &[Jg]),
    // p("L7_ChalTop", mr(Challenger, 640), Top, &[Mid]),
    // p("L7_GM_Jg",   mr(Grandmaster, 300), Jg, &[Mid]),
    // p("L7_GM_Sup",  mr(Grandmaster, 280), Sup, &[Adc]),
    // p("L7_MasAdc",  mr(Master, 180), Adc, &[Sup]),
    // p("L7_MasMid",  mr(Master, 130), Mid, &[Jg]),
    // p("L7_DiaTop",  br(Diamond, I), Top, &[Mid]),
    // p("L7_PlatSup", br(Platinum, I), Sup, &[Adc]),
    // p("L7_GoldJg",  br(Gold, I), Jg, &[Top]),
    // p("L7_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // ]);

    // // L8: 下位帯密集（Bronze/Silver/Gold中心）→ ロール偏りもあり
    // let l8 = lobby("L8_LOW_TIER", vec![
    // p("L8_GoldMid", br(Gold, II), Mid, &[Top]),
    // p("L8_GoldTop", br(Gold, III), Top, &[Jg]),
    // p("L8_SlvTop",  br(Silver, I), Top, &[Mid]),
    // p("L8_SlvMid",  br(Silver, II), Mid, &[Adc]),
    // p("L8_SlvAdc",  br(Silver, III), Adc, &[Sup]),
    // p("L8_SlvSup",  br(Silver, IV), Sup, &[Adc]),
    // p("L8_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    // p("L8_BrzAdc",  br(Bronze, II), Adc, &[Sup]),
    // p("L8_BrzTop",  br(Bronze, III), Top, &[Sup]),
    // p("L8_BrzJg",   br(Bronze, IV), Jg, &[Top]),
    // ]);

    // // L9: Master 以上の LP 差テスト（LPを幅広く）
    // let l9 = lobby("L9_LP_VARIANCE", vec![
    // p("L9_Chal700", mr(Challenger, 700), Mid, &[Jg]),
    // p("L9_Chal500", mr(Challenger, 500), Top, &[Mid]),
    // p("L9_GM350",   mr(Grandmaster, 350), Jg, &[Mid]),
    // p("L9_GM180",   mr(Grandmaster, 180), Sup, &[Adc]),
    // p("L9_Mas200",  mr(Master, 200), Adc, &[Sup]),
    // p("L9_Mas120",  mr(Master, 120), Mid, &[Top]),
    // p("L9_DiaI",    br(Diamond, I), Top, &[Mid]),
    // p("L9_DiaIII",  br(Diamond, III), Jg, &[Top]),
    // p("L9_EmeI",    br(Emerald, I), Sup, &[Adc]),
    // p("L9_PlatI",   br(Platinum, I), Adc, &[Sup]),
    // ]);

    // // L10: 極端な偏り（Challenger 1 + Bronze 9）→ 平均の罠テスト
    // let l10 = lobby("L10_EXTREME_SKEW", vec![
    // p("L10_Chal",  mr(Challenger, 800), Mid, &[Jg, Top]),
    // p("L10_B1",    br(Bronze, I), Top, &[Sup]),
    // p("L10_B2",    br(Bronze, I), Jg,  &[Top]),
    // p("L10_B3",    br(Bronze, II), Mid, &[Adc]),
    // p("L10_B4",    br(Bronze, II), Adc, &[Sup]),
    // p("L10_B5",    br(Bronze, III), Sup, &[Adc]),
    // p("L10_B6",    br(Bronze, III), Top, &[Jg]),
    // p("L10_B7",    br(Bronze, IV), Mid, &[Top]),
    // p("L10_B8",    br(Bronze, IV), Adc, &[Sup]),
    // p("L10_B9",    br(Bronze, IV), Sup, &[Adc]),
    // ]);

    //[l1,l2,l3,l4,l5,l6,l7,l8,l9,l10]
    [l1,l2,l3]
}