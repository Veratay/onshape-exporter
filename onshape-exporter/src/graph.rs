use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    mem::swap,
};

use itertools::Itertools;

use crate::{
    assembly::{AssemblyData, Occurrence},
    entity::EntityID,
};

pub fn part_groups(root: EntityID, assemblies: &HashMap<EntityID, AssemblyData>) -> Vec<HashSet<Occurrence>> {
    // we only want to memoize sub-trees that occur "uniquely" multiple times (i.e, if a sub-tree
    // occurs many times but only as part of the same larger sub tree, and it only occurs once in
    // the larger sub-tree, then we shouldnt memoize the smaller sub-tree, and instead memoize the
    // larger tree.)
    let mut memoization_opportunities = HashMap::new();

    // for us to be able to memoize a subassembly, it must either have multiple parent assemblies,
    // or occur multiple times in the same parent assembly.
    for a in assemblies.values() {
        for (id, c) in a.subassemblies.iter().counts_by(|it| it.e_id.clone()) {
            memoization_opportunities
                .entry(id)
                .and_modify(|it| *it += c)
                .or_insert(c);
        }
    }

    // dfs building the part groups, memoizing as we go
    let mut memoized_part_groups: HashMap<EntityID, Vec<HashSet<Occurrence>>> = HashMap::new();
    let mut part_groups: Vec<HashSet<Occurrence>> = Vec::new();

    fn recurse(
        assembly: &AssemblyData,
        path: &mut Occurrence,
        assemblies: &HashMap<EntityID, AssemblyData>,
        part_groups: &mut Vec<HashSet<Occurrence>>,
        memoized_part_groups: &mut HashMap<EntityID, Vec<HashSet<Vec<String>>>>,
        memoization_opportunities: &HashMap<EntityID, usize>
    ) {
        for s in assembly.subassemblies.iter() {
            path.push(s.id.clone());
            recurse(
                &assemblies[&s.e_id],
                path,
                assemblies,
                part_groups,
                memoized_part_groups,
                memoization_opportunities
            );
            path.pop();
        }

        // use memoization when available
        // if let Some(groups) = memoized_part_groups.get(&assembly.id) {
        //     // need to temp remove current assembly id from path because it is already in memoized
        //     // groups.
        //     let id = path.pop();
        //     part_groups.extend(groups.iter().map(|it| prepend_prefix(it, path)));
        //     if let Some(id) = id {
        //         path.push(id);
        //     }
        //
        //     return;
        // }

        for f in assembly.features.iter() {
            let mut path0 = path.clone();
            let mut path1 = path.clone();
            path0.extend_from_slice(&f._0[0..]);
            path1.extend_from_slice(&f._1[0..]);

            match (
                part_groups.iter().position(|it| it.contains(&path0)),
                part_groups.iter().position(|it| it.contains(&path1)),
            ) {
                (None, None) => {
                    let mut new = HashSet::new();
                    new.insert(path0);
                    part_groups.push(new);

                    let mut new = HashSet::new();
                    new.insert(path1);
                    part_groups.push(new);
                }
                (None, Some(group)) => {
                    part_groups[group].insert(path0);
                }
                (Some(group), None) => {
                    part_groups[group].insert(path1);
                }
                (Some(mut group0), Some(mut group1)) => {
                    if group0 == group1 {
                        break;
                    }

                    if group1 < group0 {
                        swap(&mut group0, &mut group1);
                    }

                    let (left, right) = part_groups.split_at_mut(group1);
                    let to_merge = right[0].drain();
                    left[group0].extend(to_merge);

                    part_groups.remove(group1);
                }
            }
        }

        // memoize if needed.
        if memoization_opportunities.get(&assembly.id).is_some_and(|it| *it>1) {
            let mut memoized_part_group = Vec::with_capacity(part_groups.len());

            for g in part_groups.iter() {
                let mut new_group = HashSet::with_capacity(g.len());

                for occurrence in g.iter().filter(|it| it.contains(path.last().unwrap())) {
                    let mut new_occurrence = Vec::with_capacity(occurrence.len() - path.len() + 1);
                    new_occurrence.extend_from_slice(&occurrence[path.len()-1..]);
                    new_group.insert(new_occurrence);
                }

                memoized_part_group.push(new_group);
            }

            memoized_part_groups.insert(assembly.id.clone(), memoized_part_group);
        }
    }

    recurse(
        &assemblies[&root],
        &mut vec![],
        assemblies,
        &mut part_groups,
        &mut memoized_part_groups,
        &memoization_opportunities
    );

    part_groups
}

fn prepend_prefix<T>(group: &HashSet<Vec<T>>, prefix: &Vec<T>) -> HashSet<Vec<T>>
where
    T: Clone + Eq + Hash,
{
    let mut result = HashSet::with_capacity(group.len());

    for occurrence in group {
        let mut new_occurrence = Vec::with_capacity(prefix.len() + occurrence.len());
        new_occurrence.extend_from_slice(prefix);
        new_occurrence.extend_from_slice(occurrence);
        result.insert(new_occurrence);
    }

    result
}
