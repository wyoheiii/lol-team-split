use crate::domain::{Division, Player, Rank, Role, TierBelowMaster, MasterLeague};
use crate::domain::states::Lobby;

fn br(tier: TierBelowMaster, div: Division) -> Rank{
  Rank::BelowMaster { tier, division: div }
}

fn mr(tier: MasterLeague, lp: u32) -> Rank {
    Rank::MasterLeague { tier, lp }
}

fn p(id: u32, name: &str, rank: Rank, main: Role, subs: &[Role]) -> Player {
    Player {
        id,
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
// pub fn sample_lobbies() -> [(&'static str, Lobby); 3] {
pub fn sample_lobbies() -> [(&'static str, Lobby); 1] {
    use crate::domain::Division::*; use crate::domain::Role::*; use crate::domain::TierBelowMaster::*;
    use crate::domain::MasterLeague::*;

    // L1: バランス系（各ロール2名ずつメイン）＋広いレート帯
    let l1 = lobby("L1_BALANCED", vec![
        p(1, "L1_ChalMid", mr(Challenger, 720), Mid, &[Jg, Top]),
        p(2, "L1_GM_Jg",   mr(Grandmaster, 310), Jg, &[Mid, Top]),
        p(3, "L1_MasTop",  mr(Master, 140), Top, &[Mid]),
        p(4, "L1_DiaAdc",  br(Diamond, I), Adc, &[Sup, Mid]),
        p(5, "L1_EmeSup",  br(Emerald, I), Sup, &[Adc]),
        p(6, "L1_PlatMid", br(Platinum, I), Mid, &[Top]),
        p(7, "L1_GoldTop", br(Gold, I), Top, &[Jg]),
    p(8, "L1_GoldJg",  br(Gold, II), Jg, &[Top]),
    p(9, "L1_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    p(10, "L1_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    ]);

    // // L2: Mid 過多（Midメイン4名）＋上位帯多め
    // let l2 = lobby("L2_MID_HEAVY", vec![
    // p(11, "L2_ChalMid", mr(Challenger, 650), Mid, &[Jg]),
    // p(12, "L2_GM_Mid",  mr(Grandmaster, 280), Mid, &[Top]),
    // p(13, "L2_MasMid",  mr(Master, 120), Mid, &[Jg]),
    // p(14, "L2_DiaMid",  br(Diamond, I), Mid, &[Top]),
    // p(15, "L2_DiaJg",   br(Diamond, II), Jg, &[Mid]),
    // p(16, "L2_EmeTop",  br(Emerald, I), Top, &[Jg]),
    // p(17, "L2_PlatAdc", br(Platinum, II), Adc, &[Sup]),
    // p(18, "L2_GoldSup", br(Gold, I), Sup, &[Adc]),
    // p(19, "L2_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    // p(20, "L2_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    // ]);

    // // L3: Jg 不足（Jgメイン1名のみ）＋低～中帯中心
    // let l3 = lobby("L3_JG_SCARCE", vec![
    // p(21, "L3_DiaTop",  br(Diamond, III), Top, &[Mid]),
    // p(22, "L3_PlatTop", br(Platinum, I), Top, &[Jg]),
    // p(23, "L3_PlatMid", br(Platinum, II), Mid, &[Top]),
    // p(24, "L3_GoldMid", br(Gold, I), Mid, &[Adc]),
    // p(25, "L3_GoldAdc", br(Gold, II), Adc, &[Sup]),
    // p(26, "L3_GoldSup", br(Gold, III), Sup, &[Adc]),
    // p(27, "L3_SlvAdc",  br(Silver, I), Adc, &[Sup]),
    // p(28, "L3_SlvSup",  br(Silver, II), Sup, &[Adc]),
    // p(29, "L3_BrzTop",  br(Bronze, I), Top, &[Sup]),
    // p(30, "L3_EmeJg",   br(Emerald, IV), Jg, &[Top]), // ここだけ Jg メイン
    // ]);

    // // L4: Sup 過多（Supメイン4名）＋上下ブレ幅大
    // let l4 = lobby("L4_SUP_HEAVY", vec![
    // p(31, "L4_ChalAdc", mr(Challenger, 700), Adc, &[Sup]),
    // p(32, "L4_GM_Sup",  mr(Grandmaster, 320), Sup, &[Adc]),
    // p(33, "L4_MasSup",  mr(Master, 160), Sup, &[Adc]),
    // p(34, "L4_DiaSup",  br(Diamond, I), Sup, &[Adc]),
    // p(35, "L4_EmeSup",  br(Emerald, I), Sup, &[Adc]),
    // p(36, "L4_PlatMid", br(Platinum, II), Mid, &[Top]),
    // p(37, "L4_GoldJg",  br(Gold, I), Jg, &[Top]),
    // p(38, "L4_SlvTop",  br(Silver, I), Top, &[Jg]),
    // p(39, "L4_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // p(40, "L4_BrzTop",  br(Bronze, II), Top, &[Sup]),
    // ]);

    // // L5: Top 多め（Topメイン4名）＋中帯中心
    // let l5 = lobby("L5_TOP_HEAVY", vec![
    // p(41, "L5_DiaTop",  br(Diamond, II), Top, &[Mid]),
    // p(42, "L5_EmeTop",  br(Emerald, I), Top, &[Jg]),
    // p(43, "L5_EmeTop2", br(Emerald, III), Top, &[Adc]),
    // p(44, "L5_PlatTop", br(Platinum, I), Top, &[Sup]),
    // p(45, "L5_PlatMid", br(Platinum, II), Mid, &[Top]),
    // p(46, "L5_GoldMid", br(Gold, I), Mid, &[Top]),
    // p(47, "L5_GoldJg",  br(Gold, II), Jg, &[Top]),
    // p(48, "L5_GoldAdc", br(Gold, I), Adc, &[Sup]),
    // p(49, "L5_SlvSup",  br(Silver, I), Sup, &[Adc]),
    // p(50, "L5_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // ]);

    // // L6: Adc 多め（Adcメイン4名）＋LPなし層多め
    // let l6 = lobby("L6_ADC_HEAVY", vec![
    // p(51, "L6_DiaAdc",  br(Diamond, I), Adc, &[Sup]),
    // p(52, "L6_PlatAdc", br(Platinum, I), Adc, &[Sup, Mid]),
    // p(53, "L6_PlatAdc2",br(Platinum, III), Adc, &[Sup]),
    // p(54, "L6_GoldAdc", br(Gold, I), Adc, &[Sup]),
    // p(55, "L6_GoldSup", br(Gold, II), Sup, &[Adc]),
    // p(56, "L6_GoldMid", br(Gold, III), Mid, &[Adc]),
    // p(57, "L6_SlvTop",  br(Silver, I), Top, &[Jg]),
    // p(58, "L6_SlvJg",   br(Silver, II), Jg, &[Top]),
    // p(59, "L6_BrzMid",  br(Bronze, I), Mid, &[Adc]),
    // p(60, "L6_BrzSup",  br(Bronze, II), Sup, &[Adc]),
    // ]);

    // // L7: 上位帯密集（Chal/GM/Master 多め）→ 極端な強弱差を含む
    // let l7 = lobby("L7_HIGH_TIER", vec![
    // p(61, "L7_ChalMid", mr(Challenger, 680), Mid, &[Jg]),
    // p(62, "L7_ChalTop", mr(Challenger, 640), Top, &[Mid]),
    // p(63, "L7_GM_Jg",   mr(Grandmaster, 300), Jg, &[Mid]),
    // p(64, "L7_GM_Sup",  mr(Grandmaster, 280), Sup, &[Adc]),
    // p(65, "L7_MasAdc",  mr(Master, 180), Adc, &[Sup]),
    // p(66, "L7_MasMid",  mr(Master, 130), Mid, &[Jg]),
    // p(67, "L7_DiaTop",  br(Diamond, I), Top, &[Mid]),
    // p(68, "L7_PlatSup", br(Platinum, I), Sup, &[Adc]),
    // p(69, "L7_GoldJg",  br(Gold, I), Jg, &[Top]),
    // p(70, "L7_BrzAdc",  br(Bronze, I), Adc, &[Sup]),
    // ]);

    // // L8: 下位帯密集（Bronze/Silver/Gold中心）→ ロール偏りもあり
    // let l8 = lobby("L8_LOW_TIER", vec![
    // p(71, "L8_GoldMid", br(Gold, II), Mid, &[Top]),
    // p(72, "L8_GoldTop", br(Gold, III), Top, &[Jg]),
    // p(73, "L8_SlvTop",  br(Silver, I), Top, &[Mid]),
    // p(74, "L8_SlvMid",  br(Silver, II), Mid, &[Adc]),
    // p(75, "L8_SlvAdc",  br(Silver, III), Adc, &[Sup]),
    // p(76, "L8_SlvSup",  br(Silver, IV), Sup, &[Adc]),
    // p(77, "L8_BrzSup",  br(Bronze, I), Sup, &[Adc]),
    // p(78, "L8_BrzAdc",  br(Bronze, II), Adc, &[Sup]),
    // p(79, "L8_BrzTop",  br(Bronze, III), Top, &[Sup]),
    // p(80, "L8_BrzJg",   br(Bronze, IV), Jg, &[Top]),
    // ]);

    // // L9: Master 以上の LP 差テスト（LPを幅広く）
    // let l9 = lobby("L9_LP_VARIANCE", vec![
    // p(81, "L9_Chal700", mr(Challenger, 700), Mid, &[Jg]),
    // p(82, "L9_Chal500", mr(Challenger, 500), Top, &[Mid]),
    // p(83, "L9_GM350",   mr(Grandmaster, 350), Jg, &[Mid]),
    // p(84, "L9_GM180",   mr(Grandmaster, 180), Sup, &[Adc]),
    // p(85, "L9_Mas200",  mr(Master, 200), Adc, &[Sup]),
    // p(86, "L9_Mas120",  mr(Master, 120), Mid, &[Top]),
    // p(87, "L9_DiaI",    br(Diamond, I), Top, &[Mid]),
    // p(88, "L9_DiaIII",  br(Diamond, III), Jg, &[Top]),
    // p(89, "L9_EmeI",    br(Emerald, I), Sup, &[Adc]),
    // p(90, "L9_PlatI",   br(Platinum, I), Adc, &[Sup]),
    // ]);

    // // L10: 極端な偏り（Challenger 1 + Bronze 9）→ 平均の罠テスト
    // let l10 = lobby("L10_EXTREME_SKEW", vec![
    // p(91, "L10_Chal",  mr(Challenger, 800), Mid, &[Jg, Top]),
    // p(92, "L10_B1",    br(Bronze, I), Top, &[Sup]),
    // p(93, "L10_B2",    br(Bronze, I), Jg,  &[Top]),
    // p(94, "L10_B3",    br(Bronze, II), Mid, &[Adc]),
    // p(95, "L10_B4",    br(Bronze, II), Adc, &[Sup]),
    // p(96, "L10_B5",    br(Bronze, III), Sup, &[Adc]),
    // p(97, "L10_B6",    br(Bronze, III), Top, &[Jg]),
    // p(98, "L10_B7",    br(Bronze, IV), Mid, &[Top]),
    // p(99, "L10_B8",    br(Bronze, IV), Adc, &[Sup]),
    // p(100, "L10_B9",    br(Bronze, IV), Sup, &[Adc]),
    // ]);

    //[l1,l2,l3,l4,l5,l6,l7,l8,l9,l10]
    //[l1,l2,l3]
    [l1]
}