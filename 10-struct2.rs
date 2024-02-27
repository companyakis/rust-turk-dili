#[derive(Debug)]

struct Calisanlar {
    ad: String,
    id: u16,
    eposta: String,
    birim: String,
    aylik_gelir: f32
}

fn main() {

    let ayhan = Calisanlar {
        ad: "Ayhan".to_string(),
        id: 1001,
        eposta: "ayhan@company.com".to_string(),
        birim: "Ödemeler".to_string(),
        aylik_gelir: 48500.0,
    };
    
    println!("Ayhan'ın bilgileri: {:?}", ayhan); //Çok da mantıklı bir yazdırma işlemi değil!
    
    println!("Ayhan'ın aylık gelir bilgisi: {}", ayhan.aylik_gelir);

}

/*
Ayhan'ın bilgileri: Calisanlar { ad: "Ayhan", id: 1001, eposta: "ayhan@company.com", birim: "Ödemeler", aylik_gelir: 48500.0 }
Ayhan'ın aylık gelir bilgisi: 48500
*/
