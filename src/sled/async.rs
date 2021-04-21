fn main() {
    let sled = sled::open("my_db").unwrap();

    let mut sub = sled.watch_prefix("");

    sled.insert(b"a", b"a").unwrap();

    extreme::run(async move {
        while let Some(event) = (&mut sub).await {
            println!("got event {:?}", event);
        }
    });
}
