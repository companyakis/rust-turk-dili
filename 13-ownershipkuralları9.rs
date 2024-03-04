fn main() {
    /*
    HEAP Memory

    Ödünç Alma Borrowing

    Verimizi, en son ödünç alan değişken kullanabilir, önceden ödünç alan değişkenler kullanamaz!

    Ek olaraktan, aşağıda görüleceği üzere, başlangıç verisi kullanılmakta ve değişmektedir.

    İlk verimiz (degişken 1'de gördüğümüz "Günaydın Türkiye") kullanıldı ve değişti.

    Ki değişken 1'i 35. satırda yazdırdım. Veri değişmiş durumda!

    İlk verimizin değiştiğini unutmadan, işlemleri sürdürmekte yarar var.

     */

    let mut degisken_1 = String::from("Günaydın Türkiye");

    let degisken_2 = &mut degisken_1;

    degisken_2.push_str(" yeni umutlara!");

    println!("İkinci değişken: {}", degisken_2); //İkinci değişken: Günaydın Türkiye yeni umutlara!

    let degisken_3 = &mut degisken_1;

    degisken_3.push_str(", bilim ve us ile gülebilmeye günaydın!"); 

    println!("Üçüncü değişken: {}", degisken_3); //Üçüncü değişken: Günaydın Türkiye yeni umutlara!, bilim ve us ile gülebilmeye günaydın!

    //degisken_2.push_str("!?..."); //Bu kodu eklersek, Değişken 3 Hata verir! => "cannot borrow `degisken_1` as mutable more than once at a time"

    println!("Birinci değişken: {}", degisken_1); //Birinci değişken: Günaydın Türkiye yeni umutlara!, bilim ve us ile gülebilmeye günaydın!
}
