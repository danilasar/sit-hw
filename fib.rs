use std::io;

// n - номер числа Фибоначчи
fn fib(n: u8) -> Result<(), String> {
    let dp:&mut [u128; 187] = &mut [0u128;187];
    if n == 0 {
        return Err(String::from("Ноль не является номером"));
    }
    if n > 187 {
        return Err(String::from("Программа способна вычислить только первые 187 чисел ряда"));
    }
    dp[0] = 0;
    println!("{}", dp[0]);
    dp[1] = 1;
    if n >= 2 {
        println!("{}", dp[1]);
    }
    let mut i:usize = 2usize;
    while i < usize::from(n) {
        dp[i] = dp[i - 1] + dp[i - 2];
        println!("{}", dp[i]);
        i += 1;
    }
    return Ok(());
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
            fib(n).unwrap_or_else(|error| {
                println!("{error}");
            });
        },
        Err(..) => println!("Это не число: {}", trimmed)
    };
}
