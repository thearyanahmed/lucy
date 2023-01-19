use redis::Commands;

pub struct RedisStore {
    con: redis::Connection,
}

impl RedisStore {
    pub fn new() -> RedisStore {
        // @TODO take connection string as parameter.
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not connect to redis");

        let con = client
            .get_connection()
            .expect("could not get connection to redis");

        RedisStore { con }
    }

    pub fn fetch_an_integer(&mut self) -> redis::RedisResult<isize> {
        println!("this is a test method");

        self.con.set("my_key_2", 421)?;
        self.con.get("my_key_2")
    }
}
