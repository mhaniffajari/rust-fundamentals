// function untuk menjumlahkan angka
//use std::io;
fn jumlah(angka:&[i32]) -> i32
{
   let mut hasil = 0;
   for number in angka{
      hasil += number;
   }
   hasil
}

fn main(){
   //println!("Enter a word please");
   //let mut name= String::new();
   //io::stdin().read_line(&mut name).expect("Failed to hear you");
   let angka = [1,2,3];
   let hasil = jumlah(&angka);
   println!("Hasil penjumlahan adalah {}", hasil);
}