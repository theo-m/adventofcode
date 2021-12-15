use std::collections::HashMap;

fn input() -> (String, HashMap<&'static str, &'static str>) {
    include_str!("../../inputs/day14.test.txt")
        .split_once("\n\n")
        .and_then(|(c, r)| Some((c.to_string(), r.lines().map(|line| {
            let mut it = line.split(" -> ");
            (it.next().unwrap(), it.next().unwrap())
        }).collect::<HashMap<&str, &str>>())))
        .unwrap()
}

fn p1() {
    let (mut polymer, rules) = input();

    (0..10).for_each(|_| {
        let curr = polymer.clone();
        (0..curr.len() - 1).rev().for_each(|i| {
            polymer = format!("{}{}{}", &polymer[..=i], rules[&curr[i..i + 2]], &polymer[i + 1..]);
        });
    });
    let cnt = polymer.chars().fold(HashMap::new(), |mut acc, v| {
        let pp = acc.entry(v).or_insert(0);
        *pp += 1;
        acc
    });
    let mut v = cnt.values().collect::<Vec<_>>();
    v.sort();
    println!("p1: {:?}", v)
}

fn p2() {
    // that's bad, counting bigrams loses the info we're looking for in the answer:
    // BHB and BHCHB have the same bigram counts but different numbers of 'H' char
    // BH:1 HB:1
    // BH:1 HC:1 HB:1
    let (polymer, rules) = input();
    let rrules = rules.iter().map(|(k, v)| {
        let left = [&(*k)[0..1], *v].join("");
        let right = [*v, &(*k)[1..]].join("");
        (*k, vec![left, right])
    }).collect::<HashMap<&str, Vec<String>>>();

    let mut bigrams = HashMap::<&'static str, usize>::from_iter(rules.iter().map(|(k, _)| (*k, 0)));
    (0..polymer.len() - 1).map(|i| &polymer[i..=i + 1]).for_each(|big| { *bigrams.entry(big).or_insert(0) += 1; });

    (0..40).for_each(|_| {
        let curr_bigrams = bigrams.clone();
        curr_bigrams.iter().for_each(|(k, curr)| {
            let update_keys = rrules.get(k).unwrap();
            update_keys.iter().for_each(|k| { bigrams.entry(k).and_modify(|mut v| *v += *curr); });
        })
    });
    let char_cnt = bigrams.iter().fold(HashMap::new(), |mut acc, (k, v)| {
        let keys: String = bigrams.clone().keys().into_iter().map(|k| *k).collect::<Vec<_>>().join("");
        keys.
        println!("{}", bigrams.clone().keys().into_iter().map(|k| *k).collect::<Vec<_>>().join(""));
        println!("[{}]:{} {:?}", *k, *v, acc);
        acc.entry(&k[0..1]).and_modify(|mut c| *c += *v).or_insert(*v);
        acc.entry(&k[1..]).and_modify(|mut c| *c += *v).or_insert(*v);
        acc
    });
    let mut cnt = char_cnt.values().collect::<Vec<_>>();
    cnt.sort();

    println!("p2: {:?}", char_cnt)
}

fn main() {
    p1();
    p2();
}