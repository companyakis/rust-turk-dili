fn main() {
    /*
    Option veri türü

    Enum türünden, özel tanımlı bir veri türü.

    İki adet değer alır -> Some(T) ve None

    Neden kullanıyoruz? Veri döndüren yapılardan güvenli veri almak için...

    Veri elde edebiliyorsak Some(T), elde edemiyorsan None değeri gelir.

    Böylece programın hata vermesi engellenmiş olur.

    */

    let sayilar: Vec<i8> = vec![-1, -3, 10, 101, 0];

    //hata alalım

    let sayi1 = &sayilar[75]; //75 diye bir dizin var mı? Yok!

    println!("Sayı 1: {}", sayi1); //index out of bounds: the len is 5 but the index is 75
}
