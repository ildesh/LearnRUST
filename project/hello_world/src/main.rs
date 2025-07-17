// Questo è un semplice programma Rust che stampa "Hello, world!" sulla console.
// Puoi compilare ed eseguire questo programma con `cargo run` se hai il Cargo installato.

// Per installare Cargo basta che scriviamo cargo new "nome_del_progetto"
// poi entriamo nella cartella del progetto con `cd nome_del_progetto`
// e infine eseguiamo `cargo run` per compilare ed eseguire il programma.

fn main() {
    println!("Hello, world!"); // RICORDA: println! è una macro, non una funzione, quindi usiamo il punto esclamativo alla fine.
                               // Una macro in Rust è un modo per generare codice durante la compilazione, e `println!` è una macro che stampa una stringa sulla console.

    println!("Hello, Rust!"); // Possiamo anche stampare altre stringhe, come questa.
}

// Nota 1: In Rust, le stringhe sono immutabili per impostazione predefinita, quindi non possiamo modificarle dopo la loro creazione.
// Per modificare una stringa, dobbiamo dichiararla come mutabile usando la parola chiave `mut`.
// Ad esempio: let mut s = String::from("Hello"); s.push_str(", world!"); println!("{}", s);

// Nota 2: Inoltre, Rust ha un sistema di gestione della memoria molto sicuro, quindi non dobbiamo
// preoccuparci di gestire manualmente la memoria come in altri linguaggi come C o C++.

// Nota 3: Rust utilizza un sistema di proprietà e prestiti per garantire la sicurezza della memoria e prevenire i problemi di accesso concorrente.