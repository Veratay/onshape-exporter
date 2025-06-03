use std::{collections::{HashMap, HashSet}, sync::Arc};

use futures::future::join_all;
use tokio::sync::{Mutex, Semaphore};
use onshape_rust::{
    apis::{assembly_api::get_features, configuration::Configuration},
    models::{self, BtAssemblyDefinitionInfo, BtAssemblyOccurrenceInfo},
};

use crate::entity::EntityID;

#[derive(Debug)]
pub struct AssemblyData {
    pub occurrences: Option<Vec<BtAssemblyOccurrenceInfo>>,
    pub id: EntityID,
    pub parts: Vec<AssemblyPart>,
    pub features: Vec<Feature>,
    pub subassemblies: Vec<AssemblyNode>,
}

#[derive(Debug)]
pub struct AssemblyPart {
    pub studio: EntityID,
    pub id: String,
}

#[derive(Debug)]
pub struct AssemblyNode {
    pub id: String,
    pub e_id: EntityID,
}

pub type Occurrence = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub mate_type: String,
    pub _0: Occurrence,
    pub _1: Occurrence,
}

impl AssemblyData {
    async fn new(configuration: &Configuration, id: EntityID, include_occurrences: bool) -> Option<Self> {
        let root = id
            .get_assembly(configuration)
            .await
            .unwrap()
            .root_assembly?;
        let id = EntityID::from_root_assembly(&root)?;

        let parts: Vec<_> = root
            .instances
            .as_deref()?
            .iter()
            .filter_map(|it| {
                Some(AssemblyPart {
                    studio: EntityID::from_instance(it)?,
                    id: it.part_id.as_deref()?.to_owned(),
                })
            })
            .collect();

        let features = root
            .features?
            .iter()
            .filter_map(|it| {
                Some(Feature {
                    mate_type: it.feature_data.as_deref()?.mate_type.as_deref()?.to_owned(),
                    _0: it.feature_data.as_deref()?.mated_entities.as_deref()?[0].mated_occurrence.as_deref()?.to_owned(),
                    _1: it.feature_data.as_deref()?.mated_entities.as_deref()?[1].mated_occurrence.as_deref()?.to_owned(),
                })
            })
            .collect();

        let subassemblies = root
            .instances
            .as_deref()?
            .iter()
            .filter(|it| it.r#type == Some(models::BtAssemblyInstanceType::Assembly))
            .filter_map(|it| {
                Some(AssemblyNode {
                    id: it.id.as_deref()?.to_owned(),
                    e_id: EntityID::from_instance(it)?,
                })
            })
            .collect();

        let occurrences = if include_occurrences {
            root.occurrences
        } else {
            None
        };

        Some(AssemblyData {
            occurrences,
            id,
            parts,
            features,
            subassemblies,
        })
    }

    pub fn part_groups(
        &self,
        assemblies_data: &HashMap<EntityID, AssemblyData>,
        path: &mut Occurrence,
    ) -> Vec<HashSet<Occurrence>> {
        let mut groups = vec![];

        for it in self.subassemblies.iter() {
            path.push(it.id.clone());
            groups.append(&mut assemblies_data[&it.e_id].part_groups(assemblies_data, path));
            path.pop();
        }

        groups
    }
}

pub async fn load_assemblies(configuration: Configuration, root_assembly: EntityID, n_concurrent_connections: usize) -> HashMap<EntityID, AssemblyData> {
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let semaphore = Arc::new(Semaphore::new(n_concurrent_connections));
    let results = Arc::new(Mutex::new(HashMap::new()));

    recurse_load_assembly_parallel(configuration, root_assembly, seen, semaphore, Arc::clone(&results), true).await;

    Arc::try_unwrap(results).unwrap().into_inner()
}

async fn recurse_load_assembly_parallel(
    configuration: Configuration,
    assembly: EntityID,
    seen: Arc<Mutex<HashSet<EntityID>>>,
    semaphore: Arc<Semaphore>,
    results: Arc<Mutex<HashMap<EntityID, AssemblyData>>>,
    include_occurrences: bool
) {
    // limit number of concurrent requests
    let data;
    {
        let _permit = semaphore.acquire().await.unwrap();
        data = AssemblyData::new(&configuration,assembly.clone(),include_occurrences).await.unwrap();
    }

    let subassemblies: Vec<_>;

    // avoid duplicate requests
    {
        let mut seen_guard = seen.lock().await;
        subassemblies = data
            .subassemblies
            .iter()
            .filter_map(|it| {
                if seen_guard.insert(it.e_id.clone()) {
                    Some(it.e_id.clone())
                } else {
                    None
                }
            })
            .collect();
    }

    // collect data
    {
        results.lock().await.insert(assembly, data);
    }

    //recurse subassemblies
    join_all(subassemblies.into_iter().map(|it| {
        // these need to be cloned so they can be moved into async closure.
        let configuration = configuration.clone();
        let seen = Arc::clone(&seen);
        let semaphore = Arc::clone(&semaphore);
        let results = Arc::clone(&results);
        async move { recurse_load_assembly_parallel(configuration, it, seen, semaphore, results, false).await }
    }))
    .await;
}
