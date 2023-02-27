/*
# 함수

함수 이름 = snake_case 형태
*/
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}


/*
# 여러개의 리턴값

함수는 튜플(tuple)을 리턴함으로써 여러개의 값을 리턴할 수 있다.
*/
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}


/*
# 리턴 X

함수에 리턴형을 지정하지 않은 경우 빈 tuple을 리턴하는데, 이는 unit이라고도 다.
*/
fn make_nothing() -> () {
    return ();
}
fn make_nothing2() {
    // 리턴 지정이 없다면 () 리턴
}


// main
fn main() {
    // # 함수
    println!("{}", add(12, 34));
    
    // # 여러개의 리턴값
    // 리턴 값의 튜플을 리턴
    let res = swap(12, 34);
    println!("{} {}", res.0, res.1);
    
    // 튜플을 두 변수명으로 분해
    let (a, b) = swap(34, 12);
    println!("{} {}", a, b);
    
    // # 리턴 X
    let t1 = make_nothing();
    let t2 = make_nothing2();
    // 반환값이 없기에 디버그하여 출력한다.
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}