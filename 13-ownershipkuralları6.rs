fn main() {
    /*
    HEAP Memory

    Ödünç Alma Borrowing

    Clone() ve move olmadan, pointer ile dayanak gösterebilirme (referans göstermek) 

    & ile veriye referans gösteriyoruz. Adrese vurgu yapmak gibi düşünebiliriz.

    Verinin değişebilir (mutable) olup olmadığına dikkat edelim
     */

    //immutable borrowing örneği

    let gunaydin_de = String::from("Günaydın Türkiye");

    let soz1 = &gunaydin_de;
    let soz2 = &gunaydin_de;
    let soz3 = &gunaydin_de;
    let soz4 = &gunaydin_de;

    println!("{}", soz1); //Günaydın Türkiye
    println!("{}", soz2); //Günaydın Türkiye
    println!("{}", soz3); //Günaydın Türkiye
    println!("{}", soz4); //Günaydın Türkiye

    //Görüldüğü üzere, veriye erişimde sorun yaşanmadı
}
