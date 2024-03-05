fn main() {

    //sayıları tersten yazdıralım
    let mut sayilar: [u8; 6] = [3, 15, 100, 90, 12, 27];

    sayilar.reverse();

    for sayi in sayilar {
        print!("{} ", sayi); //27 12 90 100 15 3 
    }

    println!("{:?}", sayilar); //[27, 12, 90, 100, 15, 3]
 
}

