// ================================================================================= //
// ================================================================================= //

// Per questo progetto, creeremo un semplice gioco di indovinare il numero in Rust.
// Per prima cosa, dobbiamo importare le librerie necessarie.

// ================================================================================= //
// ================================================================================= //

// 1. Importa la libreria per generare numeri casuali
    // i "crates" o librerie esterne sono gestite tramite Cargo, il gestore di pacchetti di Rust.
use rand::Rng; 

// 2. Importa la libreria per gestire l'input/output standard
use std::io; 

// 3. Importa la libreria per confrontare valori, utile per confrontare il numero indovinato con il numero segreto.
use std::cmp::Ordering;


// Quando definiamo una libreria dobbiamo aggiungere quest'ultima all'interno del progetto Cargo.toml con il seguente comando: // cargo add rand
// Il comando aggiunge la libreria all'interno del nostro file Cargo.toml che verrà messo nelle dipendenze del progetto, "[dependencies], che sono le librerie di cui il nostro progetto ha bisogno".
// Ora possiamo iniziare a scrivere il nostro gioco di indovinare il numero.

fn main() {
    // First part
    println!("Guess the number!"); // Stampa un messaggio di benvenuto al gioco.

    let secret_number: u32 = rand::rng().random_range(1..=100); // Genera un numero casuale tra 1 e 100.
    println!("The secret number is: {}", secret_number); // Stampa il numero segreto;

    // Second part
    loop{
        // 1- Iniziamo un ciclo infinito per permettere all'utente di indovinare il numero.
            // Utilizzeremo la libreria `std::io` per leggere l'input dell'utente dalla console.
            // Il ciclo continuerà a chiedere all'utente di indovinare il numero finché non deciderà di interrompere il programma manualmente.

        println!("Please input your guess: "); // Chiede all'utente di inserire un numero.
        let mut guess: String = String::new(); // Crea una stringa vuota per memorizzare il tentativo dell'utente.
        io::stdin().read_line(&mut guess).expect("Failed to read line"); // Legge l'input dell'utente dalla console.

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // Converte l'input in un numero intero.
        println!("You guessed: {}", guess); // Stampa il numero indovinato dall'utente.

        // 2- A questo punto, potremmo aggiungere logica per confrontare il numero indovinato con il numero segreto e fornire feedback all'utente.
            // Per ora ci concentriamo solo sulla lettura dell'input e utilizzeremo la libreria `std::cmp::Ordering` per confrontare i numeri.

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"), // Se il numero indovinato è minore del numero segreto, stampa un messaggio che dice "Too small!".

            Ordering::Greater => println!("Too big!"), // Se il numero indovinato è maggiore del numero segreto, stampa un messaggio che dice "Too big!".

            Ordering::Equal => {
                println!("You win!"); // Se l'utente indovina il numero, stampa un messaggio di vittoria.
                break; // Esce dal ciclo, terminando il gioco.

            }
        }
    }
}

// Analizziamo il codice:
// ======================================================================================================================================================= //

// Prima parte:
    // - `use rand::Rng;` importa la libreria per generare numeri casuali.
    // - `let secret_number: u32` dichiara una variabile `secret_number` di tipo `u32` (unsigned 32-bit integer) che conterrà il numero segreto.
    // - `rand::rng().random_range(1..=100)` genera un numero casuale tra 1 e 100, dove `rng` significa "random number generation".
    // - `println!("The secret number is: {}", secret_number);` stampa il numero segreto sulla console, utilizzando `{}` come segnaposto per il valore

// ====================================================================================================================================================================== //

// Seconda parte:
    // - `loop` inizia un ciclo infinito, che continuerà a chiedere all'utente di indovinare il numero finché non verrà interrotto.
    // - `println!("Please input your guess: ");` stampa un messaggio per chiedere all'utente di inserire un numero.

    // - `let mut guess: String = String::new();` crea una stringa vuota chiamata `guess` per memorizzare l'input dell'utente.
        // - `let mut` indica che la variabile `guess` è mutabile (mut == mutable), quindi possiamo modificarla in seguito.
    
    // - `io::stdin().read_line(&mut guess).expect("Failed to read line");` legge l'input dell'utente dalla console e lo memorizza nella variabile `guess`.
        // - `io::stdin()` accede all'input standard (la tastiera).
        // - `read_line(&mut guess)` legge una riga di input e la memorizza nella variabile `guess`.
        // - `expect("Failed to read line")` gestisce eventuali errori durante la lettura dell'input, mostrando un messaggio se qualcosa va storto.

    // - `let guess: u32 = guess.trim().parse().expect("Please type a number!");` converte l'input in un numero intero, rimuovendo eventuali spazi vuoti con `trim()`.
        // - `guess.trim()` rimuove gli spazi vuoti all'inizio e alla fine della stringa.
        // - `parse()` tenta di convertire la stringa in un numero intero. IL PARSING è un metodo che cerca di convertire una stringa in un tipo specifico, in questo caso `u32`.
        // - `expect("Please type a number!")` gestisce eventuali errori di conversione, mostrando un messaggio se l'input non è un numero valido. L'uso di `EXPECT` è comune in Rust per gestire errori in modo semplice e diretto.

    // - `println!("You guessed: {}", guess);` stampa il numero indovinato dall'utente.

// ====================================================================================================================================================================== //