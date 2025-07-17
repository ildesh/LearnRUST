// ----------------------------------------------- //
// Introduzione alle Tuple in Rust
//
// Una tupla è una collezione eterogenea (cioè può contenere tipi diversi)
// di valori ordinati. Una tupla ha una lunghezza fissa.
// Può contenere numeri, booleani, stringhe, altre tuple ecc.
//
// Le tuple possono essere destrutturate (assegnate a più variabili),
// oppure si può accedere ai singoli elementi tramite `nome_tupla.indice`.
// ----------------------------------------------- //

fn main() {

    // Creiamo una tupla
    let data = (10, 3.5, false); // Questa è una tupla e contiene all'interno un intero, un numero con la virgola mobile e un booleano.
    println!("data= {:?}", data); // questa riga ci mostrerà a video data= (10, 3.5, false);

    // Se vogliamo possiamo anche identificare i tipi di valori che inseriamo all'interno della tupla dobbiamo:
    let data1 : (i8, f32, bool) =(45, 7.5, true);
    println!("data= {:?}", data1);

    // ---------------------------- //
    // Destrutturazione: assegnare gli elementi della tupla a variabili separate
    // Possiamo anche accedere ai singoli valori nei seguenti modi:
    let (n,d,b) = data;
    println!("{n}, {d}, {b}.");

    let (n1,d1,b1) = data1;
    println!("{n1}, {d1}, {b1}.");
    println!("// ---------------------------- //");


    // oppure
    // Accesso diretto agli elementi tramite indice (zero-based)
    let first_data = data.0;
    let second_data = data.1;
    let third_data = data.2;

    let first_data1 = data1.0;
    let second_data1 = data1.1;
    let third_data1 = data1.2;

    println!("Data:");
    println!("  - {first_data}, \n  - {second_data}, \n  - {third_data}.");
    println!("// ---------------------------- //");
    println!("Data1:");
    println!("  - {first_data1}, \n  - {second_data1}, \n  - {third_data1}.");
    println!("// ---------------------------- //");

    // ---------------------------- //
    // Tupla vuota: tipo unitario
    let empty: () = (); 
    println!("Tupla vuota (tipo unit): {:?}", empty);
    // Il tipo `()` è chiamato "unit" ed è usato per rappresentare il nulla (simile a `void` in altri linguaggi)

    // ---------------------------- //
    // Esempio con una tupla di coordinate
    let coordinates: (f32, f32) = (12.5, -8.3);
    println!("Coordinate: ({}, {})", coordinates.0, coordinates.1);

    // Possiamo anche destrutturarla:
    let (x, y) = coordinates;
    println!("x = {x}, y = {y}");
}

