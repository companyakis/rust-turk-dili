fn main() {

    //işlevlerden çoklu dönüş elde edebiliriz
    
    //toplama ve çıkarma ve günaydın deme aynı anda yapabilen işlev
    
    //tuple dönüş olduğunu anımsayalım!
    
    fn topla_cikar_gunaydin(a: i32, b: i32, ad: String) -> (i32, i32, String) {
        let topla = a + b;
        let cikar = a - b;
        let gunaydin = format!("Günaydın, {ad}!");
        (topla, cikar, gunaydin)
    }
    
    let sayi_1 = 2500;
    
    let sayi_2 = -2500;
    
    let ad = "Mustafa";
    
    println!("Sayıların toplamı: {}", topla_cikar_gunaydin(sayi_1, sayi_2, ad.to_string()).0);
    
    
    println!("Sayıların farkı: {}", topla_cikar_gunaydin(sayi_1, sayi_2, ad.to_string()).1);
    
    println!("{}", topla_cikar_gunaydin(sayi_1, sayi_2, ad.to_string()).2);

}


/*
Sayıların toplamı: 0
Sayıların farkı: 5000
Günaydın, Mustafa!
*/

