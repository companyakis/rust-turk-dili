fn main() {
  
    let ay:u8 = 5;

    match ay {
        1 => { println!("Ocak"); }
        2 => { println!("Şubat"); }
        3 => { println!("Mart"); }
        4 => { println!("Nisan"); }
        5 => { println!("Mayıs"); }
        6 => { println!("Haziran"); }
        7 => { println!("Temmuz"); }
        8 => { println!("Ağustos"); }
        9 => { println!("Eylül"); }
        10 => { println!("Ekim"); }
        11 => { println!("Kasım"); }
        12 => { println!("Aralık"); }
        _ => { println!("Lütfen, 1 ile 12 arasında bir sayı kullanın!"); }
    }

}


