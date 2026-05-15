/*FASE 1: */

/*
 *1.Documentación de Memoria: Lo que ocurre con option::take() es que,
 * toma el valor contenido dentro de un Option, dejando None en su lugar,
 * osea que es una operacion de movimiento donde se transfiere el ownership del valor contenido,
 *  es decir, en codigo seria que lo que ocurre con la propiedad ownership usando el option::take()
 * es que toma ownership del nodo izquierdo, y.izquierdo queda None, lo mismo va logica se usa para el derecho,
 *  asi que no crea clones, porque los mueve.
 */

/*2.Prueba de Escritorio:
*
*   insercion de 5000
*   identificacion: NO HAY ROTACION
*   arbol:   5000
*  --------------------------------------
*  insercion de 3000
*  identificacion:NO HAY ROTACION
*  arbol:     5000
               /
           3000
*  --------------------------------------
* insercion de 2000
* identificacion: ROTACION SIMPLE
* arbol:        3000
               /    \
             2000   5000
* --------------------------------------
* insercion de 4000
* identificacion: NO HAY ROTACION
* arbol:
*              3000
              /    \
           2000   5000
                   /
               4000
* --------------------------------------
* insercion de 3500
* identificacion: ROTACION DOBLE
* arbol:
* se inserta:
*               3000
               /     \
           2000       5000
                    /
                 4000
                  /
               3500
 ya con rotacion doble:

               3000
              /    \
          2000     4000
                  /    \
               3500   5000
* --------------------------------------
* insercion de 6000
* identificacion: NO HAY ROTACION
* arbol:
*              3000
               /    \
           2000     4000
                   /    \
                3500   5000
                           \
                           6000
*/

/*3.Concepto de Box: usamos Box<nodo> porque en rust, todas las estructuras deben tener un
 * tamaño conocido en tiempo de compilación, si no lo usuramos seria: un nodo que contiene otro nodo
 * crearía un tamaño infinito, en cambio con box se resuelve con un puntero de tamaño fijo al heap,
 *  ademas que solo mueve el puntero, no se esta copiando
 */

/*FASE 2: */

#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Esta será nuestra clave (key)
}

#[derive(Debug)]
struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }

    // --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---
    fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
        nodo.as_ref().map_or(0, |n| n.altura)
    }

    fn actualizar_altura(nodo: &mut Nodo) {
        nodo.altura = 1 + std::cmp::max(
            Nodo::obtener_altura(&nodo.izquierdo),
            Nodo::obtener_altura(&nodo.derecho),
        );
    }

    fn obtener_balance(nodo: &Nodo) -> i32 {
        Nodo::obtener_altura(&nodo.izquierdo) - Nodo::obtener_altura(&nodo.derecho)
    }

    fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
        let mut x = y.izquierdo.take().expect("Error de radar");
        y.izquierdo = x.derecho.take();
        Nodo::actualizar_altura(&mut y);
        x.derecho = Some(y);
        Nodo::actualizar_altura(&mut x);
        x
    }

    fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
        let mut y = x.derecho.take().expect("Error de radar");
        x.derecho = y.izquierdo.take();
        Nodo::actualizar_altura(&mut x);
        y.izquierdo = Some(x);
        Nodo::actualizar_altura(&mut y);
        y
    }

    fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
        let mut nodo = match nodo_opt {
            None => return Box::new(Nodo::nuevo(vuelo)),
            Some(n) => n,
        };

        let altitud_vuelo = vuelo.altitud;

        if altitud_vuelo < nodo.vuelo.altitud {
            nodo.izquierdo = Some(Nodo::insertar(nodo.izquierdo.take(), vuelo));
        } else if altitud_vuelo > nodo.vuelo.altitud {
            nodo.derecho = Some(Nodo::insertar(nodo.derecho.take(), vuelo));
        } else {
            return nodo;
        }

        Nodo::actualizar_altura(&mut nodo);
        let balance = Nodo::obtener_balance(&nodo);

        if balance > 1 && altitud_vuelo < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
            return Nodo::rotar_derecha(nodo);
        }
        if balance < -1 && altitud_vuelo > nodo.derecho.as_ref().unwrap().vuelo.altitud {
            return Nodo::rotar_izquierda(nodo);
        }
        if balance > 1 && altitud_vuelo > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(Nodo::rotar_izquierda(hijo_izq));
            return Nodo::rotar_derecha(nodo);
        }
        if balance < -1 && altitud_vuelo < nodo.derecho.as_ref().unwrap().vuelo.altitud {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(Nodo::rotar_derecha(hijo_der));
            return Nodo::rotar_izquierda(nodo);
        }

        nodo
    }
    /*esta es la funcion de la fase 2..2.1.firma y 2.2.Restriccion */
    fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
        match nodo {
            None => None,
            Some(nodo_ref) => {
                if altitud == nodo_ref.vuelo.altitud {
                    Some(&nodo_ref.vuelo)
                } else if altitud < nodo_ref.vuelo.altitud {
                    Nodo::buscar_vuelo(&nodo_ref.izquierdo, altitud)
                } else {
                    Nodo::buscar_vuelo(&nodo_ref.derecho, altitud)
                }
            }
        }
    }

    /*estas son las funciones de fase 3 ...3.1.firma y 3.2.desafio y 3.3.validacion*/
    fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
        match nodo_opt {
            None => None,
            Some(mut nodo) => {
                if altitud < nodo.vuelo.altitud {
                    nodo.izquierdo = Nodo::eliminar_vuelo(nodo.izquierdo.take(), altitud);
                } else if altitud > nodo.vuelo.altitud {
                    nodo.derecho = Nodo::eliminar_vuelo(nodo.derecho.take(), altitud);
                } else {
                    if nodo.izquierdo.is_none() {
                        return nodo.derecho.take();
                    } else if nodo.derecho.is_none() {
                        return nodo.izquierdo.take();
                    } else {
                        if let Some(mut nodo_maximo) = Nodo::extraer_maximo(&mut nodo.izquierdo) {
                            std::mem::swap(&mut nodo.vuelo, &mut nodo_maximo.vuelo);
                            // Devolver el nodo_maximo a su lugar
                            nodo.izquierdo = Some(nodo_maximo);
                            // Eliminar el nodo duplicado (ahora en la posición del predecesor)
                            nodo.izquierdo =
                                Nodo::eliminar_vuelo(nodo.izquierdo.take(), nodo.vuelo.altitud);
                        } else {
                            return None;
                        }
                    }
                }
                Nodo::actualizar_altura(&mut nodo);
                return Nodo::balancear_nodo(nodo);
            }
        }
    }

    fn extraer_maximo(nodo: &mut Option<Box<Nodo>>) -> Option<Box<Nodo>> {
        match nodo {
            Some(n) if n.derecho.is_none() => nodo.take(),
            Some(n) => Nodo::extraer_maximo(&mut n.derecho),
            None => None,
        }
    }

    fn balancear_nodo(nodo: Box<Nodo>) -> Option<Box<Nodo>> {
        let mut nodo = nodo;
        let balance = Nodo::obtener_balance(&nodo);

        if balance > 1 && Nodo::obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
            return Some(Nodo::rotar_derecha(nodo));
        }

        if balance > 1 && Nodo::obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
            let hijo_izq = nodo.izquierdo.take().unwrap();
            nodo.izquierdo = Some(Nodo::rotar_izquierda(hijo_izq));
            return Some(Nodo::rotar_derecha(nodo));
        }

        if balance < -1 && Nodo::obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
            return Some(Nodo::rotar_izquierda(nodo));
        }

        if balance < -1 && Nodo::obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
            let hijo_der = nodo.derecho.take().unwrap();
            nodo.derecho = Some(Nodo::rotar_derecha(hijo_der));
            return Some(Nodo::rotar_izquierda(nodo));
        }

        Some(nodo)
    }

    //fase 4
    /*yo elegi la opcion B */
    fn vuelo_mas_bajo(nodo: &Option<Box<Nodo>>) -> Option<&Vuelo> {
        let mut actual = nodo.as_ref()?;

        while let Some(ref izquierdo) = actual.izquierdo {
            actual = izquierdo;
        }

        Some(&actual.vuelo)
    }
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000),
        ("UA456", 3000),
        ("IB101", 2000),
        ("AF999", 4000),
        ("TA222", 3500),
        ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo {
            id: id.to_string(),
            altitud: alt,
        };
        radar = Some(Nodo::insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");

    //FASE 2
    println!("\n--- FASE 2 ---");

    let alturas_buscar = [4000, 2000, 4500];

    for &alt in &alturas_buscar {
        match Nodo::buscar_vuelo(&radar, alt) {
            Some(vuelo) => println!("Encontrado: {} a {} pies", vuelo.id, vuelo.altitud),
            None => println!("No encontrado: {} pies", alt),
        }
    }

    // FASE 3
    println!("\n--- FASE 3 ---");
    let aterrizajes = [4000, 2000, 6000];
    for &alt in &aterrizajes {
        println!("\nAterrizando vuelo a {} pies...", alt);
        radar = Nodo::eliminar_vuelo(radar.take(), alt);

        match Nodo::buscar_vuelo(&radar, alt) {
            Some(vuelo) => println!("ERROR: {} aún permanece en radar", vuelo.id),
            None => println!("Vuelo a {} pies eliminado correctamente", alt),
        }
        // Muestro estructura del árbol, por la restriccion de la fase 3 de Validación
        println!("Árbol después de eliminar {}: {:?}", alt, radar);
    }

    println!("\n--- Vuelos en el radar después de aterrizajes ---");
    let vuelos_restantes = [3000, 3500, 5000];
    for &alt in &vuelos_restantes {
        match Nodo::buscar_vuelo(&radar, alt) {
            Some(vuelo) => println!("{} volando a {} pies", vuelo.id, vuelo.altitud),
            None => println!("No hay vuelo a {} pies", alt),
        }
    }

    //FASE 4
    println!("\n--- FASE 4: ELEGI OPCION B ---");
    match Nodo::vuelo_mas_bajo(&radar) {
        Some(vuelo) => println!(
            "EMERGENCIA: Vuelo más cercano a tierra es {} a {} pies",
            vuelo.id, vuelo.altitud
        ),
        None => println!("No hay vuelos en el radar"),
    }
}
