fn main() {

    let mut sayac: u8 = 0;
    
    let dongu_bilgisi = loop {
        
        sayac += 5;

        if sayac == 105 {
            
            break "Sayaç değeri 105 olunca, döngü sonlandırıldı!";
        }
    };
    
    println!("{}", dongu_bilgisi); //Sayaç değeri 105 olunca, döngü sonlandırıldı!

}
