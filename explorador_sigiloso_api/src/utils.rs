use crate::types::BasisPoint;


#[allow(dead_code)]
pub fn sats_to_btc(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}

pub fn expected_block_subsidy(height: usize) -> u64 {
    let halvings = height as u64 / 210_000;
    let initial_subsidy_sats = 50 * 100_000_000; // 50 BTC in sats

    // right shift is equivalent to dividing by 2^halvings
    initial_subsidy_sats >> halvings
}

pub fn get_basis_point_from(numerator: u64, denominator: u64) -> BasisPoint {
    if denominator == 0 {
        return 0;
    }
    let basis_point = (numerator * 10_000) / denominator;
    assert!(basis_point <= BasisPoint::MAX as u64, "Basis point exceeds u16 limit");
    basis_point as BasisPoint
}

