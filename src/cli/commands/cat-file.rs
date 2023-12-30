fn cat_file(object: String) -> () {
  let file = data::get_object(object, None);

  println!("{}", file.unwrap());
}