/*
# 배열

배열(array)은 고정된 길이로 된 모두 같은 자료형의 자료를 가진 collection

array의 자료형은 [T;N]로 표현
T는 원소의 자료형, N은 컴파일 타임에 주어지는 고정된 길이
*/

fn main() {
    let array: [i32; 4] = [1, 2, 3, 4]; // i32 자료형인 4칸의 배열 선언
    println!("{:?}", array);
    println!("array[3] : {}", array[3]);
}
