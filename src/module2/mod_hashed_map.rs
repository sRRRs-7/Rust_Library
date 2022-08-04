use std::collections::HashMap;

#[derive(Debug)]
enum Company {
    Linux(String),
    Mac(String),
    Windows(String),
    Ios(String),
    Android(String),
}

#[derive(Debug)]
struct ListOS {
    linux: String,
    mac: String,
    windows: String,
    ios: String,
    android: String,
}

impl ListOS {
    fn new(linux: String, mac: String, windows: String, ios: String, android: String) -> Self {
        Self {
            linux,
            mac,
            windows,
            ios,
            android,
        }
    }
}

pub fn main() {
    let os = ListOS::new(
        String::from("linux"),
        String::from("mac"),
        String::from("windows"),
        String::from("iOS"),
        String::from("android"),
    );

    // key value list
    let mut key_value = HashMap::new();
    key_value.insert(&os.linux, Company::Linux(String::from("open source")));
    key_value.insert(&os.mac, Company::Mac(String::from("apple")));
    key_value.insert(&os.windows, Company::Windows(String::from("microsoft")));

    // get
    let list1 = key_value.get(&os.linux).unwrap();
    println!("{:?}", list1);

    // for
    for (k, v) in &key_value {
        println!("{:?} {:?}", k, v);
    }
    // entry & or_insert
    key_value
        .entry(&os.ios)
        .or_insert(Company::Ios(String::from("ios")));
    key_value.insert(&os.android, Company::Android(String::from("android")));
    println!("{:?}", key_value);

    // zip
    let key_arr = vec!["google", "apple", "facebook"];
    let value_arr = vec!["golang", "swift", "graphql"];

    let lang: HashMap<_, _> = key_arr.iter().zip(value_arr.iter()).collect();
    for (key, val) in lang {
        println!("{}:{}", key, val);
    }

    // methods
    {
        let mut list = HashMap::new();
        list.insert(String::from("1"), String::from("a"));
        list.insert(String::from("2"), String::from("b"));
        let key1 = String::from("1");
        let v = list.get(&key1).unwrap();
        println!("{:?}", v);
        println!("{:?}", list);

        for val in list.values() {
            println!("{}", val);
        }

        for (index, value) in list.iter() {
            println!("{} {}", index, value);
        }

        println!("{}", list.len());

        let key_value = String::from("1");
        assert!(list.contains_key(&key_value));

        list.remove(&key_value).unwrap();
        println!("{:?}", list);
    }
}
