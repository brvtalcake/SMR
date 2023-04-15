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
        DamLev(&'static str, &'static str, usize, [usize; 4]),
        Ham(&'static str, &'static str),
    }
    impl Metric for StringComparator
    {
        fn distance(&self) -> Option<usize>
        {
            match self
            {
                StringComparator::Lev(str1, str2, coeffs) => levenshtein::distance(str1, str2, *coeffs),
                StringComparator::DamLev(str1, str2, alphabet_size, coeffs) => damereau_levenshtein::distance(str1, str2, *alphabet_size, *coeffs),
                StringComparator::Ham(str1, str2) => hamming::distance(str1, str2),
            }
        }
    }

    pub mod levenshtein 
    {
        use std::cmp::min as min;
        pub fn distance(str1: &str, str2: &str, coeffs: [usize; 3]) -> Option<usize> 
        {
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
        pub fn distance(str1: &str, str2: &str, alphabet_size: usize, coeffs: [usize; 4]) -> Option<usize>
        {
            
        }
    }

    pub mod hamming
    {
        pub fn distance(str1: &str, str2: &str) -> Option<usize>
        {
            
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
        let str1 = "kitten";
        let str2 = "sitting";
        let coeffs = [1, 1, 1, 1];
        let base_exp: usize = 2;
        let alphabet_size = base_exp.checked_pow(7).unwrap_or_else(|| panic!("{}^{} overflows", base_exp, 7));
        let dist = smr::damereau_levenshtein::distance(str1, str2, alphabet_size, coeffs).unwrap_or_else(|| usize::MAX);
        assert_eq!(dist, 3);
    }
}
