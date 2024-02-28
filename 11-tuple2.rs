fn main() {

    //Farklı türden veriler tutulabilir
    //Boyut değişmez, ama tutulan veriler değişebilir

    //ad, id, birim ve evlilik

    //mut ile veri değişebilirliği sağlanacak

    let mut calisan_bilge: (String, u16, String, bool) = ("Bilge".to_string(), 1245, "Alacaklar".to_string(), true);

    println!("ID: {}", calisan_bilge.1); //ID: 1245

    calisan_bilge.1 = 999;

    println!("ID: {}", calisan_bilge.1); //ID: 999

}

