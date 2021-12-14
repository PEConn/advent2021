use std::collections::{HashMap, HashSet};

type Polymer = Vec<char>;
type InsertionRules = HashMap<(char, char), char>;

fn parse(input: &str) -> (Polymer, InsertionRules) {
    let mut lines = input.lines();

    let template : Polymer = lines.next().unwrap().chars().collect();

    let mut insertion_rules : InsertionRules = HashMap::new();

    for line in lines.skip(1) {
        let mut characters = line.chars();
        let a = characters.next().unwrap();
        let b = characters.next().unwrap();

        let mut characters = characters.skip(4);

        let c = characters.next().unwrap();
        insertion_rules.insert((a, b), c);
    }

    (template, insertion_rules)
}

fn step(polymer: &Polymer, rules: &InsertionRules) -> Polymer {
    let mut new_polymer : Polymer = Vec::new();
    let mut last_element = *polymer.first().unwrap();
    new_polymer.push(last_element);

    for next_element in polymer.iter().skip(1) {
        let new_element = rules.get(&(last_element, *next_element)).unwrap();
        new_polymer.push(*new_element);
        new_polymer.push(*next_element);

        last_element = *next_element;
    }

    new_polymer
}

fn steps(polymer: &Polymer, rule: &InsertionRules, steps: i32) -> Polymer {
    let mut new_polymer = step(polymer, rule);
    for _ in 1..steps {
        new_polymer = step(&new_polymer, rule);
    }
    new_polymer
}

fn diff_between_most_and_least_common(polymer: &Polymer) -> usize {
    let all_characters : HashSet<char> = polymer.iter().cloned().collect();
    let mut quantities : Vec<usize> = all_characters.iter()
        .map(|a| polymer.iter().filter(|b| a == *b).count())
        .collect();

    quantities.sort();

    quantities.last().unwrap() - quantities.first().unwrap()
}

pub fn part1(input: &str) -> usize {
    let (polymer, rules) = parse(input);
    let polymer = steps(&polymer, &rules, 10);

    diff_between_most_and_least_common(&polymer)
}

fn insert(a: &mut HashMap<char, i64>, b: &HashMap<char, i64>) {
    //// Inserts all the entries from map `b` into map `a`. If an entry exists in both maps, put the
    //// sum of both entries in `a`.
    for (key, value) in b {
        *a.entry(*key).or_insert(0) += value;
    }
}

fn calculate_frequencies(a: char, b: char, rules: &InsertionRules, depth: i32,
        cache: &mut HashMap<(char, char, i32), HashMap<char, i64>>)
                         -> HashMap<char, i64> {
    //// Returns a map of the frequencies of character present in a string *between* characters `a`
    //// and `b` when expanded to the given depth.
    //// The `cache` HashMap is keyed on the triple (`a`, `b`, `depth`) and will be used to not
    //// repeat calculations that have already taken place.

    // Base case - we don't need to go any further.
    if depth == 0 {
        return HashMap::new();
    }

    // Check whether we've already calculated this.
    let cache_key = (a, b, depth);
    if let Some(frequencies) = cache.get(&cache_key) {
        return frequencies.clone();
    }

    // Find the new character.
    let c = *rules.get(&(a, b)).unwrap();

    // Add all the characters to the left of the new character.
    let mut frequencies =
        calculate_frequencies(a, c, rules, depth - 1, cache);
    // Add all the characters to the right of the new character.
    insert(&mut frequencies,
           &calculate_frequencies(c, b, rules, depth - 1, cache));
    // Add the new character.
    *frequencies.entry(c).or_insert(0) += 1;

    // Add the result to the cache.
    cache.insert(cache_key, frequencies.clone());

    frequencies
}

pub fn part2(input: &str) -> i64 {
    let (polymer, rules) = parse(input);
    let steps = 40;

    let mut frequencies: HashMap<char, i64> = HashMap::new();
    let mut cache: HashMap<(char, char, i32), HashMap<char, i64>> = HashMap::new();

    // Insert the characters in the initial polymer.
    for c in polymer.iter() {
        *frequencies.entry(*c).or_insert(0) += 1;
    }

    // Insert the characters in between.
    for (a, b) in polymer.iter().zip(polymer.iter().skip(1)) {
        insert(&mut frequencies,
               &calculate_frequencies(*a, *b, &rules, steps, &mut cache))
    }

    let max = frequencies.values().max().unwrap();
    let min = frequencies.values().min().unwrap();

    max - min
}

#[cfg(test)]
mod test{
    use super::*;

    const INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn basic() {
        let (polymer, rules) = parse(INPUT);
        assert_eq!(4, polymer.len());

        let polymer = step(&polymer, &rules);
        assert_eq!(7, polymer.len()); // After step 1.

        let polymer = steps(&polymer, &rules, 4);
        assert_eq!(97, polymer.len()); // After step 5.

        let polymer = steps(&polymer, &rules, 5);
        assert_eq!(3073, polymer.len()); // After step 10.

        assert_eq!(1749, polymer.iter().filter(|c| **c == 'B').count());
        assert_eq!(298, polymer.iter().filter(|c| **c == 'C').count());
        assert_eq!(161, polymer.iter().filter(|c| **c == 'H').count());
        assert_eq!(865, polymer.iter().filter(|c| **c == 'N').count());
    }

    #[test]
    fn given_example_part1() {
        assert_eq!(1588, part1(INPUT));
    }

    #[test]
    fn given_example_part2() {
        assert_eq!(2188189693529, part2(&INPUT));
    }
}