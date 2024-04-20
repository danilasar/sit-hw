use std::io;

// n - номер числа Фибоначчи, dp - массив с числами Фибоначчи (для ещё не вычисленных используется 0)
fn fib(n: u8, dp:&mut [u128; 184]) -> Result<u128, String> {
    if n == 0 {
        return Err(String::from("Ноль не является номером"));
    }
    if n > 186 {
        return Err(String::from("Программа способна вычислить только первые 186 чисел ряда"));
    }
    if n == 1 || n == 2 {
        return Ok(1);
    }
    let index:usize = usize::from(n - 3); // получаем индекс в массиве
    if dp[index] == 0 { // если число под этим номером ещё не вычислено, вычисляем
        dp[index] = fib(n - 1, dp).unwrap() + fib(n - 2, dp).unwrap();
    }
    return Ok(dp[index]);
}

// Макрос, создающий массив для динамики
macro_rules! fib {
    ($n:expr) => {fib($n, &mut [0u128;184])};
}

fn main() {
    println!("Введите номер желаемого числа из ряда Фибоначчи: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не могу читать входящий поток");
    let trimmed = input.trim(); // избавляемся от пробелов и прочей лабуды
    match trimmed.parse::<u8>() {
        Ok(n) => {
            match fib!(n) {
                Ok(i) => println!("{n}-е число Фибоначчи: {i}"),
                Err(err) => println!("{err}")
            }
        },
        Err(..) => println!("Это не число: {}", trimmed)
    };
}
