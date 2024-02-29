fn main() {

    /*
    HEAP Memory

    Çalışma zamanında boyutu belli olmayan veriler saklanır (String örneğin...)
    Veri boyutu değiştirilebilir olan veriler saklanır
    Veri için yer ayrıldığı zaman, bu yerin adresi Pointer'da saklanır
    Pointer'ı işaretleyici bir adres gibi düşünebiliriz
    Pointer, verinin heap memory'deki adresini saklar
    Pointer'ın boyutu sabittir ve Stack'te saklanır
     */

    let string_1 = String::from("Günaydın!");

    println!("{}", string_1); //Günaydın!

    //Bir değerin aynı anda sadece bir sahibi olabilir
    //Taşınma (move) işlemini örneklendirelim

    let string_2 = string_1;

    println!("{}", string_2); //Günaydın!

    //Bakalım, ne olacak? Hata alacağız
    //println!("{}", string_1); //"value borrowed here after move" hatası

    //string_1 artık kullanılamaz

}

