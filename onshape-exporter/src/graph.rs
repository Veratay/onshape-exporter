use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    assembly::{AssemblyData, Feature, Occurrence},
    entity::EntityID,
};

#[derive(Debug)]
struct PartGroup {
    edges: Vec<ActiveMate>,
    parts: HashSet<Occurrence>,
}

#[derive(Debug)]
struct ActiveMate {
    feature: Feature,
    other_part: usize,
}

impl PartGroup {
    fn new(parts: HashSet<Occurrence>) -> Self {
        Self {
            edges: Vec::new(),
            parts,
        }
    }

    fn prepend_prefix(&self, prefix: &Occurrence) -> HashSet<Occurrence> {
        let mut result = HashSet::with_capacity(self.parts.len());

        for occurrence in self.parts.iter() {
            let mut new_occurrence = Vec::with_capacity(prefix.len() + occurrence.len());
            new_occurrence.extend_from_slice(prefix);
            new_occurrence.extend_from_slice(occurrence);
            result.insert(new_occurrence);
        }

        result
    }

    fn contains(&self, occurrence: &Occurrence) -> bool {
        self.parts.contains(occurrence)
    }

    fn insert(&mut self, occurrence: Occurrence) -> bool {
        self.parts.insert(occurrence)
    }
}

#[derive(Debug)]
pub struct PartGroupGraph {
    next_id: usize,
    groups: HashMap<usize, PartGroup>,
}

impl PartGroupGraph {
    pub fn new(root: EntityID, assemblies: &HashMap<EntityID, AssemblyData>) -> Self {
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

        let mut res = Self {
            next_id: 0,
            groups: HashMap::new(),
        };

        res.recurse(
            &assemblies[&root],
            &mut vec![],
            assemblies,
            &mut memoized_part_groups,
            &memoization_opportunities,
        );

        res
    }

    fn merge_groups(&mut self, g0: usize, g1: usize) {
        let src = self.groups.remove(&g1).unwrap();

        for e in src.edges.iter() {
            self.groups
                .get_mut(&e.other_part)
                .unwrap()
                .edges
                .iter_mut()
                .find(|it| it.feature == e.feature)
                .unwrap()
                .other_part = g0;
        }

        let dst = self.groups.get_mut(&g0).unwrap();

        dst.parts.extend(src.parts);
        dst.edges.extend(src.edges);
    }

    fn new_group<const N: usize>(&mut self, elements: [Occurrence; N]) -> usize {
        let mut new = HashSet::new();
        for e in elements {
            new.insert(e);
        }
        self.groups.insert(self.next_id, PartGroup::new(new));
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn recurse(
        &mut self,
        assembly: &AssemblyData,
        path: &mut Occurrence,
        assemblies: &HashMap<EntityID, AssemblyData>,
        memoized_part_groups: &mut HashMap<EntityID, Vec<HashSet<Vec<String>>>>,
        memoization_opportunities: &HashMap<EntityID, usize>,
    ) {
        for s in assembly.subassemblies.iter() {
            path.push(s.id.clone());
            self.recurse(
                &assemblies[&s.e_id],
                path,
                assemblies,
                memoized_part_groups,
                memoization_opportunities,
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

        let mut bound_to_origin = Vec::new();

        for f in assembly.features.iter() {
            // record parts bound to origin TODO: handle fixed parts.
            match (f._0.is_empty(), f._1.is_empty()) {
                (true, false) => {
                    bound_to_origin.push(f._1.clone());
                }
                (false, true) => {
                    bound_to_origin.push(f._0.clone());
                }
                _ => {}
            }

            let mut path0 = path.clone();
            let mut path1 = path.clone();
            path0.extend_from_slice(&f._0[0..]);
            path1.extend_from_slice(&f._1[0..]);

            self.handle_mate(path0, path1, Some(f));

            println!("Feature: {:#?}", f);
            println!("----------------");
            println!("{:#?}", self.groups);
        }

        // if there are multiple parts fixed to the origin they are effectively fixed to eachother.
        for parts in bound_to_origin.windows(2) {
            self.handle_mate(parts[0].clone(), parts[1].clone(), None)
        }

        // memoize if needed.
        // if memoization_opportunities
        //     .get(&assembly.id)
        //     .is_some_and(|it| *it > 1)
        // {
        //     let mut memoized_part_group = HashMap::new();
        //
        //     for g in part_groups.iter() {
        //         let mut new_group = HashSet::with_capacity(g.len());
        //
        //         for occurrence in g.iter().filter(|it| it.contains(path.last().unwrap())) {
        //             let mut new_occurrence = Vec::with_capacity(occurrence.len() - path.len() + 1);
        //             new_occurrence.extend_from_slice(&occurrence[path.len() - 1..]);
        //             new_group.insert(new_occurrence);
        //         }
        //
        //         memoized_part_group.push(new_group);
        //     }
        //
        //     memoized_part_groups.insert(assembly.id.clone(), memoized_part_group);
        // }
    }

    fn handle_mate(&mut self, path0: Occurrence, path1: Occurrence, feature: Option<&Feature>) {
        let i0 = self
            .groups
            .iter()
            .find_map(|(i, it)| if it.contains(&path0) { Some(i) } else { None })
            .cloned();
        let i1 = self
            .groups
            .iter()
            .find_map(|(i, it)| if it.contains(&path1) { Some(i) } else { None })
            .cloned();

        if feature.is_none() || feature.unwrap().mate_type == "FASTENED" {
            match (i0, i1) {
                (None, None) => {
                    self.new_group([path0, path1]);
                }
                (None, Some(group)) => {
                    self.groups.get_mut(&group).unwrap().insert(path0);
                }
                (Some(group), None) => {
                    self.groups.get_mut(&group).unwrap().insert(path1);
                }
                (Some(group0), Some(group1)) => {
                    if group0 != group1 {
                        self.merge_groups(group0, group1);
                    }
                }
            }
        } else {
            let i0 = i0.unwrap_or_else(|| self.new_group([path0]));
            let i1 = i1.unwrap_or_else(|| self.new_group([path1]));

            self.groups.get_mut(&i0).unwrap().edges.push(ActiveMate {
                feature: feature.unwrap().clone(),
                other_part: i1,
            });
            self.groups.get_mut(&i1).unwrap().edges.push(ActiveMate {
                feature: feature.unwrap().clone(),
                other_part: i0,
            });
        }
    }
}
