use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Entry {
    File(String),
    Dir(String, Vec<Entry>)
}

fn crawl(path: &Path) -> io::Result<Vec<Entry>>{
    let mut list = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        let name = item.file_name().to_string_lossy().into_owned();

        if item.path().is_dir() {
            let children = crawl(&item.path())?;
            list.push(Entry::Dir(name, children));
        } else {
            list.push(Entry::File(name));
        }
    }

    list.sort_by(|a, b| {
        let (Entry::Dir(n1, _) | Entry::File(n1)) = a;
        let (Entry::Dir(n2, _) | Entry::File(n2)) = b;
        n1.cmp(n2)
    });
    Ok(list)
}

fn main() -> io::Result<()>{
    let path = ".";
    let tree = crawl(Path::new(path))?;
    println!("{:#?}", tree);
    Ok(())
}