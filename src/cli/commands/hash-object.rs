pub fn hash_object(filename: String) -> () {
  let hash = data::hash_object(filename, None);

  println!("{}", hash.unwrap());
}
