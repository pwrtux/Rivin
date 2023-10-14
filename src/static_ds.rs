use std::collections::HashMap;

pub fn create_static_regions() -> HashMap<&'static str, &'static str> {


    let mut regions = HashMap::new();
    regions.insert("A", "Südafrika");
    regions.insert("J", "Japan");

    // Africa
    regions.insert("AA–AH", "Südafrika");
    // Asia
    regions.insert("J", "Japan");
    regions.insert("AA", "Südafrika");
    regions.insert("AB", "Südafrika");
    regions.insert("AC", "Südafrika");
    regions.insert("AD", "Südafrika");
    regions.insert("AE", "Südafrika");
    regions.insert("KL–KR", "Südkorea");
    regions.insert("L", "VR China");
    regions.insert("MA", "Indien");
    regions.insert("MB", "Indien");
    regions.insert("MC", "Indien");
    regions.insert("MD", "Indien");
    regions.insert("ME", "Indien");

    // Fix Regions below
    regions.insert("MF–MK", "Indonesien");
    regions.insert("ML–MR", "Thailand");
    regions.insert("NL–NM", "Türkei");
    regions.insert("PA–PE", "Philippinen");
    regions.insert("PL–PR", "Malaysia");
    regions.insert("RA–RE", "Vereinigte Arabische Emirate");
    regions.insert("RF–RK", "Taiwan");
    regions.insert("RL–RR", "Vietnam");
    // Europe
    regions.insert("SA–SM", "Vereinigtes Königreich");
    regions.insert("SN–ST", "Deutschland");
    regions.insert("SU–SZ", "Polen");
    regions.insert("TA–TH", "Schweiz");
    regions.insert("TJ–TP", "Tschechische Republik");
    regions.insert("TR–TV", "Ungarn");
    regions.insert("UA–UM", "Dänemark");
    regions.insert("UN–UT", "Irland");
    regions.insert("UU–UZ", "Rumänien");
    regions.insert("U1–U7", "Slowakei");
    regions.insert("VA–VE", "Österreich");
    regions.insert("VF–VR", "Frankreich");
    regions.insert("VS–VW", "Spanien");
    regions.insert("VX–V2", "Jugoslawien");
    regions.insert("W", "Deutschland");
    regions.insert("XL–XP", "Niederlande");
    regions.insert("XS–XW", "UdSSR");
    regions.insert("X3–X0", "Russland");
    regions.insert("YA–YE", "Belgien");
    regions.insert("YF–YK", "Finnland");
    regions.insert("YS–YW", "Schweden");
    regions.insert("ZA–ZR", "Italien");
    // North America
    regions.insert("1", "USA");
    regions.insert("4", "USA");
    regions.insert("5", "USA");
    regions.insert("2", "Kanada");
    regions.insert("3", "Mexiko");
    // Australia/Oceania
    regions.insert("6A–6W", "Australien");
    regions.insert("7A–7E", "Neuseeland");
    // South America
    regions.insert("8A–8E", "Argentinien");
    regions.insert("8X–82", "Venezuela");
    regions.insert("9A–9E", "Brasilien");
    regions.insert("9F–9J", "Kolumbien");
    regions.insert("93–99", "Brasilien");

    regions

    

}

pub fn create_static_manufactures() -> HashMap<&'static str, &'static str> {
    let mut manufacturers = HashMap::new();

    manufacturers.insert("CL9", "Wallyscar");
    manufacturers.insert("JA", "Isuzu");
    manufacturers.insert("JF", "Fuji Heavy Industries (Subaru)");
    manufacturers.insert("JH", "Honda");
    manufacturers.insert("JMB", "Mitsubishi");
    manufacturers.insert("JMZ", "Mazda");
    manufacturers.insert("JN", "Nissan");
    manufacturers.insert("JS", "Suzuki");
    manufacturers.insert("JT", "Toyota Japan");
    manufacturers.insert("KL", "Daewoo");
    manufacturers.insert("KMH", "Hyundai (Korea)");
    manufacturers.insert("KN", "Kia");
    manufacturers.insert("KNE", "Kia (Montage nicht Korea)");
    manufacturers.insert("KPT", "Ssangyong");
    manufacturers.insert("LFV", "FAW-Volkswagen Automotive Co.");
    manufacturers.insert("LPS", "Polestar");
    manufacturers.insert("LSV", "SAIC Volkswagen");
    manufacturers.insert("SAJ", "Jaguar");
    manufacturers.insert("SAL", "Land Rover");
    manufacturers.insert("SAR", "Rover");
    manufacturers.insert("SCC", "Lotus Cars");
    manufacturers.insert("SUU", "Solaris Bus & Coach");
    manufacturers.insert("TMA", "Hyundai (Tschechien)");
    manufacturers.insert("TMB", "Škoda");
    manufacturers.insert("TRU", "Audi (Ungarn)");
    manufacturers.insert("UU", "Dacia");
    manufacturers.insert("U5Y", "Kia (Slowakei, Pkw)");
    manufacturers.insert("U6Z", "Kia (Slowakei, SUV)");
    manufacturers.insert("VA0", "ÖAF Gräf & Stift AG (Österreich)");
    manufacturers.insert("VFA", "Alpine, heute Renault");
    manufacturers.insert("VF1", "Renault");
    manufacturers.insert("VF3", "Peugeot");
    manufacturers.insert("VF7[8], VR7", "Citroën");
    manufacturers.insert("VNK", "Toyota Frankreich");
    manufacturers.insert("VSK", "Nissan Spanien");
    manufacturers.insert("VSS", "Seat");
    manufacturers.insert("WAG", "Neoplan");
    manufacturers.insert("WAU", "Audi");
    manufacturers.insert("WAP", "ALPINA");
    manufacturers.insert("WBA", "BMW");
    manufacturers.insert("WBS", "BMW M GmbH");
    manufacturers.insert("WBY", "BMW i (Elektro)");
    manufacturers.insert("WDB", "Mercedes-Benz AG");
    manufacturers.insert("W1K", "Mercedes-Benz AG");
    manufacturers.insert("W1N", "Mercedes-Benz AG");
    manufacturers.insert("WMX", "Mercedes-Benz AG");
    manufacturers.insert("WDC", "DaimlerChrysler/Daimler AG (seit 2020 abgelöst durch Mercedes-Benz AG)");
    manufacturers.insert("WDD", "DaimlerChrysler/Daimler AG (seit 2020 abgelöst durch Mercedes-Benz AG)");
    manufacturers.insert("WDF", "DaimlerChrysler/Daimler AG (seit 2020 abgelöst durch Mercedes-Benz AG)");
    manufacturers.insert("WMX", "DaimlerChrysler/Daimler AG (seit 2020 abgelöst durch Mercedes-Benz AG)");
    manufacturers.insert("WEB", "EvoBus");
    manufacturers.insert("WF0", "Ford Deutschland");
    manufacturers.insert("WGF", "GFB GmbH");
    manufacturers.insert("WJR", "Irmscher");
    manufacturers.insert("WKA", "Kaeser Kompressoren");
    manufacturers.insert("WKK", "Kässbohrer Fahrzeugwerke");
    manufacturers.insert("WLA", "Langendorf GmbH");
    manufacturers.insert("WMA", "MAN");
    manufacturers.insert("WME", "Smart");
    manufacturers.insert("WMW", "MINI");
    manufacturers.insert("WP0", "Porsche");
    manufacturers.insert("WP1", "Porsche Cayenne, Porsche Macan");
    manufacturers.insert("WSM", "Schmitz Cargobull");
    manufacturers.insert("WUA", "Audi Sport GmbH");
    manufacturers.insert("WVG", "Volkswagen");
    manufacturers.insert("WVM", "VW-MAN");
    manufacturers.insert("WVW", "Volkswagen");
    manufacturers.insert("WV1", "Volkswagen Nutzfahrzeuge");
    manufacturers.insert("WV2", "Volkswagen Nutzfahrzeuge");
    manufacturers.insert("W0L", "Opel");
    manufacturers.insert("W0V", "Opel Vauxhall");
    manufacturers.insert("W0SV", "Opel Special Vehicles");
    manufacturers.insert("W08", "Saturn Astra");
    manufacturers.insert("W09", "deutsche Hersteller < 500 Fz.");
    manufacturers.insert("XP7", "Tesla Deutschland");
    manufacturers.insert("YK1", "Saab");
    manufacturers.insert("YS3", "Saab");
    manufacturers.insert("YV1", "Volvo");
    manufacturers.insert("ZAR", "Alfa Romeo");
    manufacturers.insert("ZCF", "Iveco");
    manufacturers.insert("ZDF", "Ferrari Dino");
    manufacturers.insert("ZFA", "Fiat");
    manufacturers.insert("ZFF", "Ferrari");
    manufacturers.insert("ZHW", "Lamborghini");
    manufacturers.insert("ZLA", "Lancia");
    manufacturers.insert("ZAM", "Maserati");
    manufacturers.insert("1C", "Chrysler");
    manufacturers.insert("1F", "Ford Motor Company");
    manufacturers.insert("1G", "General Motors");
    manufacturers.insert("1G3", "Oldsmobile");
    manufacturers.insert("1GC", "Chevrolet");
    manufacturers.insert("1GM", "Pontiac");
    manufacturers.insert("1H", "Honda USA");
    manufacturers.insert("1J", "Jeep");
    manufacturers.insert("1L", "Lincoln");
    manufacturers.insert("1M", "Mercury");
    manufacturers.insert("1N", "Nissan USA");
    manufacturers.insert("1VW", "Volkswagen USA");
    manufacturers.insert("1YV", "Mazda USA");
    manufacturers.insert("2F", "Ford Motor Company Kanada");
    manufacturers.insert("2G", "General Motors Kanada");
    manufacturers.insert("2G1", "Chevrolet Kanada");
    manufacturers.insert("2G1", "Pontiac Kanada");
    manufacturers.insert("2HM", "Hyundai Kanada");
    manufacturers.insert("2M", "Mercury");
    manufacturers.insert("3F", "Ford Motor Company Mexiko");
    manufacturers.insert("3G", "General Motors Mexiko");
    manufacturers.insert("3H", "Honda Mexiko");
    manufacturers.insert("3VW", "Volkswagen Mexiko");
    manufacturers.insert("4F", "Mazda USA");
    manufacturers.insert("4M", "Mercury");
    manufacturers.insert("4S", "Subaru of Indiana Automotive");
    manufacturers.insert("4US", "BMW USA");
    manufacturers.insert("5L", "Lincoln");
    manufacturers.insert("5YJ", "Tesla USA");
    manufacturers.insert("6F", "Ford Motor Company Australien");
    manufacturers.insert("6H", "General Motors-Holden");
    manufacturers.insert("6MM", "Mitsubishi Motors Corporation Australien");
    manufacturers.insert("6T1", "Toyota Australien");
    manufacturers.insert("9BW", "Volkswagen do Brasil (Brasilien)");

    manufacturers
}