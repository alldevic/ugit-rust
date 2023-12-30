fn write_tree(directory: String) -> io::Result<()> {
  let directory = directory.unwrap_or(".");

  for entry in fs::read_dir(directory)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_dir() {
      write_tree(&path, cb)?;
    } else {
      println(&entry);
    }
  }

  Ok(())
}
