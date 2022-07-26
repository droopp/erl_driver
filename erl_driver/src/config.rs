
pub struct Config {
    pub timeout: u64
}

impl Config {

    pub fn new(params: String) -> Self{

        let mut cfg = Config{timeout: 0};

        for kv in params.split(" "){

            let val: Vec<&str> = kv.split("=").collect();
            let (k, v) = (val[0], val[1]);

            match k {
                "timeout" => cfg.timeout = v.parse::<u64>().unwrap(),
                _ => ()
            };

        };

        cfg

    }
}
