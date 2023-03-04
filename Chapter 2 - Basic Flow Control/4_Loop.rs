/*
# 무한 반복문

무한 반복문 키워드인 loop를 사용한다.
break : 반복문 종료
*/
fn main() {
    let mut cnt = 0;
    loop {
        cnt += 1;
        if cnt == 5 {
            break;
        } else {
            print(cnt)
        }
    }
    println!("end / cnt : {}", cnt);
}
