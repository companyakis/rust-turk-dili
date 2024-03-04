fn main() {
    let sayi1: i8 = 15;

    let sayi2: i8 = 100;

    let sayi3: i8 = -40;

    let en_buyuk_sayi = if sayi1 > sayi2 && sayi1 > sayi3 { sayi1 } else if  sayi2 > sayi1 && sayi2 > sayi3 { sayi2 } else { sayi3 };

    println!("En büyük sayı: {}", en_buyuk_sayi); //En büyük sayı: 100
}
