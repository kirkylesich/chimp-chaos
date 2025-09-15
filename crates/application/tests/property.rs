#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::pedantic)]

use application::usecases::compute_impact_score;
use proptest::prelude::*;

proptest! {
    #[test]
    fn impact_score_non_nan(rps_base in 1.0f64..10000.0, rps_fail in 0.0f64..10000.0, p95_base in 1.0f64..10000.0, p95_fail in 0.0f64..10000.0) {
        let res = compute_impact_score(rps_base, rps_fail, p95_base, p95_fail);
        prop_assert!(res.is_ok());
        let v = res.unwrap();
        prop_assert!(!v.is_nan());
    }
}

