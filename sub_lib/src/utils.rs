// Copyright (c) 2017-2018, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.

macro_rules! try_opt {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return None
        }
    }
}

macro_rules! try_flg {
    ($e:expr) => {
        match $e {
            Some(x) => x,
            None => return false
        }
    }
}

pub fn index_of<T> (haystack: &[T], needle: &[T]) -> Option<usize> where T: PartialEq {
    if needle.len () == 0 {panic! ("Can't find index of empty subsequence")}
    for h in 0..haystack.len () {
        let mut mismatch = false;
        for n in 0..needle.len () {
            let i = h + n;
            if i >= haystack.len () {mismatch = true; break}
            if haystack[h + n] != needle[n] {mismatch = true; break}
        }
        if !mismatch {return Some (h)}
    }
    None
}

pub fn index_of_from<T> (haystack: &Vec<T>, needle: &T, start_at: usize) -> Option<usize> where T: PartialEq {
    let mut index = start_at;
    while index < haystack.len () && (haystack[index] != *needle) {
        index += 1;
    }
    if index >= haystack.len () {None}
        else {Some (index)}
}

pub fn accumulate<F, R> (mut f: F) -> Vec<R> where F: FnMut () -> Option<R> {
    let mut result: Vec<R> = Vec::new ();
    loop {
        match f () {
            Some (r) => result.push (r),
            None => break
        }
    }
    result
}

pub fn make_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}

pub fn to_string (data: &Vec<u8>) -> String {
    match String::from_utf8 (data.clone ()) {
        Ok (string) => string,
        Err (_) => format! ("{:?}", data)
    }
}

pub fn to_string_s(data: &[u8]) -> String {
    match String::from_utf8 (Vec::from (data)) {
        Ok (string) => string,
        Err (_) => format! ("{:?}", data)
    }
}


#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn index_of_fails_to_find_nonexistent_needle_in_haystack() {
        let result = index_of("haystack".as_bytes(), "needle".as_bytes());

        assert_eq!(result, None);
    }

    #[test]
    fn index_of_finds_needle_at_beginning_of_haystack() {
        let result = index_of("haystack haystack".as_bytes(), "haystack".as_bytes());

        assert_eq!(result, Some(0));
    }

    #[test]
    fn index_of_finds_needle_at_end_of_haystack() {
        let result = index_of("needle haystack".as_bytes(), "haystack".as_bytes());

        assert_eq!(result, Some(7));
    }

    #[test]
    fn index_of_fails_to_find_nonempty_needle_in_empty_haystack() {
        let result = index_of("".as_bytes(), "needle".as_bytes());

        assert_eq!(result, None);
    }

    #[test]
    #[should_panic(expected = "Can't find index of empty subsequence")]
    fn index_of_panics_with_empty_needle() {
        index_of("haystack".as_bytes(), "".as_bytes());
    }

    #[test]
    fn index_of_fails_to_find_needle_that_ends_past_end_of_haystack() {
        let result = index_of("haystack needl".as_bytes(), "needle".as_bytes());

        assert_eq!(result, None);
    }

    #[test]
    fn index_of_from_fails_to_find_nonexistent_needle_in_haystack () {
        let haystack = vec! (true, true, true, true);

        let result = index_of_from (&haystack, &false, 0);

        assert_eq! (result, None);
    }

    #[test]
    fn index_of_from_fails_to_find_needle_in_empty_haystack () {
        let haystack: Vec<i32> = vec! ();

        let result = index_of_from (&haystack, &-42, 0);

        assert_eq! (result, None);
    }

    #[test]
    fn index_of_from_finds_needle_at_beginning_of_search () {
        let haystack = vec! (8, 7, 8, 3);

        let result = index_of_from (&haystack, &8, 2);

        assert_eq! (result, Some (2));
    }

    #[test]
    fn index_of_from_finds_needle_at_end_of_haystack () {
        let haystack = vec! (8, 7, 8, 3);

        let result = index_of_from (&haystack, &3, 0);

        assert_eq! (result, Some (3));
    }

    #[test]
    fn accumulate_returns_empty_vec_for_immediate_none () {
        let result: Vec<i32> = accumulate (|| {None});

        assert_eq! (result.len (), 0);
    }

    #[test]
    fn accumulate_can_mutate_environment () {
        let mut values = vec! (3, 2, 1);

        let result = accumulate (|| {values.pop ()});

        assert_eq! (values, vec! ());
        assert_eq! (result, vec! (1, 2, 3));
    }
}