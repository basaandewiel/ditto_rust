use dittolive_ditto::{prelude::*};
//use dittolive_ditto::{identity, prelude::*};
use std::sync::mpsc::channel;
use std::{self, str::FromStr, sync::Arc};
use structopt::StructOpt;
use serde_json::json;

#[derive(StructOpt)]
struct Args {
    #[structopt(long, env = "APP_ID")]
    app_id: String,
//    #[structopt(long, env = "SHARED_TOKEN")]
//    shared_token: String,
//    #[structopt(long, env = "COLLECTION")]
//    collection: String,
}


fn main() -> Result<(), DittoError> {
    let args = Args::from_args();
    let (sender, receiver) = channel::<(Vec<BoxedDocument>, LiveQueryEvent)>();
    let event_handler = move |documents: Vec<BoxedDocument>, event: LiveQueryEvent| {
        sender.send((documents, event)).unwrap();
    };


    let ditto = Ditto::builder() //@@@why no ; after this line?
     // creates a `ditto_data` folder in the directory containing the executing process
    .with_root(Arc::new(PersistentRoot::from_current_exe()?))
    .with_identity(|ditto_root| {
        //let app_id = AppId::from_env("0219fe97-ccff-4b24-99d2-140f54ba9a60")?;
        let shared_token = "78fcfe34-ecc0-44e6-90ba-1e3ca15435d7".to_string();
        //let shared_token = std::env::var("0aa8abeb-c950-4d5d-91da-e0a8fc9aa90a").unwrap();
        let enable_cloud_sync = true;
        let custom_auth_url = None;

        let app_id = AppId::from_str(&args.app_id).unwrap();
        //let shared_token = args.shared_token;

        OnlinePlayground::new(
            ditto_root,
            app_id,
            shared_token,
            enable_cloud_sync,
            custom_auth_url,
        )
    })?
    .build()?;

    ditto.start_sync()?;
    println!("***ditto sync started\n");

    let store = ditto.store(); // Ditto must have a longer lifetime than all live queries
    
    let live_query = store
        .collection("tasks")?
        .find("isCompleted  == false")
        .subscribe();
    println!("***subscribed to not completed tasks");


    let collection = store.collection(&"tasks".to_string())?;
    //let collection = store.collection(&args.collection)?;

    let _lq = collection.find_all().observe_local(event_handler)?;
    let res = collection.upsert(json!({
        "hello": "bassie"
    }));

    println!("***Inserted document with id={}", res.unwrap());

let person = json!({
    "name": "Susan".to_string(),
    "age": 31,
});
let collection = store.collection(&"people".to_string())?;
let id = collection.upsert(person).unwrap();
println!("***inserted person");


    loop {
        let (documents, event) = receiver.recv().unwrap();

        println!("***We have event {:?}", event);
        for doc in documents {
            println!("\tDocument {:?}", doc.to_cbor());
        }
    }

    //Ok(()) //without ; so that this value is returned; and provide empty argument (), because
        //when result is OK, the no value should be returned

}
