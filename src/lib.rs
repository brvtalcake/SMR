#![allow(dead_code)]

mod smr 
{
    pub trait Metric 
    {
        fn distance(&self) -> Option<usize>;
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum StringComparator
    {
        Lev(&'static str, &'static str, [usize; 3]),
        DamLev(&'static str, &'static str, usize),
        Ham(&'static str, &'static str),
    }
    impl Metric for StringComparator
    {
        fn distance(&self) -> Option<usize>
        {
            match self
            {
                StringComparator::Lev(str1, str2, coeffs) => levenshtein::distance(str1, str2, *coeffs),
                StringComparator::DamLev(str1, str2, max_dist) => damereau_levenshtein::distance(str1, str2, *max_dist),
                StringComparator::Ham(str1, str2) => hamming::distance(str1, str2),
            }
        }
    }

    pub mod levenshtein 
    {
        use std::cmp::min as min;
        pub fn distance(str1: &str, str2: &str, coeffs: [usize; 3]) -> Option<usize> 
        {
            if str1.chars().count() == 0
            {
                return Some(str2.chars().count());
            }
            if str2.chars().count() == 0
            {
                return Some(str1.chars().count());
            }
            let mut arr1 : Vec<usize> = vec![0; str2.chars().count()];
            let mut arr2 : Vec<usize> = vec![0; str2.chars().count()];
            let mut tmp : Vec<usize> = vec![0; str2.chars().count()];
            for i in 0 .. str2.chars().count()
            {
                arr1[i] = i;
            }
            for i in 0 .. str1.chars().count() - 1
            {
                arr2[0] = i + 1;
                for j in 0 .. str2.chars().count() - 1
                {
                    let delete_cost = (arr1[j + 1] + 1) * coeffs[0];
                    let insert_cost = (arr2[j] + 1) * coeffs[1];
                    let subs_cost = (if str1.chars().nth(i) == str2.chars().nth(j) 
                    {
                        arr1[j]
                    } 
                    else 
                    {
                        arr1[j] + 1
                    }) * coeffs[2];
                    arr2[j + 1] = min(min(delete_cost, insert_cost), subs_cost);
                }
                // swap
                tmp.copy_from_slice(&arr1);
                arr1.copy_from_slice(&arr2);
                arr2.copy_from_slice(&tmp);
            }
            return Some(arr1[str2.chars().count() - 1]);
        }
    }

    pub mod damereau_levenshtein
    {
        use std::cmp::min as min;
        use std::cmp::max as max;
        // TODO: Implement "my own version" based on the paper
        pub fn distance(str1: &str, str2: &str, max_dist: usize) -> Option<usize>
        {
            let str1_len = str1.chars().count();
            let str2_len = str2.chars().count();
            if str1_len == 0
            {
                return Some(str2_len);
            }
            if str2_len == 0
            {
                return Some(str1_len);
            }
            if str1_len > str2_len
            {
                return distance(str2, str1, max_dist);
            }
            let min_dist = str2_len - str1_len;
            /* let p = min_dist; */
            if min_dist > max_dist
            {
                return None;
            }
            let mut curr_row = vec![0; str1_len + 1];
            let mut prev_row = vec![0; str1_len + 1];
            let mut transp_row = vec![0; str1_len + 1];
            for i in 0 .. str1_len + 1
            {
                prev_row[i] = i;
            }
            for i in 1 .. str2_len + 1 
            {
                let l_sec_ch = str2.chars().nth(i - 1);
                curr_row[0] = i;
                let tmp = max(i as isize - max_dist as isize - 1, 1);
                if tmp <= 0
                {
                    panic!("from <= 0");
                }
                let from = tmp as usize;
                let to = min(i + max_dist + 1, str1_len);
                for j in from .. to + 1
                {
                    let l_fst_ch = str1.chars().nth(j - 1);
                    let cost = if l_fst_ch == l_sec_ch
                    {
                        prev_row[j - 1]
                    }
                    else
                    {
                        prev_row[j - 1] + 1
                    };
                    curr_row[j] = min(min(curr_row[j - 1] + 1, cost), prev_row[j] + 1);
                    if i > 1 && j > 1 && l_fst_ch == str2.chars().nth(i - 2) && l_sec_ch == str1.chars().nth(j - 2)
                    {
                        curr_row[j] = min(curr_row[j], transp_row[j - 2] + 1);
                    }
                }
                if i > min_dist + 1
                {
                    let mut min_dist = curr_row[from];
                    for j in from + 1 .. to + 1
                    {
                        min_dist = min(min_dist, curr_row[j]);
                    }
                    if min_dist > max_dist
                    {
                        return None;
                    }
                }
                transp_row.copy_from_slice(&prev_row);
                prev_row.copy_from_slice(&curr_row);

            }
            return Some(curr_row[str1_len]);
        }
    }

    pub mod hamming
    {
        pub fn distance(str1: &str, str2: &str) -> Option<usize>
        {
            Some(1)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use smr::Metric;

    #[test]
    fn test_levenshtein() 
    {
        let str1 = "kitten";
        let str2 = "sitting";
        let mut coeffs = [1, 1, 1];
        let mut dist = smr::levenshtein::distance(str1, str2, coeffs).unwrap_or_else(|| usize::MAX);
        assert_eq!(dist, 3);

        coeffs = [1, 1, 2];
        dist = smr::levenshtein::distance(str1, str2, coeffs).unwrap_or_else(|| usize::MAX);
        assert_eq!(dist, 11);

        let comparator = smr::StringComparator::Lev(str1, str2, coeffs);
        dist = comparator.distance().unwrap_or_else(|| usize::MAX);
        assert_eq!(dist, 11);
        println!("{:?}", comparator);
    }

    #[test]
    fn test_damereau_levenshtein() 
    {
        let str1 = "avery";
        let str2 = "garvey";
        let base_exp: usize = 2;
        let max_dist = base_exp.checked_pow(7).unwrap_or_else(|| panic!("{}^{} overflows", base_exp, 7));
        let dist = smr::damereau_levenshtein::distance(str1, str2, max_dist).unwrap_or_else(|| usize::MAX);
        assert_eq!(dist, 3);
    }
}
