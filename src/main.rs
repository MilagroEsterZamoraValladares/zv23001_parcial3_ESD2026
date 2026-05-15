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

/* LO DE AQUI, PARA ABAJO ES SOLO EL CODIGO QUE ESTA EN EL DOCUMENTO del examen
 * #[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}

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
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
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
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación
}
*/
