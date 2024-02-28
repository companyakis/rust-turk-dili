fn main() {

    //&str veri türü

    let ata_sozu: &str = "Kurt kışı geçirir, ama yediği ayazı unutmaz";

    //String türünden farklı

    //Değiştirilemez. Slicing (parça alma) yapılabilir

    println!("Atasözü: {}", ata_sozu);

    println!("Börü : {}", &ata_sozu[0..5]); //Börü : Kurt

}

