use std::cell::Cell;

thread_local! {
    static NEXT_IMPLEMENTATION_ID: Cell<usize> = Cell::new(0);
}

/// Grab a new implementation ID, thus incrementing the global ID counter.
pub fn grab_new_implementation_id(yin: bool) -> usize {
    NEXT_IMPLEMENTATION_ID.with(|id| {
        let mut return_id = id.get();
        id.set(return_id + 1);

        if !yin && return_id == 0 {
            // things that build on top of Yin are effectively one-indexed instead of zero-indexed
            // like Yin is
            return_id += 1;
            id.set(return_id + 1);
        }

        return_id
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yin_id_zero_indexed() {
        assert_eq!(grab_new_implementation_id(true), 0);
        assert_eq!(grab_new_implementation_id(true), 1);
    }

    #[test]
    fn yang_yin_id_one_indexed() {
        assert_eq!(grab_new_implementation_id(false), 1);
        assert_eq!(grab_new_implementation_id(false), 2);
    }
}
