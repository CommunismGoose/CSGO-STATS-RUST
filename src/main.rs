#![allow(unused_variables)]
//importing needed librarys
//use std::fs;
use std::path::Path;
use std::io;
//use std::fs::File;
use std::process::exit;
//struct defining for data abstraction
struct Data{
    kills:i64,
    assists:i64,
    deaths:i64,
    hskill:i64
}
fn main() ->  std::io::Result<()> {
    let _stats = ["kill", "assist", "death", "mvps", "hs%", "adr","score"];
    let _maps = ["agency.txt","office.txt","ancient.txt", "anubis.txt", "cache.txt", "dust 2.txt", "inferno.txt", "mirage.txt", "nuke.txt", "overpass.txt", "train.txt", "tuscan.txt", "vertigo.txt"];
    //gets path for stats.txt
    let path = Path::new("stats.txt");
    //makes tuple for all the stats in stats.txt
    let userdata = statadder(path,0,0,0,0);
    println!("You have:");
    println!("{} kills, {} assists, {} deaths, and {} headshot kills!",userdata.0,userdata.1,userdata.2,userdata.3);
    println!("please choose a map to add a stat too.");
    let mapchoose: String = input();
    println!("Please write the number you are adding then click senter for every stat\nFirst:Kills:");
    let kills: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nAssists:");
    let assists: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nDeaths:");
    let deaths: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nHeadshot %(please note to include this as raw number no %");
    let mut hsprecent:i64 = input().parse().expect("Please type in a valid number!");
    hsprecent = hsprecent/100;
    println!("\n # of mvps:");
    let mvps: i64 = input().parse().expect("Please type in a valid number!");
    println!("Average Damage per Round (ADR):");
    let average_damage_round: i64 = input().parse().expect("Please type in a valid number!");
    println!("Please type in the final score!");
    let score: i64 = input().parse().expect("Please type in a valid number!");
    println!("Please type in the amount of rounds won");
    let rounds_won: i64 = input().parse().expect("Please type in a valid number!");
    println!("Please type in the rounds lost");
    let rounds_lost: i64 = input().parse().expect("Please type in a valid number!");
    //fs::write(path, .as_bytes())?;
    let path = match &mapchoose[..]{
        "Agency"=>Path::new("agency.txt"),
        "Ancient"=>Path::new("ancient.txt"),
        "Anubis"=>Path::new("anubis.txt"),
        "Cache"=>Path::new("cache.txt"),
        "Dust 2"=>Path::new("dust 2.txt"),
        "Inferno"=>Path::new("inferno.txt"),
        "Mirage"=>Path::new("mirage.txt"),
        "Nuke"=>Path::new("nuke.txt"),
        "Office"=>Path::new("office.txt"),
        "Overpass"=>Path::new("overpass.txt"),
        "Train"=>Path::new("train.txt"),
        "Tuscan"=>Path::new("tuscan.txt"),
        "Vertigo"=>Path::new("vertigo.txt"),
        &_=>exit(0),
    };
    let userdata = statadder(path,0,0,0,0);
    println!("{mapchoose},{:?}",path,);
    Ok(())
}
fn statadder(path:&Path,kill:i64,assist:i64,death:i64,hskill:i64)->(i64,i64,i64,i64){
    let content = std::fs::read_to_string(&path).expect("could not read file");
    let mut userdata = Data{kills:kill,assists:assist,deaths:death,hskill:hskill};
    for line in content.lines() {
        //create temp line to make string slice of
        let templine = line;
        let slice: String = (&templine[5..]).to_string();
        let slice: i64 = match slice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //adds the amount to where needed 
        if line.contains("kills"){
            userdata.kills = userdata.kills+slice;
        }else if line.contains("asist"){
            userdata.assists = userdata.assists+slice;
        }else if line.contains("death"){
            userdata.deaths = userdata.deaths+slice;
        } else if line.contains("hs%"){
            userdata.hskill = userdata.hskill+slice;
        } 
        
    }
    return (userdata.kills,userdata.assists,userdata.deaths,userdata.hskill);
}
fn input() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Something failed!");
    let x = x.trim().to_string();
    return x;
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
