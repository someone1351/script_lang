
use super::super::super::common::*;

pub fn calc_ind(ind : IntT , len:usize) -> Option<usize> {
    let len : IntT = len.try_into().unwrap_or_default();

    if ind >= len || (ind < 0 && ind.abs() > len) {
        None
    } else {
        let ind = if ind<0 {len+ind} else {ind};
        Some(ind.try_into().unwrap_or_default())
    }
}
