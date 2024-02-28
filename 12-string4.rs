fn main() {

    //String birleştirme

    let ad = String::from("Mustafa"); //tür belirtmedik

    let soyad: String = "Büyükdereli".to_string();

    let ben_kimim = ad + " " + &soyad; 

    println!("Ben kimim: {}", ben_kimim); //Ben kimim: Mustafa Büyükdereli
}

