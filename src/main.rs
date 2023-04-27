use std::fs::File;
use std::fs;
use std::path::Path;

fn main() ->  std::io::Result<()> {
    let path = Path::new("stats.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    fs::write(path, b"opensource.com")?;
    Ok(())
}

//step 1:
//read files 
//asks what the user would like to add
//add to files
//write to files

//step 2:
//take numbers from file and calculate stats
//print stats in useful formats

//step 3:
//use https://github.com/rakijah/CSGSI to get theese stats live
//