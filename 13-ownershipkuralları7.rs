fn main() {
    /*
    HEAP Memory

    Ödünç Alma Borrowing

    Clone ve move olmadan, pointer ile dayanak gösterebilirme (referans göstermek) 

    & ile veriye referans gösteriyoruz. Adrese vurgu yapmak gibi düşünebiliriz.

    Verinin değişebilir (mutable) olup olmadığına dikkat edelim
     */

    //mutable borrowing örneği

    //Ödünç alınan veriyi aynı anda kullanamayız

    let mut gunaydin_de = String::from("Günaydın Türkiye");

    println!("{}", gunaydin_de); //Günaydın Türkiye

    let soz1 = &mut gunaydin_de;

    println!("{}", soz1); //Günaydın Türkiye

    //println!("{} -- {}", soz1, gunaydin_de); //Aynı anda kullanamayız. Error!

}
