fn main() {
    /*

    referansı kaldırma
    dereferencing

     */

    let mut yil: u16 = 2024;

    let yeni_yil = &mut yil;

    *yeni_yil = 2045; //Adres bilgisini kaldırıyoruz gibi düşünebiliriz

    println!("{}", yeni_yil); //2045

    yil = 2020;

    println!("{}", yil); //2020
}
