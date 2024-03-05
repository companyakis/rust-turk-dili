fn main() {

    //veri güncelleme (ekleme ve silme yapalım)
    //mut olarak belirttik

    let mut sayilar: Vec<u8> = Vec::new();

    sayilar.push(34);
    sayilar.push(44);
    sayilar.push(11);
    sayilar.push(8);
    sayilar.push(70);
    sayilar.push(51);

    println!("Sayılar: {:?}", sayilar); //Sayılar: [34, 44, 11, 8, 70, 51]

    //son öğeyi silme

    sayilar.pop();

    println!("Sayılar: {:?}", sayilar); //Sayılar: [34, 44, 11, 8, 70]

    //dizin numarası ile öğe silme, ilk öğeyi silelim

    sayilar.remove(0);

    println!("Sayılar: {:?}", sayilar); //Sayılar: [44, 11, 8, 70]

}

/*


*/
