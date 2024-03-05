fn main() {
    //vector öğelerine erişim

    let adlar: Vec<String> = vec![
        "Mustafa".to_string(),
        "Aygün".to_string(),
        "Bilge".to_string(),
        "Kültigin".to_string(),
    ];

    //dizin sayısı (index) ile erişim

    let ikinci_kisi: &str = &adlar[1];

    println!("İkinci kişi: {}", ikinci_kisi); //İkinci kişi: Aygün
}
