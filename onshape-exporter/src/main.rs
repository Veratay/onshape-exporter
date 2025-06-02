use clap::Parser;
use assembly::load_assemblies;
use graph::PartGroupGraph;
use onshape_rust::apis::configuration::Configuration;
use url::Url;

mod assembly;
mod entity;
mod graph;

use entity::EntityID;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Onshape API Access key
    #[arg(long)]
    access_key: String,

    #[arg(long)]
    secret_key: String,

    /// Onshape Document Link
    link: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut configuration = Configuration::new();
    configuration.basic_auth = Some((cli.access_key, Some(cli.secret_key)));

    let url = Url::parse(&cli.link).unwrap_or_else(|_| {
        eprintln!("Error: invalid document link");
        std::process::exit(1);
    });

    let s: Vec<_> = url.path_segments().unwrap().collect();

    let root_assembly = EntityID::new(
        s[1].to_owned(),
        s[2].to_owned(),
        s[3].to_owned(),
        s[5].to_owned(),
    );
    
    let assemblies = load_assemblies(configuration, root_assembly.clone(), 8).await;

    println!("{:#?}",assemblies);

    let part_groups = PartGroupGraph::new(root_assembly, &assemblies);

    println!("{:#?}",part_groups)
}
