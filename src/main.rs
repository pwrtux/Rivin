
use clap::Parser;
use std::collections::HashMap;
use colored::Colorize;
mod static_ds;
mod test;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    vin: String
}


fn findyear(vis: char) -> String {
    match vis {

        '1' => String::from("2000 or 1970"),
        '2' => String::from("2001 or 1971"),
        '3' => String::from("2002 or 1972"),
        '4' => String::from("2003 or 1973"),
        '5' => String::from("2004 or 1974"),
        '6' => String::from("2005 or 1975"),
        '7' => String::from("2006 or 1976"),
        '8' => String::from("2007 or 1977"),
        '9' => String::from("2008 or 1978"),
        'A' => String::from("2009 or 1979"),
        'B' => String::from("2010 or 1980"),
        'C' => String::from("2011 or 1981"),
        'D' => String::from("2012 or 1982"),
        'E' => String::from("2013 or 1983"),
        'F' => String::from("2014 or 1984"),
        'G' => String::from("2015 or 1985"),
        'H' => String::from("2016 or 1986"),
        'J' => String::from("2017 or 1987"),
        'K' => String::from("2018 or 1988"),
        'L' => String::from("2019 or 1989"),
        'M' => String::from("2020 or 1990"),
        'N' => String::from("2021 or 1991"),
        'P' => String::from("2022 or 1992"),
        'R' => String::from("2023 or 1993"),
        'S' => String::from("2024 or 1994"),
        'T' => String::from("2025 or 1995"),
        'V' => String::from("2026 or 1996"),
        'W' => String::from("2027 or 1997"),
        'X' => String::from("2028 or 1998"),
        'Y' => String::from("2029 or 1999"),

        _ => String::from("No year found")

    }
}

fn identify_single_region( wmi: char) -> String {
    
  match wmi {    
        '1' | '4' |  '5'  => String::from("America"),
        '2' => String::from("Canada"),
        '3' => String::from("Mexico"),
        'W' => String::from("Germany"),
        'J' => String::from("Japan"),
        'L' => String::from("China"),
        _  => String::from("Unknown Region"),    
      
}
}
 

fn identify_region(regions:  HashMap<&str, &str>, wmi: &str) -> String{
    match regions.get(wmi) {
        Some(&region) => format!("{}",  region),
        _ => String::from("Unknown Region")
    }

}



fn identify_wmi(manufactures: HashMap<&'static str, &'static str>, wmi: &str) -> String {

    match manufactures.get(wmi) {
        Some(&manufac) => format!("{}",  manufac),

        _ => format!("Unknown manufacturer")
    }

}

 
fn validate_vin(vin: &String) -> Vec<(&str, &str, &str)> {
    let re = Regex::new(r"^(?<wmi>[0-9A-HJ-NPR-Z]{3})(?<vds>[0-9A-HJ-NPR-Z]{6})(?<vis>[0-9A-HJ-NPR-Z]{8})$").unwrap();
    if !re.is_match(vin) {
        panic!("{} {}", "VIN is not valid:".red(), vin.bold())
    }
    let dates: Vec<(&str, &str, &str)> = re.captures_iter(vin).map(|caps| {
        let (_, [year, month, day]) = caps.extract();
        (year, month, day)
    }).collect();
    dates
}
const LOGO: &str = r#"

                    ██████╗ ██╗██╗   ██╗██╗███╗   ██╗
                    ██╔══██╗██║██║   ██║██║████╗  ██║
                    ██████╔╝██║██║   ██║██║██╔██╗ ██║
                    ██╔══██╗██║╚██╗ ██╔╝██║██║╚██╗██║
                    ██║  ██║██║ ╚████╔╝ ██║██║ ╚████║
                    ╚═╝  ╚═╝╚═╝  ╚═══╝  ╚═╝╚═╝  ╚═══╝
                                            
"#;



fn main() {
    
    println!("{}", LOGO.bright_red());


    let args = Cli::parse();
   
    let allchrs = validate_vin(&args.vin);
    let firsti = allchrs.first();

    let regions = static_ds::create_static_regions();
    let manufacturers = static_ds::create_static_manufactures();

    let wmi =   firsti.unwrap().0;
    let vds =  firsti.unwrap().1;
    let vis = firsti.unwrap().2;

    

    let single_region_lookup = identify_single_region(wmi.chars().next().unwrap());

    let mut vds2 = firsti.unwrap().0.to_string();
    vds2.remove(2); // Remove third charakter to do a lookup 
    
    if single_region_lookup == "Unknown Region".to_string() {
        println!("No single region found");
        println!("{} {}", "Region:".underline(), identify_region(regions, &vds2));
    } else {
        println!("{} {}", "Region:".underline(), single_region_lookup);
    }

    
    println!("{} {}", "Hersteller:".underline(), identify_wmi(manufacturers ,wmi));

    println!("{} {}", "Modelljahr:".underline(), findyear(vis.chars().next().unwrap()));

    
 
    println!("{} {}", "Baureihe:".underline(), vds);



    
}
