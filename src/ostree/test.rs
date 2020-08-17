
use std::collections::HashMap;
use types::Repository;
use zvariant::Type;
use zvariant::OwnedValue;
use zvariant::{from_slice, EncodingContext};
use zvariant_derive::Type;



#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct {
    field1: Vec<Struct2>,
    field2: HashMap<String, OwnedValue>,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct2 {
    field1: String,
    field2: Struct3,
}
#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct3 {
    field1: u64,
    field2: Vec<u8>,
    field3: HashMap<String, OwnedValue>
}
let file = gio::File::new_for_path("/home/bilelmoussaoui/.local/share/flatpak/repo/");
let repo = ostree::Repo::new(&file);
repo.open(gio::NONE_CANCELLABLE).unwrap();
/*println!(
    "{:#?}",
    repo.remote_list().iter().map(|s| s.to_string()).collect::<Vec<String>>()
);*/
let (b, b2) = repo
    .remote_fetch_summary("flathub", gio::NONE_CANCELLABLE)
    .unwrap();
let ctxt = EncodingContext::<LE>::new_dbus(0);
let decoded: Struct = from_slice(&b.to_vec(), ctxt).unwrap();

println!("{:#?}", decoded);
Ok(())