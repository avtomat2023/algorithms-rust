use std::collections::HashMap;

pub fn recursive(n: u32, prices: &[u32]) -> u32 {
    (1..=n)
        .map(|left| prices[left as usize] + recursive(n - left, prices))
        .max()
        .unwrap_or(0)
}

pub fn bottom_up_dp(n: usize, prices: &[u32]) -> u32 {
    let mut table = vec![0; n+1];
    for i in 1..=n {
        table[i] = (1..=i)
            .map(|left| prices[left as usize] + table[i - left])
            .max()
            .unwrap()
    }
    table[n]
}

pub fn top_down_dp_hash_map(n: usize, prices: &[u32]) -> u32 {
    fn go(n: usize, prices: &[u32], memo: &mut HashMap<usize, u32>) -> u32 {
        memo.get(&n).cloned().unwrap_or_else(|| {
            let max_price = (1..=n)
                .map(|left| prices[left] + go(n - left, prices, memo))
                .max()
                .unwrap_or(0);
            memo.insert(n, max_price);
            max_price
        })
    }

    go(n, prices, &mut HashMap::new())
}
