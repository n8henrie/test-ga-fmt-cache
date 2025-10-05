use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Foo {
    val: u32,
}

fn main() {
    let myfoo: Foo = serde_json::from_str(r#"{ "val": 42 }"#).expect("couldn't deserialize");
    println!("{}", myfoo.val);
}
