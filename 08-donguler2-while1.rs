fn main() {

    let mut sayac: u8 = 0; //Bir önceki örnekte u16 kullandık. Sizce hangisi doğru?
    
    while sayac < 100 {
    
        sayac += 5;
    
        println!("Sayaç değeri: {}", sayac);
    }
    
    println!("Sayaç sonlandı!");
    
    println!("Sayacın son değeri: {}", sayac);
 
}

/*

Sayaç değeri: 5
Sayaç değeri: 10
Sayaç değeri: 15
Sayaç değeri: 20
Sayaç değeri: 25
Sayaç değeri: 30
Sayaç değeri: 35
Sayaç değeri: 40
Sayaç değeri: 45
Sayaç değeri: 50
Sayaç değeri: 55
Sayaç değeri: 60
Sayaç değeri: 65
Sayaç değeri: 70
Sayaç değeri: 75
Sayaç değeri: 80
Sayaç değeri: 85
Sayaç değeri: 90
Sayaç değeri: 95
Sayaç değeri: 100
Sayaç sonlandı!
Sayacın son değeri: 100

*/



