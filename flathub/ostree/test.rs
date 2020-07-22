
use std::collections::HashMap;
use types::Repository;
use zvariant::Type;
use zvariant::Value;
use zvariant::{from_slice, EncodingContext};
use zvariant_derive::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct<'s> {
    field1: Vec<Struct2<'s>>,
    field2: Value<'s>,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct2<'s> {
    field1: String,
    field2: Struct3<'s>,
}
#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct3<'s> {
    field1: u64,
    field2: Vec<u8>,
    field3: HashMap<String, Value<'s>>
}

    /*let d =
        flatmanager::ostree::Variant::new(ostree::SUMMARY_GVARIANT_STRING.to_string(), b.to_vec())
            .unwrap();
    println!("{:#?}", d.as_string_vec())
    */
    let ctxt = EncodingContext::<LE>::new_dbus(0);
    let decoded: Struct = from_slice(&b.to_vec(), ctxt).unwrap();

    println!("{:#?}", decoded)

    '(a(s(taya{sv}))a{sv})\


    */
    // let fedora = "/var/lib/flatpak/appstream/fedora/x86_64/appstream.xml.gz";
    //let manifest: Collection = Collection::from_gzipped(fedora.into()).unwrap();
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