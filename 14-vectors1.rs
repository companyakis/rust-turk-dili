fn main() {
    /*
    Çoklu veri yapıları - Vectors

    Heap Memory'de saklanırlar

    Veri ekleme ve çıkarma yapılabilir

    Aynı türden verileri saklar

    Her bir öğreye dizin sayısı (index) ile ulaşabiliriz

    */

    //boş bir vector oluşturalım

    let yaslar: Vec<u8> = Vec::new();

    //veri eklenmiş olarak vector oluşturalım

    let yillar: Vec<u16> = vec![1923, 2024, 2025];

    println!("Yıllar verisi: {:?}", yillar); //Yıllar verisi: [1923, 2024, 2025]

}

