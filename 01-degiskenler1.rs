fn main() {

    //Değişken oluşturma
    //Değişken adı ve türü
    let sayi_1:u8;
    
    //Değişkene değer atama
    sayi_1 = 101;
    
    //Değişken oluşturma ve değer atama birlikte
    let sayi_2:u8 = 99;
    
    //Değişkenin değerini yazdıralım ve burada println!("", {}) kullanalım
    println!("sayi_1: {}", sayi_1);
    println!("sayi_2: {}", sayi_2);
    
    //Verinin RAM adresini yazdırabiliriz
    //sayi_1 değişkeninin değerinin tutulduğu adres 
    println!("sayi_1 değişkenin verisinin RAM adresi: {:p}", &sayi_1);
    
    //sayi_2 değişkeninin değerinin tutulduğu adres
    println!("sayi_2 değişkenin verisinin RAM adresi: {:p}", &sayi_2);

}


/*
sayi_1: 101
sayi_2: 99
sayi_1 değişkenin verisinin RAM adresi: 0x7ffd98019476
sayi_2 değişkenin verisinin RAM adresi: 0x7ffd98019477
*/

