//kullanıcı tanımlı işlev ve ödünçlük

//&String kullanımı ile ödünç almaya vurgu yapıyoruz

fn yazdir_islevi(sayi: u8, yazi: &String) {
    println!("Sayı: {} ve yazı: {}", sayi, yazi);
}

fn main() {

    let sayi: u8 = 101;

    let ben: String = "Mustafa Büyükdereli".to_string();

    yazdir_islevi(sayi, &ben); //Sayı: 101 ve yazı: Mustafa Büyükdereli

    //hata almayacağız; çünkü taşınma değil, ödünçlük gerçekleşti

    println!("{}", ben); //Mustafa Büyükdereli 
}
