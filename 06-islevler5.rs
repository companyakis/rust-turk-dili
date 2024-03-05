//kullanıcı tanımlı işlev ve ödünçlük için farklı bir örnek

fn gunaydin_de(kisi: &mut String) {
    println!("{}", kisi);

    kisi.push_str(", günaydın sana!");

    println!("{}", kisi);
}

fn main() {
  
    let mut ben: String = "Mustafa Büyükdereli".to_string();

    gunaydin_de(&mut ben);

    println!("{}", ben);
}

/*
Mustafa Büyükdereli
Mustafa Büyükdereli, günaydın sana!
Mustafa Büyükdereli, günaydın sana!

*/
