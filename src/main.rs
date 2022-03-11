use std::io;
//Генерирование n-го числа Фибоначчи.


fn main() {
    let mut n_max = String::new();
    //переменная для индекса нужного числа

    let mut fibonacc= vec![0, 1];

    loop {
        println!("Please enter your number");

        io::stdin()
            .read_line(&mut n_max)
            .expect ("Failed to read line!!!");

        if n_max.find("stop") != None {
            println!("Good bay!");
            break
        } else {
            let n: usize = match n_max.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
            n_max.clear();
            // println!("{:#?}", number_generation(n, &mut  fibonacc));
        }
    }
}


fn number_generation (n_max: usize, fibonacci: &mut Vec<i32>) { // ->i32
    //в случае если нужного числа нет, то вектор
    // дополяется до него, если есть то возвращает
    // нужное число
    if n_max >= fibonacci.len() {
        let mut n: usize = fibonacci.len();
        while n <= n_max {
            let x: i32 = fibonacci[n - 1] + fibonacci[n - 2];
            //println!("{}", x);
            fibonacci.push(x);
            n += 1;
        }
    }
    //let x: usize = fibonacci.len() - 1;
    println!("{:#?}", fibonacci);
    //fibonacci.to_vec()[n_max]


}



