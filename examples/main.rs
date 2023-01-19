use lucy::drivers::redis_store::RedisStore;
use lucy::{record::Record, Lucy};
use lucy::datastore::{DatastoreDriver, Datastore};
use fakes_gen::faker::Faker;
use fakes_gen::faker::fake_options::FakeOption;

fn main() {
    println!("running example");
    
    let mut lucy = Lucy::new(DatastoreDriver::InMemoryHashmap);

    let mut faker = Faker::default();

    for _ in 0..1 {
        match lucy.record(Record::new(faker.gen(&FakeOption::URL))) {
            Err(err) => println!("error: {}",err),
            _ => {},
        }
    }

    for r in lucy.all() {
        println!("uuid: {} url:{}",r.uuid, r.url);
    }

    let mut c = RedisStore::new();
    let fake_record = Record::new(faker.gen(&FakeOption::URL));
    let fake_uuid = fake_record.uuid.to_string();
    let fake_url = fake_record.url.to_string();

    match c.record(fake_record) {
        Ok(_) => println!("recored {} into redis : {}",&fake_uuid, &fake_url),
        Err(e) => println!("{}",e)
    }

    let x = vec![fake_uuid, "hello".to_string()];

    for y in x {
        match c.find(&y) {
            Ok(r) => {
                println!("FOUND URL: {}",r.url)
            },
            Err(e) => print!("NOT FOUND {}",e.to_string()),
        }
    }
}