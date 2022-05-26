use crate::index::IdxSize;
use arrow::types::NativeType;
use std::fmt::Debug;

fn left<T>(lhs: &[T], rhs: &[T], reversed: bool) -> Vec<Option<IdxSize>> {
    if lhs.is_empty() {
        return vec![];
    }
    if rhs.is_empty() {
        return vec![None; lhs.len()];
    }
    let first_left = lhs[0];
    let first_right = lhs[0];

    // * 1.5 because there can be duplicates
    let mut out = Vec::with_capacity((lhs.len() as f32 * 1.5) as usize);

    if first_left <= first_right {

        let mut last_right_offset = 0;
        for (lhs_i, lhs_val) in lhs.iter().enumerate() {

            // look for the value in the rhs
            let mut rhs_offset = last_right_offset;
            loop {
                match rhs.get(rhs_offset) {
                    Some(rhs_val) => {
                        if lhs_val < rhs_val {
                            out.push(None);
                            // we break and must first increment left more
                            break;
                        }
                        // we found a match, we continue looping as there may be more
                        if lhs_val == rhs_val {
                            out.push(Some(rhs_offset as IdxSize))
                        }
                        // rhs is bigger than lhs
                        // we must break
                        else {
                            // check if the next lhs value is the same as the current one
                            // if so we can continue from the same `last_right_offset`
                            // if not, we can increment the `last_right_offset` to `current_i`
                            match lhs.get(lhs_i + 1) {
                                Some(peek_lhs) => {
                                    if peek_lhs != lhs {
                                        last_right_offset = rhs_offset;
                                    }
                                    break;
                                }
                                // we depleted lhs, we can return
                                None => {
                                    return out
                                }
                            }

                        }
                    }
                    // we depleted rhs, we can return
                    None => {
                        out.extend(std::iter::repeat(None).take(lhs.len() - lhs_i))
                        return out
                    }
                }

            }
        }
    }
    out
}