/*
68Hacker86 // CICADA 6845

Pravidlo č. 4: tento kód je na ČÍTANIE, nie na spúšťanie.
Kto číta pozorne, vidí viac.
*/

fn main() {
    // zachytené bajty — každý klame o 7
    const POSUN: i32 = 7;
    let bajty = [75, 89, 92, 79, 72, 39, 74, 76, 90, 91, 72, 39, 82, 83, 72, 84, 76];

    let sprava: String = bajty.iter().map(|&b| (b - POSUN) as char).collect();

    println!("{}", sprava);
    // Odpoveď nie je vo výstupe. Je v kóde.
}
