use std::path::PathBuf;

fn main() {
    let path1 = PathBuf::from("src/main.rs");

    let mut path2 = PathBuf::new();
    path2.push("src");
    path2.push("main.rs");

    assert_eq!(path1, path2);
    println!("Paths are equal: {:?}", path1);

    let path = PathBuf::from("/home/user/documents/file.txt");
    println!("File name: {:?}", path.file_name());
    println!("Extension: {:?}", path.extension());
    println!("Parent: {:?}", path.parent());

    let mut new_path = PathBuf::from("/home/user");
    new_path.push("documents");
    new_path.push("file.txt");
    println!("Joined path: {:?}", new_path);
}
