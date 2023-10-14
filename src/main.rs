


use clap::Parser;
mod static_ds;
use std::collections::HashMap;


fn identify_region(regions:  HashMap<&str, &str>, wmi: &str) -> String {
    match regions.get(wmi) {
        Some(&region) => format!("{}",  region),

        _ => format!("Unknown Region")
    }
}

#[derive(Parser)]
struct Cli {
    vin: String
}


fn findyear(vis: &str) -> String {
    match vis {

        "1" => String::from("2000"),
        "2" => String::from("2002"),
        "3" => String::from("2003"),
        "4" => String::from("2004"),
        "5" => String::from("2005"),
        "6" => String::from("2006"),
        "7" => String::from("2007"),
        "8" => String::from("2008"),
        "9" => String::from("2009"),
        "A" => String::from("2010"),
        "B" => String::from("2011"),
        "C" => String::from("2012"),
        "D" => String::from("2013"),
        "E" => String::from("2014"),
        "F" => String::from("2015"),
        "G" => String::from("2016"),
        "H" => String::from("2017"),
        "J" => String::from("2018"),
        "K" => String::from("2019"),
        "L" => String::from("2020"),
        "M" => String::from("2021"),
        "N" => String::from("2022"),
        "P" => String::from("2023"),
        "R" => String::from("2024"),
        "S" => String::from("2025"),
        "T" => String::from("2026"),
        "V" => String::from("2027"),
        "W" => String::from("2028"),
        "X" => String::from("2029"),
        "Y" => String::from("2030"),

        _ => String::from("No Year found or newer 2030")

    }
}

fn identify_wmi(manufactures: HashMap<&'static str, &'static str>,wmi: &str) -> String {
    match manufactures.get(wmi) {
        Some(&manufac) => format!("{}",  manufac),

        _ => format!("Unknown manufacturer")
    }

}


fn main() {
    let args = Cli::parse();

    if args.vin != args.vin.to_uppercase() {
        return println!("Please use uppercase letters")
    }
  
    let all_chars = args.vin.split("").collect::<Vec<&str>>(); 

    let regions = static_ds::create_static_regions();
    let manufacturers = static_ds::create_static_manufactures();

    let wmi =  &all_chars[1..4];
    let vds = &all_chars[4..10];
    let vis = all_chars[10];
    let herstellerwerk = all_chars[11];

    let ident_serial = &all_chars[12..];

    println!("Region: {}", identify_region(regions,&wmi.first().unwrap()));
    println!("Hersteller: {}", identify_wmi(manufacturers ,&wmi.concat()));
    println!("Modelljahr {}", findyear(vis));


    println!("Baureihe: {}", vds.join(""));
    println!("Hersteller Werk: {}", herstellerwerk);

    println!("Fahrzeug Identifier: {}", &ident_serial.concat());






    
}
