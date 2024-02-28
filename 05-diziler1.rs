fn main() {

    //arrays-diziler, degişmez uzunluktalar ve aynı türden veri tutarlar
    
    let sayilar: [u8; 5] = [0, 25, 50, 100, 190];
    
    //bütününü yazdırmak için {:?} unutmayalım
    
    println!("Sayılar: {:?}", sayilar);
    
    //rust dizini (index) 0'dan başlar
    
    println!("Üçüncü sayı: {}", sayilar[2]);
    
}


/*
Sayılar: [0, 25, 50, 100, 190]
Üçüncü sayı: 50
*/

