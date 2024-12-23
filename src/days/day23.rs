use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::solution::Solution;

pub struct Day23;

impl Solution for Day23 {
    type Answer = String;
    fn day(&self) -> u8 {
        23
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let network = Network::try_from(input)?;
        let triangles = network.find_triangles();
        let triangles_with_starting_with_t = triangles
            .iter()
            .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            .count();

        Ok((triangles_with_starting_with_t / 6).to_string())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let network = Network::try_from(input)?;
        let largest_clique = network.find_largest_clique();
        Ok(clique_to_sorted_string(&largest_clique))
    }
}

struct Network<'a> {
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Network<'a> {
    fn find_triangles(&self) -> Vec<(&'a str, &'a str, &'a str)> {
        let mut triangles = Vec::new();
        for (a, b_edges) in &self.edges {
            for b in b_edges {
                if let Some(c_edges) = self.edges.get(b) {
                    for c in c_edges {
                        if self.edges.get(c).unwrap().contains(a) {
                            triangles.push((*a, *b, *c));
                        }
                    }
                }
            }
        }
        triangles
    }

    fn find_largest_clique(&self) -> HashSet<&'a str> {
        fn bron_kerbosch<'b>(
            current: HashSet<&'b str>,
            mut candidates: HashSet<&'b str>,
            mut excluded: HashSet<&'b str>,
            network: &Network<'b>,
        ) -> HashSet<&'b str> {
            if candidates.is_empty() && excluded.is_empty() {
                return current;
            }

            let mut largest_clique = current.clone();
            for &v in &candidates.clone() {
                let mut new_current = current.clone();
                new_current.insert(v);
                let new_candidates = candidates
                    .intersection(&network.edges[v])
                    .copied()
                    .collect();
                let new_excluded = excluded.intersection(&network.edges[v]).copied().collect();
                let new_clique = bron_kerbosch(new_current, new_candidates, new_excluded, network);
                if new_clique.len() > largest_clique.len() {
                    largest_clique = new_clique;
                }
                candidates.remove(v);
                excluded.insert(v);
            }
            largest_clique
        }

        let vertices = self.edges.keys().copied().collect();
        bron_kerbosch(HashSet::new(), vertices, HashSet::new(), self)
    }
}

impl<'a> TryFrom<&'a str> for Network<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> std::result::Result<Self, Self::Error> {
        let lines = s.lines().filter_map(|line| line.split_once('-'));
        let mut edges = HashMap::new();
        for (from, to) in lines {
            edges.entry(from).or_insert_with(HashSet::new).insert(to);
            edges.entry(to).or_insert_with(HashSet::new).insert(from);
        }

        Ok(Network { edges })
    }
}

fn clique_to_sorted_string(clique: &HashSet<&str>) -> String {
    let mut sorted_clique: Vec<_> = clique.iter().copied().collect();
    sorted_clique.sort();
    sorted_clique.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(Day23.run_test1(), "7");
    }

    #[test]
    fn part2_example() {
        assert_eq!(Day23.run_test2(), "co,de,ka,ta");
    }
}
