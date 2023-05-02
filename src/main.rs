#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
//importing needed librarys
use std::fs;
use std::io;
use std::path::Path;
//use std::fs::File;
use std::process::exit;
//struct defining for data abstraction
struct Data<T>{
    kills: T,
    assists: T,
    deaths: T,
    hskill: T,
    adr: T,
    score: T,
    rounds_won: T,
    rounds_lost: T,
    win: T,
    lost: T,
}


fn main() -> std::io::Result<()> {
    //gets path for stats.txt
    let path = Path::new("stats.txt");
    //makes tuple for all the stats in stats.txt
    let userdata = statadder(path, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    if userdata.9 !=0 && userdata.2 != 0 {   
        println!("You have:");
        println!(
        "{} kills\n{} assists\n{} deaths\n{} K/D\n{} headshot kills\n{} Average Damange per Round \n{} Score\n{} Rounds Won\n{} Rounds Lost\n{} Wins\n{} Loses\n{} Win rate",
        userdata.0, userdata.1, userdata.2, userdata.0 as f64/userdata.2 as f64, userdata.3, userdata.4, userdata.5, userdata.6, userdata.7, userdata.8, userdata.9, userdata.8 as f64/userdata.9 as f64
        );
    } else{
        println!("You have:");
        println!(
        "{} kills\n{} assists\n{} deaths\n{} headshot kills\n{} Average Damange per Round \n{} Score\n{} Rounds Won\n{} Rounds Lost\n{} Wins\n{} Loses",
        userdata.0, userdata.1, userdata.2, userdata.3, userdata.4, userdata.5, userdata.6, userdata.7, userdata.8, userdata.9
        );
    }
    println!("please choose a map to add a stat too.");
    let mapchoose: String = input();
    println!(
        "Please write the number you are adding then click enter for every stat\nFirst, Kills:"
    );
    let kills: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nAssists:");
    let assists: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nDeaths:");
    let deaths: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nHeadshot %(please note to include this as raw number no %");
    let hsprecent: i64 = input().parse().expect("Please type in a valid number!");
    println!("\n# of mvps:");
    let mvps: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nAverage Damage per Round (ADR):");
    let average_damage_round: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nPlease type in the final score!");
    let score: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nPlease type in the amount of rounds won");
    let rounds_won: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nPlease type in the rounds lost");
    let rounds_lost: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nHow many matches did you win across this data?");
    let win: i64 = input().parse().expect("Please type in a valid number!");
    println!("\nHow many matches did you lose across this data?");
    let lose: i64 = input().parse().expect("Please type in a valid number!");
    let path = match &mapchoose[..] {
        "Agency" => Path::new("agency.txt"),
        "Ancient" => Path::new("ancient.txt"),
        "Anubis" => Path::new("anubis.txt"),
        "Cache" => Path::new("cache.txt"),
        "Dust 2" => Path::new("dust 2.txt"),
        "Inferno" => Path::new("inferno.txt"),
        "Mirage" => Path::new("mirage.txt"),
        "Nuke" => Path::new("nuke.txt"),
        "Office" => Path::new("office.txt"),
        "Overpass" => Path::new("overpass.txt"),
        "Train" => Path::new("train.txt"),
        "Tuscan" => Path::new("tuscan.txt"),
        "Vertigo" => Path::new("vertigo.txt"),
        &_ => exit(0),
    };
    let userdata = statadder(
        path,
        kills,
        assists,
        deaths,
        hsprecent,
        average_damage_round,
        score,
        rounds_won,
        rounds_lost,
        win,
        lose,
    );
        let unusedvariable = writetofile(&path,userdata.0,userdata.1,userdata.2,userdata.3,userdata.4,userdata.5,userdata.6,userdata.7,userdata.8,userdata.9,);
    let path = Path::new("stats.txt");
    let unusedvariable = writetofile(&path,userdata.0,userdata.1,userdata.2,userdata.3,userdata.4,userdata.5,userdata.6,userdata.7,userdata.8,userdata.9,);    Ok(())
}


fn statadder(
    path: &Path,
    kill: i64,
    assist: i64,
    death: i64,
    hskill: i64,
    adr: i64,
    score: i64,
    rounds_won: i64,
    rounds_lost: i64,
    win: i64,
    lost: i64,
) -> (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64) {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    let mut userdata = Data {
        kills: kill,
        assists: assist,
        deaths: death,
        hskill: hskill,
        adr: adr,
        score: score,
        rounds_won: rounds_won,
        rounds_lost: rounds_lost,
        win: win,
        lost: lost,
    };
    for line in content.lines() {
        //create temp line to make string slice of
        let templine = line;
        let slice: String = (&templine[5..]).to_string();
        let slice: i64 = match slice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //adds the amount to where needed
        if line.contains("Kills") {
            userdata.kills = userdata.kills + slice;
        } else if line.contains("Assts") {
            userdata.assists = userdata.assists + slice;
        } else if line.contains("Death") {
            userdata.deaths = userdata.deaths + slice;
        } else if line.contains("HSnum") {
            //let hsprec = ((userdata.hskill as f64/100.0)*userdata.kills as f64)+slice as f64;
        } else if line.contains("HSpre"){
            //
        }else if line.contains("AvgDR") {
            userdata.adr = userdata.adr + slice;
        } else if line.contains("Score") {
            userdata.score = userdata.score + slice
        } else if line.contains("RWins") {
            userdata.rounds_won = userdata.rounds_won + slice
        } else if line.contains("RLose") {
            userdata.rounds_lost = userdata.rounds_lost + slice
        } else if line.contains("Wins#") {
            userdata.win = userdata.win + slice
        } else if line.contains("Lose#") {
            userdata.lost = userdata.lost + slice;
        }
    }
    return (
        userdata.kills,
        userdata.assists,
        userdata.deaths,
        userdata.hskill,
        userdata.adr,
        userdata.score,
        userdata.rounds_won,
        userdata.rounds_lost,
        userdata.win,
        userdata.lost,
    );
}



fn writetofile(
    path: &Path,
    kill: i64,
    assist: i64,
    death: i64,
    hskill: i64,
    adr: i64,
    score: i64,
    rounds_won: i64,
    rounds_lost: i64,
    win: i64,
    lost:i64
) -> std::io::Result<()> {
    let userdata = Data{
    kills : String::from("Kills ") + &(kill.to_string()),
    assists : String::from("\nAssts ") + &(assist.to_string()),
    deaths : String::from("\nDeath ") + &(death.to_string()),
    hskill : String::from("\nHSnum ") + &(hskill.to_string()),
    adr : String::from("\nAvgDR ") + &(adr.to_string()),
    score : String::from("\nScore ") + &(score.to_string()),
    rounds_won : String::from("\nRWins ") + &(rounds_won.to_string()),
    rounds_lost : String::from("\nRLose ") + &(rounds_lost.to_string()),
    win : String::from("\nWins# ") + &(win.to_string()),
    lost : String::from("\nLose# ") + &(lost.to_string()),
    };
    fs::write(path, userdata.kills+&userdata.assists+&userdata.deaths+&userdata.hskill+&userdata.adr+&userdata.score+&userdata.rounds_won+&userdata.rounds_lost+&userdata.win+&userdata.lost)?;
    Ok(())
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
