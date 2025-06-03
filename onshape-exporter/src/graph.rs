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

#[derive(Debug, Clone)]
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

    fn prepend_prefix(&self, prefix: &Occurrence) -> PartGroup {
        let mut parts = HashSet::with_capacity(self.parts.len());

        for occurrence in self.parts.iter() {
            let mut new_occurrence = Vec::with_capacity(prefix.len() + occurrence.len());
            new_occurrence.extend_from_slice(prefix);
            new_occurrence.extend_from_slice(occurrence);
            parts.insert(new_occurrence);
        }

        let mut res = PartGroup::new(parts);
        res.edges = self.edges.clone();

        res
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

        let mut memoized_part_groups = HashMap::new();

        let mut res = Self {
            next_id: 0,
            groups: HashMap::new(),
        };

        // dfs building the part groups, memoizing as we go
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

    fn insert(&mut self, group: PartGroup) -> usize {
        self.groups.insert(self.next_id, group);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn recurse(
        &mut self,
        assembly: &AssemblyData,
        path: &mut Occurrence,
        assemblies: &HashMap<EntityID, AssemblyData>,
        memoized_part_groups: &mut HashMap<EntityID, HashMap<usize, PartGroup>>,
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
        if let Some(groups) = memoized_part_groups.get(&assembly.id) {
            let mut id_map: HashMap<usize, usize> = HashMap::new();

            // copy groups, making note of new ids
            for (i, g) in groups.iter() {
                let new_g = g.prepend_prefix(path);

                println!("new_g: {:#?}", new_g);

                id_map.insert(*i, self.insert(new_g));
            }

            // update links to new ids
            for i in id_map.values() {
                for e in self.groups.get_mut(i).unwrap().edges.iter_mut() {
                    println!("Accessing: {}", e.other_part);
                    e.other_part = *id_map.get(&e.other_part).unwrap();
                }
            }

            return;
        }

        let will_memoize = memoization_opportunities
            .get(&assembly.id)
            .is_some_and(|it| *it > 1);

        // keeps track of what part groups are from this assembly for recording memo
        let mut assembly_part_groups = HashSet::new();

        let mut bound_to_origin = Vec::new();
        for f in assembly.features.iter() {
            // record parts bound to origin TODO: handle fixed parts.
            match (f._0.is_empty(), f._1.is_empty()) {
                (true, false) => {
                    bound_to_origin.push(f._1.clone());
                    continue;
                }
                (false, true) => {
                    bound_to_origin.push(f._0.clone());
                    continue;
                }
                _ => {}
            }

            let mut path0 = path.clone();
            let mut path1 = path.clone();
            path0.extend_from_slice(&f._0[0..]);
            path1.extend_from_slice(&f._1[0..]);

            let groups = self.handle_mate(path0, path1, Some(f));

            if will_memoize {
                assembly_part_groups.insert(groups[0]);
                assembly_part_groups.insert(groups[1]);
            }

            // println!("Feature: {:#?}", f);
            // println!("----------------");
            // println!("{:#?}", self.groups);
        }

        // if there are multiple parts fixed to the origin they are effectively fixed to eachother.
        for parts in bound_to_origin.windows(2) {
            let groups = self.handle_mate(parts[0].clone(), parts[1].clone(), None);

            if will_memoize {
                assembly_part_groups.insert(groups[0]);
                assembly_part_groups.insert(groups[1]);
            }
        }

        // record memo if needed.
        if will_memoize {
            let mut memoized = HashMap::new();

            for (i, g) in assembly_part_groups
                .into_iter()
                // some of the part groups that have been created during processing this assembly
                // will have already merged into others
                .filter_map(|i| self.groups.get(&i).map(|g| (i, g)))
            {
                let mut parts = HashSet::with_capacity(g.parts.len());

                for occurrence in g.parts.iter() {
                    // stripping the occurrence path to be relative to this assembly.
                    // if the path is ["TopAssembly","ThisAssembly","part"]
                    // the memoized path will be ["part"]
                    let mut new_occurrence = Vec::with_capacity(occurrence.len() - path.len());
                    new_occurrence.extend_from_slice(&occurrence[path.len()..]);
                    parts.insert(new_occurrence);
                }

                let mut new_group = PartGroup::new(parts);

                new_group.edges = g.edges.clone();

                memoized.insert(i, new_group);
            }

            memoized_part_groups.insert(assembly.id.clone(), memoized);
        }
    }

    fn handle_mate(
        &mut self,
        path0: Occurrence,
        path1: Occurrence,
        feature: Option<&Feature>,
    ) -> [usize; 2] {
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
                    let new = self.new_group([path0, path1]);
                    [new; 2]
                }
                (None, Some(group)) => {
                    self.groups.get_mut(&group).unwrap().insert(path0);
                    [group; 2]
                }
                (Some(group), None) => {
                    self.groups.get_mut(&group).unwrap().insert(path1);
                    [group; 2]
                }
                (Some(group0), Some(group1)) => {
                    if group0 != group1 {
                        self.merge_groups(group0, group1);
                    }
                    [group0; 2]
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

            [i0, i1]
        }
    }
}
