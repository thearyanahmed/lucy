use lucy::drivers::redis_store::RedisStore;
use lucy::{record::Record, Lucy};
use lucy::datastore::DatastoreDriver;
use fakes_gen::faker::Faker;
use fakes_gen::faker::fake_options::FakeOption;

fn main() {
    println!("running example");
    
    let mut lucy = Lucy::new(DatastoreDriver::InMemoryHashmap);

    let mut faker = Faker::default();

    for _ in 0..3 {
        match lucy.record(Record::new(faker.gen(&FakeOption::URL))) {
            Err(err) => println!("error: {}",err),
            _ => {},
        }
    }

    for r in lucy.all() {
        println!("uuid: {} url:{}",r.uuid, r.url);
    }

    let mut c = RedisStore::new();
    let res = c.fetch_an_integer();

    println!("redis result: {:?}", res)
}