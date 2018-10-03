extern crate gio;
extern crate libostree;

use std::io::Write;
use std::fs::File;

use gio::prelude::*;
use libostree::prelude::*;

fn main() {
    let repo = libostree::Repo::new_for_str("../../../repo-bare");

    //let result = repo.create(libostree::RepoMode::Archive, Option::None);
    //result.expect("we did not expect this to fail :O");

    repo.open(None).expect("should have opened");

    let (file, checksum) = repo.read_commit("test", None).unwrap();

    println!("path: {:?}", file.get_path());

    println!("sha256: {}", checksum);

    let objs = repo.traverse_commit(checksum.as_str(), -1, None).unwrap();

    for obj in objs {
        //let (name, obj_type) = libostree::object_name_deserialize(&obj);
        println!("  {}", obj.name());

        let (stream, size) = repo.load_object_stream(obj.object_type(), obj.checksum(), None).unwrap();
        println!("  bytes: {}", size);

        let mut file = File::create(obj.name()).unwrap();
        let mut read = 1;
        let mut buffer = [0 as u8; 4096];
        while read != 0 {
            read = stream.read(buffer.as_mut(), None).unwrap();
            file.write(&buffer[0 .. read]).unwrap();
        }
    }
}
