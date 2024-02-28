fn main() {

    //Farklı türden veriler tutulabilir
    //Boyut değişmez, ama tutulan veriler değişebilir

    //ad, id, birim ve evlilik

    let calisan_bilge: (String, u16, String, bool) = ("Bilge".to_string(), 1245, "Alacaklar".to_string(), true);

    //let ad = calisan_bilge.0;

    //println!("Adı: {}", ad); //Adı: Bilge

    //bütün değişkenler

    let (ad, id, birim, evlilik) = calisan_bilge;

    println!("Adı: {}", ad); 
    println!("ID: {}", id); 
    println!("Birim: {}", birim); 
    println!("Evlilik durumu: {}", evlilik); 

    /*
    Adı: Bilge
    ID: 1245
    Birim: Alacaklar
    Evlilik durumu: true
    */

    //Tutulan veriler değişebilir demiştik. Deneyelim mi? 

    //calisan_bilge.1 = 999;

    //println!("ID: {}", id); //cannot assign hatası aldık!

    //hatanın nedenini ikinci bölümde giderelim

}

