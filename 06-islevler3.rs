fn yazdir_islevi(sayi: u8, yazi: String) {
  
    println!("Sayı: {} ve yazı: {}", sayi, yazi);
}

fn main() {

    //kullanıcı tanımlı işlev ve can sıkıcı sahiplik kuralları için bir örnek

    let sayi: u8 = 101;

    let ben: String = "Mustafa Büyükdereli".to_string();

    yazdir_islevi(sayi, ben); //Sayı: 101 ve yazı: Mustafa Büyükdereli

    //println!("{}", ben); //hata alırız! ben değişkeninin değeri yazi'ya taşındı

    //clone() kullanmak istiyorum:) => bir sonraki örnekte ödünçlük ile sorunu çözmeye çalışalım
}
