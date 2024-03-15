use std::io::stdin;
use caja_supermercado::compra::{agregar_item, mostrar_items, pagar_compra, quitar_item, total_compra, Item};
use caja_supermercado::pago::{MetodoDePago, ResultadoPago};

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

fn manejar_realizar_pago(items_compra: &mut Vec<Item>)
{
    let monto_a_pagar = total_compra(items_compra);

    println!("Monto a pagar: ${}", monto_a_pagar);
    println!("Selecciona el metodo de pago.");
    println!("1. En Efectivo");
    println!("2. Tarjeta");
    println!("3. Transferencia Bancaria");

    let mut recibido_del_cliente = 0.0;
    let mut tarjeta = String::from("N/A");

    //obtenemos la opcion que selecciona el usuario
    let mut opcion: String = String::new();
    stdin().read_line(&mut opcion).unwrap();
    //limpiando el input de la terminal 
    let opcion_seleccionada = opcion.replace("\n", "").replace("\r", "").parse::<usize>().unwrap();

    let metodo_de_pago = match opcion_seleccionada
    {
        1 => MetodoDePago::Efectivo
        ,2 => MetodoDePago::Tarjeta
        ,3 => MetodoDePago::TransferenciaBancaria
        ,_ => MetodoDePago::Efectivo
    };

    if opcion_seleccionada == 1 
    {
        // El metodo de pago es efectivo, necesitamos saber cuanto recvibimos del cliente 
        println!("Monto Recibido del cliente: ");
        let mut recibido: String = String::new();
        stdin().read_line(&mut recibido).unwrap();
        recibido_del_cliente = recibido.replace("\n", "").replace("\r", "").parse::<f32>().unwrap();
    }

    if opcion_seleccionada == 2 
    {
        println!("Num. De Tarjeta:");
        //El metodo de pago es contarjeta , necesitamos saber el numero
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        tarjeta = input.replace("\n", "").replace("\n", "");
    }

    let resultado_del_pago: ResultadoPago = pagar_compra(metodo_de_pago, monto_a_pagar, recibido_del_cliente, &tarjeta);

    if resultado_del_pago.fue_exitoso
    {
        println!("El pago fue exitoso");
        println!("Metodo de pago: {}", resultado_del_pago.metodo_pago);
        println!("Cambio: ${}", resultado_del_pago.metodo_pago);
    }
    else 
    {
        println!("Hubo un problema procesando el pago, intentalo de nuevo");
    }


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
            5 => manejar_realizar_pago(&mut item_compra), // realizar el pago 
            6 => break, // terminando el programa
            _ => println!("Opcion Invalida") // la opcion no es valida, el programa continua 
        };

    }

    println!("Programa Finalizado");   

}
