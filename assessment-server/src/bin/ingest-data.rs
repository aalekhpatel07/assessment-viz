use assessment_server::{Record, Features};
use clap::Parser;
use std::path::PathBuf;
use mongodb::{Client, options::{ClientOptions, ServerApi, ServerApiVersion, IndexOptions}, Database, IndexModel, results::InsertManyResult};
use mongodb::bson::doc;
use tracing::info;


#[derive(Parser)]
struct Opts {
    #[arg(help="The path to the geojson file.")]
    input: PathBuf,

    #[arg(short, long, help="Mongodb host", default_value_t = String::from("localhost"))]
    mongo_host: String,
    #[arg(short, long, help="Mongodb port", default_value_t = 27017)]
    mongo_port: u16,
    #[arg(short, long, help="", default_value_t = String::from("assessment"))]
    mongo_collection: String,
    #[arg(long, help="If specified, will dump the data to mongodb.", default_value_t = false)]
    ingest: bool
}

pub type IgnoreError<T = ()> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;


async fn connect_to_mongodb(host: &str, port: u16) -> IgnoreError<Client> {
    let uri = format!("mongodb://root:root@{host}:{port}");
    let mut client_options = ClientOptions::parse(uri).await?;
    
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let db = Client::with_options(client_options)?;
    Ok(db)
}


#[derive(Debug)]
pub struct AppState {
    pub db: Database,
    pub contents: Features
}


pub fn parse_geojson<P: AsRef<std::path::Path>>(file: P) -> IgnoreError<Features> {
    let contents = std::fs::read_to_string(file)?;
    info!("Read to string successful. String length: {}", contents.len());
    let features: Features = serde_json::from_str::<Features>(&contents)?;
    Ok(features)
}

pub async fn create_collection_if_not_exists(db: &Database, name: &str) -> IgnoreError {
    let all_collections =
    db
    .list_collection_names(None).await?;

    if !all_collections.contains(&name.to_string()) {
        db.create_collection(name, None).await?;
    }
    Ok(())
}

pub async fn create_index<T>(db: &Database, collection_name: &str) -> IgnoreError<()> {

    db
    .collection::<T>(collection_name)
    .create_index(
        IndexModel::builder()
        .keys(doc! { "$**" : 1 })
        .options(IndexOptions::builder().name(Some("$**_1".to_string())).build())
        .build(),
        None
    ).await?;

    Ok(())
}


pub async fn init_db(db: &Database, name: &str) -> IgnoreError {
    create_collection_if_not_exists(db, name).await?;
    create_index::<()>(db, name).await?;
    Ok(())
}

pub async fn load_data(db: &Database, name: &str, records: &[Record]) -> IgnoreError<InsertManyResult> {
    println!("Total records: {:#?}", records.len());
    Ok(
        db
        .collection::<Record>(name)
        .insert_many(
            records.into_iter(),
            None
        )
        .await?
    )
}


#[tokio::main]
async fn main() -> IgnoreError {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    let collection_name = opts.mongo_collection.clone();    
    let db_connection = tokio::task::spawn(async move {
        let client = connect_to_mongodb(&opts.mongo_host, opts.mongo_port).await.unwrap();
        let db = client.database(&collection_name);
        info!("Established a connection to Mongodb at: {:#?}", client.list_databases(None, None).await.unwrap());

        db
    });

    info!("Beginning to read geojson data from: {:?}", opts.input);
    let features = std::thread::spawn(move || {
        parse_geojson(opts.input).unwrap()
    });


    let db = db_connection.await?;
    let features = features.join().unwrap();

    info!("Finished loading geojson data in memory. Number of records: {:?}", features.len());

    let app_state = AppState {
        db,
        contents: features
    };

    if opts.ingest {
        init_db(&app_state.db, &opts.mongo_collection).await?;
        info!("Finished initializing db...");
        let Features::FeatureCollection(features) = &app_state.contents;
        load_data(&app_state.db, &opts.mongo_collection, &features.features).await?;
        info!("Finished loading data...");
    } else {
        info!("Not loading data because '--ingest' was not specified.")
    }

    Ok(())
}
