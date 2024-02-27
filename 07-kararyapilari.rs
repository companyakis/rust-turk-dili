// if-else yapıları ve mantıksal sınamalar

// ==, !=, <, >, <=, >=, !, ||, && kullanımları

fn main() {
    
    let sayi1: u8 = 221;
    
    let sayi2: u8 = 221;
    
    if sayi1 > sayi2 {
        println!("Sayı 1 sayı 2'den büyüktür.");
    } 
    else if sayi1 != 100 && sayi2 > sayi1 {
        println!("Sayı 1 100'e eşit değil ve sayı 2 sayı 1'den büyük.");
    }
    else if sayi1 == 200 || sayi1 >= sayi2 {
        println!("Sayı 1 200'e eşit veya sayı 1 sayı 2'ye eşit veya daha büyük.");
    }
    else {
        println!("Bu kadar karmaşa iyi değil, ama tek örnekte çoklu durum göstermek istedim");
    }
}
