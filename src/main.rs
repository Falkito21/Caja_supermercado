use std::io::stdin;
use caja_supermercado::compra::{Item, agregar_item, quitar_item, mostrar_items, total_compra};
fn mostrar_menu() {
    println!("OPCIONES: ");
    println!("1. Agregar Item");
    println!("2. Quitar Item");
    println!("3. Mostrar Item");
    println!("4. Total a Pagar");
    println!("5. Realizar Pago");
    println!("6. Cancelar Compra y Salir");
    println!("Selecciona una opcion");
}

fn manejar_agregar_item(items_compra: &mut Vec<Item>)
{
    //solicito al usuario que registre los datos del item 
    let mut input: String = String::new();
    println!("Escribe los detalles del Item");

    println!("NOMBRE: ");
    stdin().read_line(&mut input).unwrap();
    let nombre = input.replace( "\n", "").replace("\r", "");

    println!("CANTIDAD: ");
    input = String::new();
    stdin().read_line(&mut input).unwrap();
    let cantidad = input.replace( "\n", "").replace("\r", "").parse::<f32>().unwrap();

    println!("PRECIO UNITARIO: ");
    input = String::new();
    stdin().read_line(&mut input).unwrap();
    let precio_unitario = input.replace( "\n", "").replace("\r", "").parse::<f32>().unwrap();

    //creando el item con la estructura que importamos de nuestro modulo 
    let item: Item = Item {
        nombre
        ,precio_unitario
        ,cantidad
    };

    //agregamos el item a la compra 
    agregar_item(items_compra, item);

    println!("Item Agregado");
}

fn manejar_quitar_item(items_compra: &mut Vec<Item>)
{
    // mostrando los items para que el usario pueda saber cula quitar
    // REUTILIZANDO MODULO 
    println!("Selecfciona el indice que quieres quitar");
    mostrar_items(items_compra);
    
    // obteniendo el item a limpiar 
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    //limpiamos el input 
    let indice = input.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

    // eliminamos el item utlizando la funcion dentro del modulo de compra 
    quitar_item(items_compra, indice);
    println!("Item Eliminado");

    
}

fn manejar_realizar_pago()
{
    println!("En construccion");
}
fn main() {
    // creamos un vector para llevar el registro de los items de la compra 
    let mut item_compra: Vec<Item> = Vec::new();

    // iniciamos en loop en el cual vamos a preguntar al usuario la accion a realizar
    loop{
        mostrar_menu();

        //obtenemos la opcion que selecciona el usuario
        let mut opcion: String = String::new();
        stdin().read_line(&mut opcion).unwrap();

        // limpiando el input de la terminal 
        let opcion_seleccionada = opcion.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

        match opcion_seleccionada {
            1 => manejar_agregar_item(&mut item_compra), // agregar un item
            2 => manejar_quitar_item(&mut item_compra), // quitar un item
            3 => mostrar_items(&item_compra), // mostrar todos los items y sus indices
            4 => println!("Total a pagar: ${}", total_compra(&item_compra)), // mostrando el total a pagar 
            5 => manejar_realizar_pago(), // realizar el pago 
            6 => break, // terminando el programa
            _ => println!("Opcion Invalida") // la opcion no es valida, el programa continua 
        };

    }

    println!("Programa Finalizado");   

}
