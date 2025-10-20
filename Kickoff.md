
# Proyecto 1 Thread city

**Integrantes:** Joshua Solís, Ion Dolanescu

## 1. Introducción

El proyecto consiste en hacer una simulación de la ciudad **Thread city**, la base del funcionamiento de este lugar es el manejo de hilos similar al del sistema operativo.  
La idea general del proyecto es implementar la biblioteca **pthreads** de GNU pero en **Rust**, nuestra versión llamada **mypthreads** va a implementar varias funciones básicas.  
Esto nos permitirá realizar distintos algoritmos como **Round Robin**, **Lottery** y **Tiempo Real**.  
Una vez tengamos esto implementado, generamos la simulación de la ciudad donde diversos hilos representarán carros, ambulancias, barcos y otros, todas actuando acorde a la política de planificación establecida.

### Funciones básicas por implementar:

- `my_thread_create`
- `my_thread_end`
- `my_thread_yield`
- `my_thread_join`
- `my_thread_detach`
- `my_mutex_init`
- `my_mutex_destroy`
- `my_mutex_lock`
- `my_mutex_unlock`
- `my_mutex_trylock`

### Estrategia de desarrollo:

1. Investigar el funcionamiento de pthreads, cómo se comporta y cómo fue implementado en C.
2. Implementar las funciones de la forma más similar posible pero en Rust para nuestra biblioteca pthreads.
3. Implementar los tres schedulers (**RoundRobin**, **Lottery**, **RealTime**) y la función `my_thread_chsched` para cambiar dinámicamente el tipo de scheduler de un hilo.
4. Hacer una versión básica de la interfaz gráfica ciudad con los elementos estáticos.
5. Posteriormente hacer los elementos dinámicos para ver cómo se comporta con los alrededores.
6. Pruebas de cada scheduler y pruebas funcionales de la simulación (casos críticos como entrega a planta nuclear).
7. Documentar diseño, decisiones y bitácoras de trabajo.

### Métodos:
- `my_thread_chsched`

---

## 2. Ambiente de desarrollo

- **Lenguaje:** Rust  
- **Sistema operativo:** Ubuntu  
- **Entorno de desarrollo:** RustRover  
- **Control de versiones:** Github repositorio  

### Librerías:

- `std::thread`, `std::sync` para bases de concurrencia
- `gtk-rs` para animaciones gráficas

### Compilación y debugging:

- `Cargo` para compilación y gestión de dependencias
- `gdb` o `lldb` para depuración de bajo nivel
- `println!` instrumentado para debugging lógico

### Ejecución:
Compilado para arquitectura **x86-64 Linux**

### Flujo de trabajo:
- `main`: rama estable y funcional donde vamos a hacer la mayoría de commits.

### Control de versiones:
**Proyecto1ThreadCity**, link al repositorio para el control de versiones:  
[https://github.com/Josh-SolisF/Proyecto1ThreadCity.git](https://github.com/Josh-SolisF/Proyecto1Thread


![alt text]([http://url/to/img.png](https://github.com/Josh-SolisF/Proyecto1ThreadCity/blob/main/Thread%20City.png))




