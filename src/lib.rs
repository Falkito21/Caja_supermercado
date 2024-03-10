pub mod compra {
    #[derive(Debug)]
    pub struct Item {
        pub nombre: String, // Nombre del item 
        pub precio_unitario: f32, // precio unitario del item
        pub cantidad: f32, // cantidad a comprar del item
    }

    pub fn agregar_item(items_compra: &mut Vec<Item>, item: Item) {
        // agregara un item a un vector con todos los items de la compra
        items_compra.push(item);
    }

    pub fn quitar_item(items_compra: &mut Vec<Item>, indice: usize) {
        // quitara un item del array a partir de un indice
        items_compra.remove(indice);
    }

    pub fn mostrar_items(items_compra: &Vec<Item>) {
        // mostrando los items y le indice 
        for (index, item) in items_compra.iter().enumerate() {
            let subtotal = item.cantidad * item.precio_unitario;
            println!("[{}]. {} - Cantidad: {} - Precio U: ${} - Subtotal: ${}", index, item.nombre, item.cantidad, item.precio_unitario, subtotal);
        }
    }

    pub fn total_compra(items_compra: &Vec<Item>) -> f32 {
        // devolvera el total a pagar de todos los items de la compra
        let mut total_compra: f32 = 0.0;
        for item in items_compra {
            total_compra = total_compra + (item.cantidad * item.precio_unitario);
        }
        
        //redondeando a dos decimales
        let y = 10i32.pow(2) as f32;
        total_compra = (total_compra * y).round() / y;

        total_compra
    }
}