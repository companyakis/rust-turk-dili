fn main() {

    //String veri türü (&str ile karıştırmayalım!)

    //Genişletilebilir veya küçültülebilir (sabit değil)

    //UTF-8 kodlama kullanılarak, her karakter byte olarak kayıt edilir

    //String::from() veya to_string() kullanacağız

    let ad = String::from("Mustafa"); //tür belirtmedik

    let soyad: String = "Büyükdereli".to_string();

    println!("{} {}", ad, soyad); //Mustafa Büyükdereli

}

