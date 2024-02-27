fn main() {
    let mut sayac: u16 = 0;
    
    loop {
        sayac += 5;
        
        println!("Sayaç değeri: {}", sayac);
        
        if sayac == 105 {
            println!("Sayaç sonlandı!");
            break;
        }
    }
    
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
Sayaç değeri: 105
Sayaç sonlandı!
Sayacın son değeri: 105




*/
