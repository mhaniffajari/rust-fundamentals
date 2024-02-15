// function untuk menjumlahkan angka dan menentukan bilangan genap atau ganjil

fn jumlah_ganjil_genap(numbers: &[i32]){
 let mut sum = 0;
 for number in numbers{
    sum += number;
 }
 println!("jumlah dari angka tersebut adalah: {}",sum);

 if sum % 2 == 0 {
    println!("Jumlah dari angka tersebut adalah genap")
 } else {
    println!("Jumlah dari angka tersebut adalah ganjil")
 }

}

fn main(){
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    jumlah_ganjil_genap(&numbers);
} 