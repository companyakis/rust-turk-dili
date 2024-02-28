fn main() {

    //String işlevleri örnekleri

    let mut ad = String::from("Mustafa"); //tür belirtmedik

    let mut soyad: String = "Büyükdereli".to_string();

    //ad.push('m');

    //println!("{}", ad); //Mustafam

    ad.push_str(" Kutluk");

    println!("{}", ad); //Mustafa Kutluk

    //uzunluk bilgisi

    println!("{}", ad.len()); //14, boşluğun da sayıldığını unutmayalım

    //son harf gitsin

    soyad.remove(soyad.len() - 1);

    println!("{}", soyad); //Büyükderel  

}

