#![allow(dead_code)]

mod smr 
{
    pub mod levenshtein 
    {
        use std::cmp::min as min;
        pub fn distance(str1: &str, str2: &str, coeffs: &[usize; 3]) -> usize 
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
            return arr1[str2.chars().count() - 1]
        }
    }

    pub mod damereau_levenshtein
    {
        pub fn distance(str1: &str, str2: &str, coeffs: &[usize; 4]) -> usize
        {
            return 0;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() 
    {
        let str1 = "kitten";
        let str2 = "sitting";
        let coeffs = [1, 1, 1];
        let dist = smr::levenshtein::distance(str1, str2, &coeffs);
        assert_eq!(dist, 3);
    }

    #[test]
    fn test_damereau_levenshtein() 
    {
        let str1 = "kitten";
        let str2 = "sitting";
        let coeffs = [1, 1, 1, 1];
        let dist = smr::damereau_levenshtein::distance(str1, str2, &coeffs);
        assert_eq!(dist, 3);
    }
}
