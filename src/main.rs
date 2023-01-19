// use lucy::{drivers::hashmap_store::HashmapStore, datastore::Datastore, Record};
use lucy::{record::Record};

fn main() {
    // let hmstore = HashmapStore::new();

    // let uuid = "random-uuid";

    // match hmstore.find(uuid) {
    //     Ok(found) => println!("found : {}",found),
    //     Err(err) => println!("got error: {}",err)
    // }

    let url_string = "https://github.com/thearyanahmed";

    match Record::from(url_string.to_string(), "asd".to_string()) {
        Ok(_url) => print!("valid url"),
        Err(err) => println!("err: {}",err)
    }
}