fn main() {

    let sayilar: Vec<i8> = vec![-1, -3, 10, 101, 0];

    //Option ile iki örnek yapalım

    let aranan_sayi_1: Option<&i8> = sayilar.get(3);

    //match kullanimi

    match aranan_sayi_1 {
        Some(i) => println!("Aranan sayı 1: {i}"), //Aranan sayı 1: 101
        None => println!("Bu dizine ait sayı bulunmamaktadır!"),
    }
}


