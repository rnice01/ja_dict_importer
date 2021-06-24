mod dictionaries;
use dictionaries::importer::DictImporter;
use dictionaries::yomican_importer::YomichanImporter;
use std::{fs, env, path::Path};

fn main() {
  let cwd = env::current_dir().unwrap();
  let path = cwd.join(Path::new("src/dictionaries/data/yomichan_jdict"));
  let importer = YomichanImporter{};

  match fs::read_dir(path) {
    Ok(dir) => for entry in dir {
      let entry = entry.unwrap();
      match fs::File::open(entry.path()) {
        Ok(file) => importer.import(file).unwrap(),
        Err(why) => panic!("{}", why)
      }
    },
    Err(why) => panic!("{}", why)
  }
}
