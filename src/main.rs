use clap::{arg, command};

fn main() -> Result<(), anyhow::Error> {
  let matches = command!()
    .arg(arg!(<subtraction_archive> "Archive to subtract"))
    .arg(arg!(<target> "Folder to subtract from"))
    .get_matches();

  let subtraction = matches
    .get_one::<String>("subtraction_archive")
    .expect("required");
  let target = matches.get_one::<String>("target").expect("required");

  println!("subtracting {:?} from {:?}", &subtraction, &target);

  archive_subtract::subtract(subtraction, target)?;
  println!("done.");
  Ok(())
}
