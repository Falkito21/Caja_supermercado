pub mod pago
{
    pub enum MetodoDePago 
    {
        Efectivo 
        ,Tarjeta
        ,TransferenciaBancaria
    }

    pub struct ResultadoPago 
    {
        pub metodo_pago: String //La descripcion del metodo de pago 
        ,pub fue_exitoso: bool // true si el pago se pudo hacer o false sino
        ,pub cambio: f32 //cambio a devolver al cliente
    }

    pub fn pagar(metodo_de_pago: MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> ResultadoPago
    {
        // metodo de pago -> es la forma de pago elegida por el clietne
        //monto_a_pagar -> total a pagar de la compra 
        //recibido_del_cliente -> cantidad de dinero recibida del  cliente
        //tarjeta -> numero de tarjeta del cliente 

        //Ahora, dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas
        //lo podemos hacer porque estan dentro del mismo alcance de este metodo.
        let resultado = match metodo_de_pago
        {
            MetodoDePago::Efectivo => pago_en_efectivo(monto_a_pagar, recibido_del_cliente)
            ,MetodoDePago::Tarjeta => pago_con_tarjeta(monto_a_pagar, tarjeta)
            ,MetodoDePago::TransferenciaBancaria => pago_por_transferencia_bancaria(monto_a_pagar)
        };
        resultado
    }

    fn pago_en_efectivo(monto_a_pagar: f32, recibido_del_cliente: f32) -> ResultadoPago 
    {
        //Si el resultado es en efectivo, se calcula el cambio a devolver al cliente
        ResultadoPago
        {
            metodo_pago: String::from("En Efectivo")
            ,fue_exitoso: true
            ,cambio: recibido_del_cliente - monto_a_pagar
        }
    }

    fn pago_con_tarjeta(monto_a_pagar: f32, numero_tarjeta: &str) -> ResultadoPago 
    {
        //Si el pago es con tarjeta, simularemos el resultado

        //Aca se estaria procesando todo aquello critico a nivel de seguridad
        println!("Realizando el pago con el servicio de tarjeta credito/debito");
        println!("Monto a pagar: {}", monto_a_pagar);
        println!("Tarjeta: {}", numero_tarjeta);

        //Simulamos el resultado
        ResultadoPago 
        {
            metodo_pago: String::from("Tarjeta")
            ,fue_exitoso: true
            ,cambio: 0.0
        }
    }

    fn pago_por_transferencia_bancaria(monto_a_pagar: f32) -> ResultadoPago 
    {
        //Si el pago es via transferencia, simularemos que solamente necesitamos la cuenta del supermercado
        //simularemos el resultado de transferencia

        //Aca se estaria procesando todo aquello critico a nivel de seguridad
        println!("Realizando las conexiones y transferencias con el banco");
        println!("Monto a pagar: {}", monto_a_pagar);
        println!("Cuenta bancaria secreta: 123456789-0");

        //simulamos el resultado
        ResultadoPago 
        {
            metodo_pago: String::from("Transferencia Bancaria")
            ,fue_exitoso: true
            ,cambio: 0.0
        }
    }
}

pub mod compra 
{
    use crate::pago;
    #[derive(Debug)]
    pub struct Item 
    {
        pub nombre: String, // Nombre del item 
        pub precio_unitario: f32, // precio unitario del item
        pub cantidad: f32, // cantidad a comprar del item
    }

    pub fn agregar_item(items_compra: &mut Vec<Item>, item: Item) 
    {
        // agregara un item a un vector con todos los items de la compra
        items_compra.push(item);
    }

    pub fn quitar_item(items_compra: &mut Vec<Item>, indice: usize) 
    {
        // quitara un item del array a partir de un indice
        items_compra.remove(indice);
    }

    pub fn mostrar_items(items_compra: &Vec<Item>) 
    {
        // mostrando los items y le indice 
        for (index, item) in items_compra.iter().enumerate() 
        {
            let subtotal = item.cantidad * item.precio_unitario;
            println!("[{}]. {} - Cantidad: {} - Precio U: ${} - Subtotal: ${}", index, item.nombre, item.cantidad, item.precio_unitario, subtotal);
        }
    }

    pub fn total_compra(items_compra: &Vec<Item>) -> f32 
    {
        // devolvera el total a pagar de todos los items de la compra
        let mut total_compra: f32 = 0.0;
        for item in items_compra 
        {
            total_compra = total_compra + (item.cantidad * item.precio_unitario);
        }
        
        //redondeando a dos decimales
        let y = 10i32.pow(2) as f32;
        total_compra = (total_compra * y).round() / y;

        total_compra
    }

    pub fn pagar_compra(metodo_de_pago: pago::MetodoDePago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> pago::ResultadoPago
    {
        //El parametro metodo_de_pago es la forma de pago elegida por el cliente 
        // monto_a_pagar es el total a pagar de la compra 
        // recibido_del_cliente si el pago es en efectivo o por transferencia, no es necesario
        // tarjeta, es el numero de tarjeta del cliente

        // dependiendo del metodo de pago elegido por el cliente, invocamos las funciones privadas
        // lo podemos hacer ya que estamos dentro del mismo alcance del metodo 
        
        let resultado = pago::pagar(metodo_de_pago, monto_a_pagar, recibido_del_cliente, tarjeta); 

        resultado
    }
}