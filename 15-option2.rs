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

    //Option ile iki örnek yapalım

    let aranan_sayi_1: Option<&i8> = sayilar.get(75);

    println!("Aranan sayı 1: {:?}", aranan_sayi_1); //Aranan sayı 1: None

    let aranan_sayi_2: Option<&i8> = sayilar.get(2);

    println!("Aranan sayı 2: {:?}", aranan_sayi_2); //Aranan sayı 2: Some(10)

}

/*


*/
