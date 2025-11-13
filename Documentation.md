Proyecto 1 Thread city
Integrantes: Joshua Solís, Ion Dolanescu

## 1.Introducción

El proyecto consiste en hacer una simulación de la ciudad Thread city, la base del
funcionamiento de este lugar es el manejo de hilos similar al del sistema operativo, pero
para esto debemos implementar la biblioteca pthreads de GNU pero en Rust, nuestra
versión llamada mypthreads va a implementar varias funciones básicas, estos nos permitirá
realizar distintos algoritmos como Round Robin, Lottery y Tiempo Real. Una vez tengamos
esto implementado generamos la simulación de la ciudad donde diversos hilos
representarán carros, ambulancias, barcos y otros, todas actuando acorde a la política de
planificación establecida.

## 2.Ambiente de desarrollo

- Lenguaje: Rust
- Sistema operativo: Ubuntu
- Entorno de desarrollo: RustRover
- Control de versiones: Github repositorio

## 3. Estructura de datos usadas para y funciones:

Paquete mypthreads: contiene toda la librería de hilos en espacio de usuario y el módulo de schedulers.

  - MyThread:
      - mod.rs: expone el módulo principal.
          - mypthread.rs: fachada de la API pública
      - myruntime.rs: runtime/coordinador; maneja creación, cambio de contexto, finalización, join/detach, y delega al scheduler.
        - mythread.rs: tipos de hilo
      - mythreadattr.rs: atributos de hilo
            - thread_state.rs: máquina de estados del hilo
      - mymutex.rs: implementación del mutex
      - mutexlockkind.rs: atributos/variedades de mutex
        - Códigos de salida/errores: mypthreadexits.rs
      - Módulo scheduler/: políticas de planificación.
  - Scheduler:
    - round_robin
      - mod.rs: Implementación sencillas del Round Robin
    - lottery
      - mod.rs: Implementación sencilla de lottery usando splitmix
    - real_time
       - mod.rs: Implementación sencilla de lottery usando splitmix
    - mod.rs
    - trait.rs
    - scheduler_type.rs
    - scheduler_param.rs
    
- Paquete thread-city: Contiene todo lo necesario para la simulación de la ciudad
  - city
      - mod.rs: expone el módulo de ciudad.
        - simulation_controller.rs: bucle de simulación y orquestación de ticks.
Crea entidades, avanza el tiempo, coordina con GUI y delega decisiones de movimiento al traffic_handler.
          - supply_kind.rs: tipos de suministros
          - traffic_handler.rs: árbitro central de tránsito. Implementa la política de avance en dos fases: recolecta intenciones de movimiento de los vehículos, verifica ocupación y reglas locales y consolida qué movimientos se concretan en cada tick.
  - cityblock
     - mod.rs: módulo raíz de bloques de la ciudad.
     - block.rs: bloque del mapa
     - block_type.rs: tipos de bloque
     - coord.rs: coordenadas y utilidades
     - map.rs: mapa/grade de la ciudad
     - transport_policy.rs: reglas de circulación por tipo de bloque
   - bridge/
      - mod.rs: expone la funcionalidad de puentes.
    - bridge_permission_enum.rs: estados/decisiones de permiso
          - control.rs: BridgeController: aplica la lógica de cada puente
        - traffic_light.rs: semáforos/timers del Puente 1
- dock
     - mod.rs: elementos del muelle; integra a los barcos y su relación con puentes
- nuclearplant/
   - mod.rs: fachada del submódulo de planta.
   - plant_status.rs: estado operacional
   - supply_spec.rs: especificaciones/SLAs de entrega
- road
        - mod.rs: modela tramos de carretera
- shopblock
      - mod.rs: comercios/destinos de vehículos
- water
  - mod.rs: río cuerpos de agua.
- GUI
   - mod.rs
   - main.rs: integración con GTK.
- vehicle/
  - mod.rs: expone el submódulo de vehículos.
  - vehicle.rs: estructura base de vehículo
    - vehicle_type.rs: enum de tipos
    - Subcarpetas por tipo con su mod.rs:
      - Ambulance/: comportamiento con prioridad
      - car/: vehículo estándar.
      - cargotruck/: logística de planta nuclear
      - Ship/: barcos
    - tests/: mod.rs, vehicle.rs, vehicle_type.rs: pruebas de creación/comportamiento básico por tipo.
- src/main.rs
  - Punto de entrada de la app de simulación. Inicializa mapa, controladores, vehículos iniciales, runtime/scheduler y GUI; arranca el bucle de simulación.


## 4. Funciones basado en archivos:


### **MyMutexAttr**

#### 


**Tipo:** Struct


**Uso:** Representa los atributos de configuración de un mutex , análogo a pthread_mutexattr en


**Propósito:** Centralizar el modo de operación del mutex, aunque en el estado actual solo conserva el entero “kind”


**Parámetros:** kind: i32 .


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Estructura inmutable tras su construcción que actúa como contenedor del valor “kind”.






### **MyMutex**

#### 


**Tipo:** Struct


**Uso:** Implementa un mutex en espacio de usuario con semántica cooperativa de bloqueo. al estar ocupado,


**Propósito:** Proveer exclusión mutua entre hilos gestionados por el runtime/scheduler del sistema, separando


**Parámetros:** 


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Mantiene el estado de inicialización, propiedad actual, un indicador atómico de bloqueo y una cola FIFO de espera. La transferencia al siguiente hilo no se realiza automáticamente al liberar; se delega en el planificador/scheduler que gestiona los reintentos de adquisición.




### **MyMutex::is_locked**

#### 


**Tipo:** Función/Método


**Uso:** Consultar si el mutex se encuentra actualmente bloqueado.


**Propósito:** Proveer una lectura coherente del estado de bloqueo para decisiones del runtime.


**Parámetros:** .


**Retorno:** bool .


**Descripción del funcionamiento:**  Lee el indicador atómico con semántica Acquire para observar de forma consistente el estado actualizado por el propietario al liberar.




### **MyMutex::new**

#### 


**Tipo:** Función/Método


**Uso:** Construir una instancia de mutex en estado no inicializado.


**Propósito:** Proveer un estado base que requiere invocar a la inicialización antes de emplear funciones de


**Parámetros:** .


**Retorno:** MyMutex .


**Descripción del funcionamiento:**  Establece valores por defecto sin habilitar todavía el uso del mutex.




### **MyMutex::init_mut**

#### 


**Tipo:** Función/Método


**Uso:** Inicializar el mutex dejándolo listo para su uso.


**Propósito:** Preparar el estado interno  y marcar el objeto como inicializado.


**Parámetros:** .


**Retorno:** c_int .


**Descripción del funcionamiento:**  Restablece el bloqueo a falso, elimina cualquier propietario, limpia la cola de espera y activa la marca de inicialización.




### **MyMutex::destroy**

#### 


**Tipo:** Función/Método


**Uso:** Destruir el mutex si su estado es válido para eliminación.


**Propósito:** Liberar el recurso lógico, invalidando su uso posterior.


**Parámetros:** .


**Retorno:** c_int .


**Descripción del funcionamiento:**  Verifica que el objeto esté inicializado, sin bloqueo activo y sin hilos en la cola de espera; en caso de éxito desactiva la inicialización y borra el propietario.




### **MyMutex::lock**

#### 


**Tipo:** Función/Método


**Uso:** Intentar adquirir el mutex; si está ocupado, encola el hilo  y devuelve un código sin


**Propósito:** Coordinar la entrada a sección crítica bajo una política cooperativa con asistencia del planificador.


**Parámetros:** tid: ThreadId .


**Retorno:** c_int (MutexNotInitialized si no fue inicializado; MutexLocked si ya está tomado; MutexLockApproved si


**Descripción del funcionamiento:**  Comprueba la inicialización; si el bloqueo está activo, incorpora el hilo a la cola de espera evitando duplicados y reporta que el mutex está ocupado; si está libre, activa el bloqueo con orden de memoria adecuada, establece el propietario y confirma la adquisición.




### **MyMutex::unlock**

#### 


**Tipo:** Función/Método


**Uso:** Liberar el mutex únicamente si el llamador es el propietario actual.


**Propósito:** Finalizar la sección crítica y permitir que otros hilos progresen según la política del planificador.


**Parámetros:** tid: Option .


**Retorno:** c_int (UnknownThread si no se proporciona identidad; MutexInvalidOwner si no coincide con el


**Descripción del funcionamiento:**  Verifica la presencia y coincidencia de la identidad con el propietario; si procede, desactiva el bloqueo y borra el propietario; no despierta ni asigna automáticamente al siguiente hilo en la cola.




### **Mypthread**

#### mypthread.rs:

**Tipo:** Módulo

**Uso:** Contiene la fachada de la API pública que expone funciones estilo pthread para hilos y mutex.

**Propósito:** Proveer una interfaz de alto nivel que encapsule la lógica interna del runtime y las primitivas de sincronización.

**Parámetros:** No aplica.

**Retorno:** No aplica.

**Descripción del funcionamiento:** Actúa como punto de entrada para las operaciones de hilos y mutex, delegando en el runtime subyacente.


### **MyPThread**

#### mypthread.rs:

**Tipo:** Struct

**Uso:** Contenedor de alto nivel que expone la API tipo pthread y delega la ejecución al runtime subyacente.

**Propósito:** Encapsular un MyTRuntime y ofrecer funciones estilo C para creación, unión, planificación, finalización y sincronización de hilos mediante mutex.

**Parámetros:** runtime: MyTRuntime (instancia interna del runtime).

**Retorno:** No aplica.

**Descripción del funcionamiento:** Mantiene un runtime propio y actúa como fachada de las operaciones de hilos y mutex, traduciendo las llamadas externas a acciones sobre el runtime y sobre las primitivas de sincronización.


### **MyPThread::my_thread_create**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Crear un nuevo hilo gestionado por el runtime con parámetros y planificador opcional.

**Propósito:** Registrar y preparar la ejecución de una rutina de hilo con atributos, argumento y tipo de planificador.

**Parámetros:** thread: *mut ThreadId; attr: *mut MyThreadAttr; start_routine: MyTRoutine; arg: *mut AnyParam; scheduler: Option.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Delegado directo a runtime.create, pasando el identificador de salida, atributos, rutina de inicio, argumento y configuración de planificador; devuelve el código proporcionado por el runtime.


### **scheduler: Option**

#### mypthread.rs:

**Tipo:** Parámetro

**Uso:** Indica la política de planificación opcional para el hilo creado.

**Propósito:** Permitir seleccionar el tipo de scheduler (RoundRobin, Lottery, RealTime) al crear el hilo.

**Parámetros:** Valor opcional de SchedulerType.

**Retorno:** No aplica.

**Descripción del funcionamiento:** Si se especifica, el runtime asigna el hilo al planificador indicado; si no, usa el predeterminado.


### **MyPThread::my_thread_join**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Esperar a que un hilo finalice y recuperar su valor de retorno.

**Propósito:** Sincronizar con la terminación de un hilo concreto y obtener su resultado.

**Parámetros:** thread: ThreadId; ret_val: *mut *mut AnyParam.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Invoca runtime.join con el identificador de hilo y el puntero donde se almacenará el valor de retorno; el resultado numérico indica el estado de la operación.


### **MyPThread::my_thread_yield**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Ceder voluntariamente la CPU al planificador.

**Propósito:** Permitir que el planificador seleccione otro hilo para ejecución, facilitando alternancia cooperativa.

**Parámetros:** (ninguno).

**Retorno:** c_int (Ok).

**Descripción del funcionamiento:** Solicita al runtime guardar el contexto actual y programar el siguiente hilo; tras la operación, devuelve código de éxito.


### **MyPThread::my_thread_end**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Finalizar el hilo actual proporcionando un valor de retorno.

**Propósito:** Indicar al runtime la terminación del hilo en curso con un puntero de resultado.

**Parámetros:** retval: *mut AnyParam.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Delega en runtime.end_current para cerrar el hilo activo y registrar el puntero de retorno asociado.


### **MyPThread::my_thread_detach**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Marcar un hilo para ejecución desacoplada, sin necesidad de join posterior.

**Propósito:** Permitir que un hilo libere sus recursos al finalizar sin requerir sincronización de unión.

**Parámetros:** thread: ThreadId.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Invoca runtime.detach para configurar el hilo identificado como separado del control de join.


### **MyPThread::my_mutex_init**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Inicializar una estructura MyMutex con atributos opcionales.

**Propósito:** Preparar un mutex para su uso en operaciones de bloqueo y desbloqueo.

**Parámetros:** mutex: *mut MyMutex; attr: *const MyMutexAttr.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Verifica puntero nulo de mutex; ignora los atributos en esta versión; delega en el método init_mut del propio mutex y retorna el código resultante.


### **MyPThread::my_mutex_destroy**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Destruir un mutex previamente inicializado.

**Propósito:** Invalidar el mutex y liberar su estado lógico de uso.

**Parámetros:** mutex: *mut MyMutex.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Llama directamente al método destroy del mutex y devuelve su resultado.


### **MyPThread::my_mutex_lock**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Solicitar la adquisición de un mutex por el hilo actual.

**Propósito:** Iniciar la operación de bloqueo para entrar en sección crítica.

**Parámetros:** mutex: *mut MyMutex.

**Retorno:** c_int (ThreadBlocked si se gestionará la espera; NullMutex si el puntero es nulo; CurrentIsEmpty si no hay hilo actual; otros códigos internos según avance).

**Descripción del funcionamiento:** Verifica puntero nulo; si existe hilo actual, invoca lock sobre el mutex con el ThreadId actual; en cualquier caso, devuelve ThreadBlocked para indicar al planificador que gestione el avance o la espera conforme a la política del runtime.


### **MyPThread::my_mutex_unlock**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Liberar un mutex previamente adquirido por el hilo actual.

**Propósito:** Salir de la sección crítica y permitir el progreso de otros hilos.

**Parámetros:** mutex: *mut MyMutex.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Verifica puntero nulo; solicita al mutex la liberación usando la identidad del hilo actual obtenida del runtime; devuelve el código que emite el mutex.


### **MyPThread::my_thread_chsched**

#### mypthread.rs:

**Tipo:** Función/Método

**Uso:** Cambiar la política de planificación asociada a un hilo.

**Propósito:** Actualizar el tipo de scheduler aplicado a un hilo específico.

**Parámetros:** thread: ThreadId; new_kind: SchedulerType.

**Retorno:** c_int (código de resultado).

**Descripción del funcionamiento:** Delega en runtime.change_scheduler para registrar el nuevo tipo de planificador del hilo indicado.


### **Mypthreadexit**

#### mypthreadexits.rs:

**Tipo:** Enum

**Uso:** Tabla de códigos simbólicos de resultado y estados para operaciones de hilos y mutex.

**Propósito:** Estandarizar los valores de retorno entre componentes del runtime, los wrappers estilo pthread y las primitivas de sincronización.

**Parámetros:** Ok = 0; ThreadBlocked = 1; MutexNotInitialized = 2; MutexInvalidState = 3; NullMutex = 4; MutexLockApproved = 5; MutexLocked = 6; CurrentIsEmpty = 7; ThreadIsTerminated = 8; UnknownThread = 9; MutexInvalidOwner = 10.

**Retorno:** No aplica.

**Descripción del funcionamiento:** Define identificadores numéricos constantes para representar éxito, estados de bloqueo y condiciones de error relacionadas con inicialización, propiedad, validez de punteros, estado del hilo actual y terminación; se utilizan como c_int en las funciones expuestas por MyPThread.


### **MyMutex::try_lock**

#### mymutex.rs:

**Tipo:** Función/Método

**Uso:** Intento de adquisición no bloqueante y sin encolamiento; adquiere solo si el mutex está libre.

**Propósito:** Ofrecer una verificación rápida para escenarios en los que no se desea esperar.

**Parámetros:** tid: ThreadId (identificador del hilo que intenta la adquisición).

**Retorno:** c_int.

**Descripción del funcionamiento:** Comprueba la inicialización; si el mutex está ocupado informa el estado sin modificar la cola; si está libre, activa el bloqueo y establece el propietario reportando éxito.



**Tipo:** Función/Método ​


**Uso:** Cambiar la política de planificación asociada a un hilo.​


**Propósito:** Actualizar el tipo de scheduler aplicado a un hilo específico.​


**Parámetros:** thread: ThreadId; new_kind: SchedulerType.​


**Retorno:** c_int .​


**Descripción del funcionamiento:**  Comprueba la inicialización; si el mutex está ocupado informa el estado sin modificar la cola; si está libre, activa el bloqueo y establece el propietario reportando éxito. ​ finalización y sincronización de hilos mediante mutex.​ Mantiene un runtime propio y actúa como fachada de las operaciones de hilos y mutex, traduciendo las llamadas externas a acciones sobre el runtime y sobre las primitivas de sincronización. ​ Delegado directo a runtime.create, pasando el identificador de salida, atributos, rutina de inicio, argumento y configuración de planificador; devuelve el código proporcionado por el runtime. ​ Invoca runtime.join con el identificador de hilo y el puntero donde se almacenará el valor de retorno; el resultado numérico indica el estado de la operación. ​ Solicita al runtime guardar el contexto actual y programar el siguiente hilo; tras la operación, devuelve código de éxito. ​ Delega en runtime.end_current para cerrar el hilo activo y registrar el puntero de retorno asociado. ​ Invoca runtime.detach para configurar el hilo identificado como separado del control de join. ​ Verifica puntero nulo de mutex; ignora los atributos en esta versión; delega en el método init_mut del propio mutex y retorna el código resultante. ​ Llama directamente al método destroy del mutex y devuelve su resultado. ​ hay hilo actual; otros códigos internos según avance).​ Verifica puntero nulo; si existe hilo actual, invoca lock sobre el mutex con el ThreadId actual; en cualquier caso, devuelve ThreadBlocked para indicar al planificador que gestione el avance o la espera conforme a la política del runtime. ​ Verifica puntero nulo; solicita al mutex la liberación usando la identidad del hilo actual obtenida del runtime; devuelve el código que emite el mutex. ​ Delega en runtime.change_scheduler para registrar el nuevo tipo de planificador del hilo indicado.




### **Exits**

#### 


**Tipo:** Enum


**Uso:** Tabla de códigos simbólicos de resultado y estados para operaciones de hilos y mutex.


**Propósito:** Estandarizar los valores de retorno entre componentes del runtime, los wrappers estilo pthread y las


**Parámetros:** Ok = 0; ThreadBlocked = 1; MutexNotInitialized = 2; MutexInvalidState = 3; NullMutex = 4;


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Define identificadores numéricos constantes para representar éxito, estados de bloqueo y condiciones de error relacionadas con inicialización, propiedad, validez de punteros, estado del hilo actual y terminación; se utilizan como c_int en las funciones expuestas por MyPThread.




### **MyTRuntime**

#### 


**Tipo:** Struct


**Uso:** Representa el runtime de hilos, responsable del tiempo lógico, la administración de hilos, la selección del


**Propósito:** Centralizar la creación, el ciclo de vida y la planificación de hilos, incluyendo colas de ejecución,


**Parámetros:** time_ms: usize; run_queue: VecDeque; threads: HashMap<ThreadId, MyThread>; next_id:


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Mantiene el reloj interno, la tabla de hilos por id, el hilo actual, estructura de espera por dependencia , y una colección de planificadores disponibles; expone operaciones para crear hilos, cambiarles la política de planificación, seleccionar el próximo a ejecutar y avanzar su estado hasta la terminación.




### **MyTRuntime::change_scheduler**

#### 


**Tipo:** Función/Método


**Uso:** Cambiar el planificador asociado a un hilo específico.


**Propósito:** Actualizar la política de planificación de un hilo en ejecución o listo para ejecutar, asegurando la


**Parámetros:** tid: ThreadId; new_kind: SchedulerType.


**Retorno:** c_int.


**Descripción del funcionamiento:**  Verifica que el hilo exista y no esté terminado; si el nuevo tipo coincide con el actual, no realiza cambios y retorna éxito; si difiere, actualiza el campo scheduler del hilo, reconstruye las colas de los planificadores para reflejar el cambio y retorna éxito.




### **MyTRuntime::rebuild_ready_queues**

#### 


**Tipo:** Función/Método​


**Uso:** Reconstruir las colas de listos en todos los planificadores.​


**Propósito:** Restablecer el estado interno de los planificadores y reencolar los hilos en estado Ready conforme a


**Parámetros:** .​


**Retorno:** .​


**Descripción del funcionamiento:**  Reinstancia los planificadores conocidos y recorre la tabla de hilos; por cada hilo en estado Ready, lo reencola en el planificador que corresponda a su tipo configurado.




### **MyTRuntime::advance_steps**

#### 


**Tipo:** Función/Método


**Uso:** Avanzar el reloj interno del runtime en milisegundos lógicos.


**Propósito:** Registrar el progreso del tiempo para decisiones de planificación o métricas.


**Parámetros:** passed: usize.


**Retorno:** .


**Descripción del funcionamiento:**  Incrementa el contador interno de tiempo usando suma saturada para evitar desbordamientos.




### **MyTRuntime::create**

#### 


**Tipo:** Función/Método


**Uso:** Crear un nuevo hilo, establecer su estado inicial y encolarlo en el planificador correspondiente.


**Propósito:** Registrar un hilo con su rutina, atributos y argumento, quedando listo para selección futura por el


**Parámetros:** thread_out: *mut ThreadId; attr: *mut MyThreadAttr; start_routine: MyTRoutine; args: *mut


**Retorno:** c_int.


**Descripción del funcionamiento:**  Asigna un identificador incremental, crea el hilo en estado Ready, lo inserta en la tabla de hilos y lo encola en el planificador seleccionado ; si se proporciona thread_out, escribe el identificador creado; retorna código de éxito.




### **MyTRuntime::pick_any_next**

#### 


**Tipo:** Función/Método


**Uso:** Seleccionar el próximo hilo a ejecutar según la prioridad entre planificadores.


**Propósito:** Implementar la política de selección global considerando el orden de preferencia entre RealTime,


**Parámetros:** .


**Retorno:** Option.


**Descripción del funcionamiento:**  Itera sobre los tipos de planificador en orden de prioridad; para el primero que no esté vacío, solicita un identificador de hilo mediante pick_next y lo devuelve; si ninguno tiene candidatos, retorna None.




### **MyTRuntime::run_thread**

#### 


**Tipo:** Función/Método


**Uso:** Ejecutar la rutina asociada a un hilo hasta su retorno y actualizar su estado a terminado.


**Propósito:** Avanzar la ejecución efectiva de un hilo elegido, registrar su valor de retorno y aplicar efectos de


**Parámetros:** tid: ThreadId.


**Retorno:** .


**Descripción del funcionamiento:**  Marca el hilo como Running, extrae su rutina y argumentos, ejecuta la rutina y captura el valor de retorno; al finalizar, actualiza el hilo como Terminated y registra la salida; notifica al planificador correspondiente su salida y despierta a los hilos que estaban en espera  sobre ese identificador; maneja el caso de hilos marcados como detached.




### **MyTRuntime::schedule_next**

#### 


**Tipo:** Función/Método


**Uso:** Programar la ejecución del siguiente hilo disponible según los planificadores.


**Propósito:** Conducir el ciclo de selección y ejecución de hilos, incluyendo el manejo de hilos ya terminados.


**Parámetros:** .


**Retorno:** c_int.


**Descripción del funcionamiento:**  Solicita un candidato con pick_any_next; si existe, lo establece como actual y, si ya estuviera terminado, despierta a sus joiners y retorna éxito; en caso contrario, ejecuta el hilo con run_thread y retorna éxito; si no hay candidatos, limpia el hilo actual y devuelve un código que indica ausencia de trabajo.




### **MyTRuntime::save_context**

#### 


**Tipo:** Función/Método


**Uso:** Guardar el contexto lógico del hilo actual marcándolo como listo.


**Propósito:** Preparar al hilo en ejecución para ser reprogramado posteriormente.


**Parámetros:** .


**Retorno:** .


**Descripción del funcionamiento:**  Si existe un hilo actual, actualiza su estado a Ready para que pueda volver a ser elegido por un planificador.




### **MyTRuntime::enqueue**

#### 


**Tipo:** Función/Método


**Uso:** Encolar un identificador de hilo en la cola de ejecución general.


**Propósito:** Registrar un hilo para posterior procesamiento en la cola interna run_queue.


**Parámetros:** tid: ThreadId.


**Retorno:** .


**Descripción del funcionamiento:**  Inserta el identificador al final de la cola run_queue.




### **MyTRuntime::detach**

#### 


**Tipo:** Función/Método


**Uso:** Marcar un hilo como detach y, si ya terminó, liberar su registro del runtime.


**Propósito:** Habilitar la finalización sin necesidad de join y la liberación de recursos asociados al hilo.


**Parámetros:** tid: ThreadId.


**Retorno:** c_int.


**Descripción del funcionamiento:**  Si el hilo existe, marca su atributo como detached; si su estado es Terminated, lo elimina de la tabla de hilos; devuelve un código de resultado indicando éxito o ausencia del hilo.




### **MyTRuntime::end_current**

#### 


**Tipo:** Función/Método


**Uso:** Señalar la finalización del hilo actual y avanzar la planificación.


**Propósito:** Registrar el valor de retorno del hilo actual, marcarlo como terminado y continuar con la selección del


**Parámetros:** retval: *mut AnyParam.


**Retorno:** c_int.


**Descripción del funcionamiento:**  Verifica la existencia de un hilo activo; si existe, almacena el valor de retorno y marca su estado como Terminated; despierta a los hilos que estaban esperando , limpia el hilo actual, invoca schedule_next para continuar la ejecución y retorna un código de resultado.




### **MyTRuntime::mark_blocked_on**

#### 


**Tipo:** Función/Método


**Uso:** Registrar que un hilo queda bloqueado esperando a otro hilo específico.


**Propósito:** Administrar dependencias de espera  entre hilos.


**Parámetros:** waiter: ThreadId; target: ThreadId.


**Retorno:** .


**Descripción del funcionamiento:**  Cambia el estado del hilo que espera a Blocked y lo asocia en la estructura wait_on bajo el identificador del hilo objetivo, para su posterior reactivación cuando el objetivo termine.




### **MyTRuntime::on_terminated**

#### 


**Tipo:** Función/Método


**Uso:** Despertar a los hilos que estaban esperando la finalización de un hilo objetivo.


**Propósito:** Reintegrar a ejecución a los hilos dependientes una vez que el hilo objetivo termina.


**Parámetros:** target: ThreadId.


**Retorno:** .


**Descripción del funcionamiento:**  Busca en wait_on los hilos que aguardaban al identificador especificado; los remueve de la estructura, los marca en estado Ready y los encola para su futura selección por los planificadores.





### **join**

#### 


**Tipo:** Función/Método​


**Uso:** Esperar a la terminación de un hilo objetivo y, al concluir, recuperar su valor de retorno.​


**Propósito:** Sincronizar la ejecución del hilo llamador con la finalización del hilo objetivo, entregando su resultado


**Parámetros:** target: ThreadId ; ret_val_out: *mut *mut AnyParam (puntero de


**Retorno:** c_int (0 en éxito; -1 en condiciones inválidas, por ejemplo objetivo inexistente, join sobre detached, join


**Descripción del funcionamiento:**   Verifica la existencia del hilo objetivo; si no existe, retorna -1. Comprueba si el hilo objetivo está marcado como detached; si lo está, retorna -1. Si el objetivo ya está en estado Terminated, escribe su valor de retorno en ret_val_out (si el puntero no es nulo) y devuelve 0. Si no hay hilo actual , itera ejecutando el planificador hasta que el objetivo termine; al concluir, escribe el valor de retorno  y devuelve 0; si el planificador no puede avanzar, retorna -1. En modo runtime , rechaza join a sí mismo; registra al hilo actual como “waiter” del objetivo asegurando no duplicar entradas y que solo exista un waiter para ese objetivo; marca el hilo actual como Blocked. Ejecuta un bucle donde el planificador corre otros hilos hasta que el objetivo termine; al volver a ser el hilo actual, reevalúa el estado del objetivo; si el planificador no puede avanzar, retorna -1. Cuando el objetivo está Terminated, escribe el valor de retorno en ret_val_out  y limpia la estructura de espera asociada al objetivo; devuelve 0.




### **wake_thread**

#### 


**Tipo:** Función/Método​


**Uso:** Cambiar el estado de un hilo específico a listo para ejecución, siempre que no haya terminado.​


**Propósito:** Reinsertar un hilo en la planificación marcándolo como Ready para que pueda ser seleccionado por


**Parámetros:** target: ThreadId .​


**Retorno:** c_int .​


**Descripción del funcionamiento:**   Localiza el hilo en la tabla interna; si no existe, retorna UnknownThread. Si el hilo no está Terminated, actualiza su estado a Ready y devuelve Ok. Si el hilo está Terminated, devuelve ThreadIsTerminated.






### **wake_joiners**

#### 


**Tipo:** Función/Método​


**Uso:** Despertar y reencolar a los hilos que estaban esperando por la terminación de un objetivo específico.​


**Propósito:** Rehabilitar para ejecución a los hilos bloqueados por join sobre un objetivo que ha concluido,


**Parámetros:** objective: &ThreadId .​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**   Extrae la lista de hilos que esperaban al objetivo desde la estructura de espera; si no hay entradas, no realiza acciones. Para cada esperador, verifica si su estado es Blocked; si lo es, lo cambia a Ready y lo encola en la run_queue para que pueda ser planificado. No escribe valores de retorno ni modifica otros estados del objetivo; únicamente reencola a los esperadores para su reactivación.




### **MyThread::run**

#### 


**Tipo:** Función/Método


**Uso:** Ejecutar la rutina asociada al hilo y registrar su resultado.


**Propósito:** Avanzar el hilo desde su estado actual hasta la terminación, capturando el valor de retorno producido


**Parámetros:** .


**Retorno:** Ninguno.


**Descripción del funcionamiento:**  Si el hilo ya está en Terminated, no realiza acción; en caso contrario, invoca la rutina con el argumento configurado, almacena el resultado en ret_val y establece el estado del hilo en Terminated.




### **MyThread**

#### 


**Tipo:** Struct


**Uso:** Representa un hilo administrado por el runtime, incluyendo su identidad, estado, atributos, rutina de inicio,


**Propósito:** Mantener el estado y la configuración necesarios para crear, ejecutar y finalizar un hilo dentro del


**Parámetros:** id: ThreadId; state: ThreadState; attr: *mut MyThreadAttr; start_routine: MyTRoutine; arg: *mut


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Conserva los metadatos y el estado de ejecución del hilo; el scheduler y el runtime consultan y actualizan estos campos durante el ciclo de vida del hilo.




### **MyThreadAttr**

#### 


**Tipo:** Struct


**Uso:** Representa los atributos de configuración de un hilo administrado por el runtime.


**Propósito:** Centralizar parámetros de ejecución de un hilo, incluyendo fecha límite , nivel de prioridad


**Parámetros:** inner: pthread_attr_t; dead_line: usize; priority: PriorityLevel; detached: bool.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Estructura que almacena los parámetros de planificación y control del hilo; el campo inner mantiene un descriptor de atributos POSIX inicializado, mientras que dead_line, priority y detached determinan el comportamiento de planificación y la relación con operaciones como join.



### **SchedulerType**

#### scheduler_type.rs:

**Tipo:** Enum

**Uso:** Identificar la política de planificación que se aplicará a los hilos en el runtime.

**Propósito:** Permitir la selección entre diferentes algoritmos de scheduling según las necesidades del sistema (justicia, aleatoriedad ponderada o plazos).

**Parámetros:** RoundRobin; Lottery; RealTime.

**Retorno:** No aplica.

**Descripción del funcionamiento:** Define los modos de planificación disponibles. RoundRobin selecciona hilos en orden FIFO cíclico. Lottery selecciona aleatoriamente ponderado por “tickets” asociados a cada hilo. RealTime prioriza hilos de acuerdo con su fecha límite declarada.


### **SchedulerParams**

#### scheduler_params:

**Tipo:** Enum

**Uso:** Especificar parámetros opcionales para la configuración de planificadores.

**Propósito:** Transportar valores de ajuste de cada política (por ejemplo, número de tickets o deadline).

**Parámetros:** None; Lottery (tickets: u32); RealTime (deadline_ms: u64); Priority (valor: i32).

**Retorno:** No aplica.

**Descripción del funcionamiento:** Estructura de parámetros que acompaña a la selección de SchedulerType para ajustar el comportamiento del planificador; cada variante modela el dato relevante para su estrategia.


### **Scheduler**

#### scheduler.rs:

**Tipo:** Trait

**Uso:** Definir el contrato común para implementaciones de planificadores de hilos.

**Propósito:** Unificar la interfaz de encolado, selección del próximo hilo y manejo de eventos del ciclo de vida.

**Parámetros:** Métodos: enqueue(tid: ThreadId, t: &MyThread); pick_next() -> Option; on_block(_tid: ThreadId); on_exit(_tid: ThreadId); is_empty() -> bool.

**Retorno:** Según método; pick_next retorna un identificador de hilo o None.

**Descripción del funcionamiento:** Establece las operaciones mínimas que debe implementar un scheduler. enqueue inserta hilos listos para ejecución. pick_next decide el siguiente hilo a ejecutar. on_block y on_exit son ganchos de evento para actualizar estructuras internas si se desea. is_empty informa si no hay hilos listos.


### **ThreadState**

#### thread_state.rs:

**Tipo:** Enum

**Uso:** Representar el estado de vida y ejecución de un hilo dentro del runtime.

**Propósito:** Proveer una codificación clara y explícita del ciclo de vida del hilo para decisiones del planificador y operaciones de sincronización.

**Parámetros:** New; Ready; Running; Blocked; Terminated.

**Retorno:** No aplica.

**Descripción del funcionamiento:** Enumera los estados posibles de un hilo administrado por el sistema. New indica que el hilo fue creado pero aún no ha sido planificado. Ready indica que está listo para ser ejecutado y puede ser seleccionado por el scheduler. Running indica que se encuentra actualmente en ejecución. Blocked indica que está suspendido a la espera de algún evento o recurso (por ejemplo, join o mutex). Terminated indica que ha finalizado su ejecución y puede liberar recursos o despertar a hilos que esperaban su finalización.



**Tipo:** Trait


**Uso:** Definir el contrato común para implementaciones de planificadores de hilos.


**Propósito:** Unificar la interfaz de encolado, selección del próximo hilo y manejo de eventos del ciclo de vida.


**Parámetros:** Métodos: enqueue; pick_next -> Option; on_block;


**Retorno:** Según método; pick_next retorna un identificador de hilo o None.


**Descripción del funcionamiento:**  Enumera los estados posibles de un hilo administrado por el sistema. New indica que el hilo fue creado pero aún no ha sido planificado. Ready indica que está listo para ser ejecutado y puede ser seleccionado por el scheduler. Running indica que se encuentra actualmente en ejecución. Blocked indica que está suspendido a la espera de algún evento o recurso . Terminated indica que ha finalizado su ejecución y puede liberar recursos o despertar a hilos que esperaban su finalización. . Define los modos de planificación disponibles. RoundRobin selecciona hilos en orden FIFO cíclico. Lottery selecciona aleatoriamente ponderado por “tickets” asociados a cada hilo. RealTime prioriza hilos de acuerdo con su fecha límite declarada. Estructura de parámetros que acompaña a la selección de SchedulerType para ajustar el comportamiento del planificador; cada variante modela el dato relevante para su estrategia. on_exit; is_empty -> bool. Establece las operaciones mínimas que debe implementar un scheduler. enqueue inserta hilos listos para ejecución. pick_next decide el siguiente hilo a ejecutar. on_block y on_exit son ganchos de evento para actualizar estructuras internas si se desea. is_empty informa si no hay hilos listos.




### **LotteryScheduler**

#### 


**Tipo:** Struct​


**Uso:** Planificador por lotería con selección aleatoria ponderada por tickets.​


**Propósito:** Distribuir tiempo de CPU de manera probabilística donde los hilos con más tickets tienen mayor


**Parámetros:** entries: Vec<> ; rng_state: u64 (estado del


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene una lista de hilos listos junto con la cantidad de tickets asignados. Utiliza un generador congruente aditivo tipo splitmix64 para obtener valores pseudoaleatorios y seleccionar el hilo proporcionalmente a la suma de tickets.




### **LotteryScheduler::with_seed**

#### 


**Tipo:** Función/Método ​


**Uso:** Crear una instancia del planificador de lotería con una semilla específica.​


**Propósito:** Permitir reproducibilidad en escenarios de prueba controlando la secuencia aleatoria.​


**Parámetros:** seed: u64.​


**Retorno:** LotteryScheduler.​


**Descripción del funcionamiento:**  Igual a la construcción estándar, pero inicializa rng_state con el valor proporcionado.




### **LotteryScheduler::enqueue**

#### 


**Tipo:** Función/Método​


**Uso:** Encolar un hilo listo con su cantidad de tickets calculada.​


**Propósito:** Registrar al hilo para posible selección en el próximo ciclo de planificación.​


**Parámetros:** tid: ThreadId; t: &MyThread.​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Obtiene la prioridad del hilo desde sus atributos y la interpreta como número de tickets . Agrega el par  a la lista interna.




### **LotteryScheduler::pick_next**

#### 


**Tipo:** Función/Método​


**Uso:** Ganchos de evento para reaccionar a bloqueo o salida de hilos.​


**Propósito:** Disponibles para actualizar estructuras internas si se requiere política adicional.​


**Parámetros:** _tid: ThreadId.​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Calcula la suma total de tickets, genera un número pseudoaleatorio acotado y recorre entries acumulando tickets hasta encontrar el intervalo que contiene el valor aleatorio. Extrae el elemento seleccionado y devuelve su identificador. LotteryScheduler::on_block / on_exit ​ Implementaciones vacías en esta versión; no alteran el estado del planificador.




### **RealTimeScheduler**

#### 


**Tipo:** Struct​


**Uso:** Planificador de tiempo real basado en prioridad por fecha límite.​


**Propósito:** Seleccionar siempre el hilo con menor deadline declarado para cumplir políticas basadas en tiempo.​


**Parámetros:** heap: BinaryHeap<Reverse<>> .​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene un heap mínimo  de pares ; el hilo con deadline más cercano se elige primero.




### **RealTimeScheduler::new**

#### 


**Tipo:** Función/Método​


**Uso:** Construir una instancia del planificador de tiempo real.​


**Propósito:** Inicializar la estructura de datos de prioridad.​


**Parámetros:** .​


**Retorno:** RealTimeScheduler.​


**Descripción del funcionamiento:**  Crea un heap vacío listo para recibir hilos.




### **RealTimeScheduler::enqueue**

#### 


**Tipo:** Función/Método​


**Uso:** Encolar un hilo listo con su fecha límite.​


**Propósito:** Registrar al hilo para que pueda ser priorizado por su deadline.​


**Parámetros:** tid: ThreadId; t: &MyThread.​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Extrae dead_line desde los atributos del hilo y lo inserta junto con tid en el heap como un elemento con prioridad por menor valor de deadline.




### **RealTimeScheduler::pick_next**

#### 


**Tipo:** Función/Método​


**Uso:** Seleccionar el siguiente hilo con deadline más cercano.​


**Propósito:** Garantizar que el hilo con menor fecha límite sea ejecutado primero.​


**Parámetros:** .​


**Retorno:** Option.​


**Descripción del funcionamiento:**  Extrae del heap el elemento de menor deadline y devuelve su identificador.




### **RealTimeScheduler::is_empty**

#### 


**Tipo:** Función/Método​


**Uso:** Indicar si no existen hilos listos.​


**Propósito:** Permitir al runtime conocer si hay trabajo pendiente.​


**Parámetros:** .​


**Retorno:** bool.​


**Descripción del funcionamiento:**  Informa si el heap no contiene elementos.




### **RRScheduler**

#### 


**Tipo:** Struct​


**Uso:** Planificador Round-Robin con cola FIFO.​


**Propósito:** Ofrecer una política justa en la que los hilos listos se ejecutan por turnos en orden de llegada.​


**Parámetros:** q: VecDeque .​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene una cola FIFO; cada hilo que se encola será seleccionado en el mismo orden, rotando de frente a fondo a medida que avanza la ejecución. El resto son iguales a las otras funciones




### **SimulationController**

#### 


**Tipo:** Struct


**Uso:** Coordinar la simulación de la ciudad integrando el manejo de tráfico, el registro de plantas nucleares y el acceso al mapa


**Propósito:** Proveer un punto central de orquestación que inicializa el mapa, identifica bloques relevantes  y


**Parámetros:** traffic: TrafficHandler ; nuclear_plants: Vec (coordenadas de bloques


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Mantiene referencias a los componentes principales de la simulación. El mapa es compartido de forma referenciada para permitir acceso coordinado entre subsistemas. La lista de coordenadas de plantas nucleares sirve para actualizar sus estados internos a medida que transcurre el tiempo simulado.




### **SimulationController::advance_time**

#### 


**Tipo:** Función/Método


**Uso:** Avanzar la simulación un número discreto de cuadros de tiempo .


**Propósito:** Progresar el estado de los bloques con dinámica temporal  y del sistema de tráfico de manera


**Parámetros:** frames: u8 .


**Retorno:** Ninguno.


**Descripción del funcionamiento:**  Obtiene acceso mutable al mapa y repite por cada frame: itera sobre la lista de coordenadas de plantas nucleares; para cada coordenada, obtiene el bloque en el mapa, realiza el intento de conversión al tipo NuclearPlantBlock y, si es posible, avanza su estado interno en una unidad de tiempo. Tras procesar las plantas nucleares en ese frame, invoca el avance de tiempo del TrafficHandler para actualizar el movimiento y estado del tráfico.




### **supply_kind**

#### 


**Tipo:** Enum


**Uso:** Identificar el tipo de insumo transportado por ciertos vehículos dentro de la simulación.


**Propósito:** Distinguir entre clases de suministros para habilitar reglas específicas de entrega en bloques destino.


**Parámetros:** Water; NuclearMaterial.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Enumera los tipos de carga reconocidos por el sistema, permitiendo que los bloques de destino  apliquen lógica de recepción acorde al insumo transportado.




### **Traffic_handler**

#### 


**Tipo:** Struct​


**Uso:** Orquestar el avance temporal de la simulación y la interacción entre vehículos y bloques del mapa.​


**Propósito:** Gestionar el ciclo de vida de los vehículos, su planificación de movimiento, la ocupación de vías, la coordinación de


**Parámetros:** vehicles: HashMap<ThreadId, Box>; road_coords: Vec; shops_coords: Vec; water_spawns: Vec; dock: Option;


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene el mapa y el conjunto de vehículos activos, junto con listas de coordenadas clave . Lleva métricas de éxito y fallos por tipo de vehículo y tick de simulación. Coordina la selección de movimientos y el acceso a recursos con capacidad limitada .




### **TrafficHandler::new_car**

#### 


**Tipo:** Función/Método​


**Uso:** Crear y registrar un vehículo tipo automóvil en una celda de carretera disponible y asignarle un destino de tienda.​


**Propósito:** Incorporar un nuevo coche a la simulación respetando la capacidad de la celda de origen y estableciendo su ruta


**Parámetros:** tid: ThreadId .​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Selecciona aleatoriamente una coordenada de carretera y verifica disponibilidad de espacio en el bloque correspondiente; en caso de ocupación, reintenta con otra coordenada hasta conseguir una celda libre y consume su espacio. Elige aleatoriamente un destino entre las coordenadas de tiendas. Construye el automóvil, lo inicializa con el mapa y el identificador proporcionado y lo inserta en la colección de vehículos gestionados.




### **TrafficHandler::advance_time**

#### 


**Tipo:** Función/Método​


**Uso:** Avanzar un ciclo de simulación aplicando la fase de planificación de movimientos y la resolución de accesos a puentes.​


**Propósito:** Ejecutar el progreso coordinado de todos los vehículos, determinando intenciones de movimiento y resolviendo la


**Parámetros:** .​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Recorre los vehículos para obtener su intención de movimiento según su tipo: los marítimos formulan intenciones acuáticas y el resto intenciones de carretera. Agrupa por coordenada de puente los vehículos que desean ingresar. Para cada puente con candidatos, solicita al bloque de puente que arbitre la entrada; si se concede acceso a un vehículo, se ordena su movimiento inmediato; si el puente está ocupado, no se realizan cambios sobre los vehículos rechazados en ese ciclo.




### **TrafficHandler::road_intention**

#### 


**Tipo:** Función/Método​


**Uso:** Determinar y procesar la intención de movimiento por carretera de un vehículo específico, aplicando las reglas de


**Propósito:** Evaluar la siguiente acción de un vehículo terrestre y aplicar los efectos sobre el mapa y las métricas de la


**Parámetros:** tid: &ThreadId .​


**Retorno:** Option<> (coordenada de un puente y el identificador del vehículo si el siguiente bloque es un


**Descripción del funcionamiento:**  Obtiene del vehículo su intención y coordenadas actuales. Si la intención es llegada, registra éxito y elimina el vehículo; adicionalmente, si el vehículo es un camión de carga destinado a una planta nuclear, confirma la entrega en el bloque correspondiente antes de retirarlo. Si la intención es avanzar a una coordenada de carretera, calcula la “paciencia” del vehículo y verifica si se encuentra saliendo desde un puente: de ser así, consulta al bloque de puente si puede egresar; si procede, consume capacidad del bloque destino y ordena el movimiento del vehículo, sin retornar intención de puente. Si no está en puente, intenta consumir capacidad de la celda destino y ordena el movimiento; si el resultado indica que alcanzó su límite de pasos permitidos en el tick y se movió, libera el espacio de la celda origen; si el vehículo se queda sin paciencia , incrementa el contador de fallos por tipo, registra el fallo y elimina el vehículo. Si la intención indica que el próximo bloque es un puente, devuelve la coordenada del puente junto con el identificador del vehículo para su tratamiento en la fase de arbitraje de puentes dentro de advance_time.




### **EntryOutcome**

#### 


**Tipo:** Enum


**Uso:** Representar el resultado de una solicitud de entrada al puente.


**Propósito:** Comunicar si el acceso fue concedido y, en ese caso, para qué vehículo, o si el recurso permanece ocupado.


**Parámetros:** GrantedFor { tid: ThreadId }; Occupied.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Cuando la admisión es concedida, la variante incluye el identificador del vehículo al que se reservó el recurso; en caso contrario, indica que no hay disponibilidad o que el semáforo/política no permiten el ingreso en este ciclo.




### **BridgeBlock**

#### 


**Tipo:** Struct


**Uso:** Modelar un bloque de puente con control de acceso y exclusión mediante un mutex.


**Propósito:** Coordinar la admisión y salida de vehículos del puente, respetando la política de control y la reserva del recurso a


**Parámetros:** base: BlockBase; control: Control; mutex: Option.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Mantiene la política de transporte, el control de semáforos y reglas de paso, y un mutex que actúa como reserva del carril de puente; expone operaciones para solicitar entrada, salir del puente y devolver el mutex.




### **BridgeBlock::request_entry**

#### 


**Tipo:** Función/Método


**Uso:** Solicitar la entrada al puente para un conjunto de vehículos candidatos.


**Propósito:** Arbitrar la admisión de un vehículo y, de concederse, reservar el recurso mediante el mutex.


**Parámetros:** vehicles: Vec<&Box>.


**Retorno:** EntryOutcome (GrantedFor con el identificador del vehículo elegido; Occupied si no se admite o no se puede


**Descripción del funcionamiento:**  Usa el control para seleccionar un candidato entre los vehículos. Si no hay candidato, devuelve Occupied. Si hay mutex, intenta reservarlo usando una operación de bloqueo no bloqueante; si el intento es exitoso, retorna GrantedFor con el identificador del vehículo; en caso contrario, retorna Occupied. Si no hay mutex, retorna Occupied.




### **BridgeBlock::exit_bridge**

#### 


**Tipo:** Función/Método


**Uso:** Intentar la salida del vehículo desde el puente, liberando la reserva si corresponde.


**Propósito:** Determinar si, dadas las reglas de salida, el vehículo puede egresar y, de ser así, liberar el recurso asociado al


**Parámetros:** v_type: VehicleType; v_pat: PatienceLevel.


**Retorno:** bool .


**Descripción del funcionamiento:**  Autoriza siempre la salida para barcos o cuando el control permite la salida según el tipo de vehículo y su paciencia. En caso de autorización, invoca la liberación del mutex asociado al puente y confirma con verdadero; en caso contrario, mantiene la reserva y devuelve falso.




### **BridgeBlock::return_mutex**

#### 


**Tipo:** Función/Método


**Uso:** Extraer el mutex interno del puente, transfiriendo su propiedad al llamador.


**Propósito:** Permitir la recuperación del recurso de exclusión cuando se requiere reconfiguración o reasignación externa.


**Parámetros:** .


**Retorno:** Option .


**Descripción del funcionamiento:**  Toma el mutex interno y devuelve su valor, dejando al bloque sin mutex asociado.




### **Control**

#### 


**Tipo:** Struct​


**Uso:** Gestionar las reglas de acceso de entrada y salida en un puente, incluyendo semaforización y preferencias por tipo de


**Propósito:** Determinar, en cada ciclo de simulación, qué vehículo puede entrar al puente y si un vehículo puede salir,


**Parámetros:** in_traffic_light: Option; out_traffic_light: Option; has_yield: bool; can_pass_boats: bool.​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene de forma opcional dos semáforos  para regular accesos. El modo has_yield activa reglas probabilísticas de salida basadas en el nivel de paciencia del vehículo; cuando no hay ceda el paso, la salida está gobernada por el semáforo y excepciones para ambulancias y barcos. El indicador can_pass_boats permite que los barcos tengan prioridad de paso según la política del puente.




### **Control::with_traffic**

#### 


**Tipo:** Función/Método​


**Uso:** Construir un Control con semáforos de entrada y salida activos.​


**Propósito:** Inicializar el control del puente para operar con intervalos definidos de actualización de semáforos.​


**Parámetros:** interval_in: usize ; interval_out: usize (intervalo de


**Retorno:** Control.​


**Descripción del funcionamiento:**  Crea semáforos para entrada y salida con los intervalos especificados; desactiva ceda el paso y no habilita paso preferente a barcos.




### **Control::without_traffic**

#### 


**Tipo:** Función/Método​


**Uso:** Construir un Control sin semáforos, con o sin ceda el paso.​


**Propósito:** Configurar el puente para operar sin semaforización, determinando si aplica una política de ceda el paso.​


**Parámetros:** has_yield: bool .​


**Retorno:** Control.​


**Descripción del funcionamiento:**  Desactiva semáforos de entrada y salida. Si has_yield es verdadero, la salida se rige por reglas probabilísticas; si es falso, se habilita can_pass_boats para dar paso preferente a embarcaciones.




### **Control::advance_time**

#### 


**Tipo:** Función/Método​


**Uso:** Avanzar el tiempo de los semáforos internos.​


**Propósito:** Actualizar el estado de los semáforos de entrada y salida según el tiempo transcurrido.​


**Parámetros:** frames: usize .​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Si existen semáforos, llama a su avance de tiempo con el valor recibido; actualiza internamente la luz roja o verde según el ciclo.




### **Control::allow_in**

#### 


**Tipo:** Función/Método​


**Uso:** Seleccionar, entre varios vehículos candidatos, cuál puede entrar al puente en este ciclo.​


**Propósito:** Establecer una política de arbitraje de entrada basada en tipo de vehículo y nivel de paciencia, priorizando casos


**Parámetros:** vehicles: Vec<&Box> .​


**Retorno:** Option .​


**Descripción del funcionamiento:**  Si no hay candidatos, devuelve None. Calcula, para cada vehículo, su tipo y nivel de paciencia. Concede prioridad inmediata a camiones con paciencia en estado crítico. Si no existen, prioriza ambulancias, eligiendo la de mayor urgencia según la escala de paciencia. Si no hay ambulancias, elige el vehículo con mayor “score” de urgencia , devolviendo su identificador.




### **Control::allow_out**

#### 


**Tipo:** Función/Método​


**Uso:** Determinar si un vehículo puede salir del puente en este ciclo.​


**Propósito:** Regular la salida considerando semaforización, ceda el paso y excepciones por tipo de vehículo.​


**Parámetros:** vehicle_type: VehicleType; patience_level: PatienceLevel.​


**Retorno:** bool .​


**Descripción del funcionamiento:**  Si no hay ceda el paso, siempre permite la salida de ambulancias y barcos; para el resto, consulta el semáforo de salida. Si hay ceda el paso, permite siempre a ambulancias; en otros vehículos, aplica una probabilidad de salida en función del nivel de paciencia .




### **TrafficLight**

#### 


**Tipo:** Struct​


**Uso:** Modelar un semáforo binario  con un intervalo fijo de actualización.​


**Propósito:** Representar el control temporizado de acceso, alternando entre permitir y denegar paso según un ciclo predefinido.​


**Parámetros:** in_red: bool; time_passed_ms: usize; update_interval_ms: usize.​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene un contador de tiempo transcurrido; al alcanzar el intervalo de actualización, alterna el estado en rojo/verde y reinicia el acumulado correspondiente.




### **TrafficLight::can_pass**

#### 


**Tipo:** Función/Método​


**Uso:** Consultar si está permitido el paso en el estado actual del semáforo.​


**Propósito:** Facilitar decisiones de control de acceso basadas en el color actual.​


**Parámetros:** .​


**Retorno:** bool .​


**Descripción del funcionamiento:**  Devuelve verdadero cuando in_red es falso; en caso contrario, deniega el paso.




### **TrafficLight::advance_time**

#### 


**Tipo:** Función/Método​


**Uso:** Avanzar el estado del semáforo según el tiempo transcurrido.​


**Propósito:** Actualizar el color del semáforo con base en su ciclo de actualización.​


**Parámetros:** time_passed: usize.​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Incrementa el contador; cuando alcanza o supera el intervalo, alterna el estado rojo/verde y descuenta del acumulado el intervalo aplicado.




### **Advance_time**

#### 


**Tipo:** Función/Método​


**Uso:** Avanzar el estado global de la simulación por una cantidad discreta de frames, aplicando progreso a plantas nucleares y


**Propósito:** Sincronizar el avance temporal de los subsistemas de la simulación, garantizando que cada frame afecte a los


**Parámetros:** frames: u8 .​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Obtiene acceso mutable al mapa. Por cada unidad de tiempo desde 0 hasta frames−1, recorre la lista de coordenadas de plantas nucleares y, para cada una, obtiene el bloque correspondiente y ejecuta su avance temporal con un paso de 1. Tras procesar todas las plantas para ese frame, invoca el avance del controlador de tráfico para aplicar las reglas de movilidad y control de infraestructura asociadas a ese ciclo.




### **SimulationController**

#### 


**Tipo:** Struct​


**Uso:** Coordinar y encapsular los componentes principales de la simulación, incluyendo tráfico, plantas nucleares y el mapa


**Propósito:** Proveer un punto de control único para el progreso temporal y la interacción entre el controlador de tráfico y los


**Parámetros:** traffic: TrafficHandler ; nuclear_plants: Vec (coordenadas de plantas nucleares


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Mantiene el estado y referencias necesarias para avanzar la simulación. Permite que el ciclo de simulación invoque de manera ordenada las actualizaciones de plantas nucleares y del tráfico sobre el mismo mapa, asegurando consistencia temporal en cada frame procesado.




### **MoveIntent**

#### 


**Tipo:** Enum​


**Uso:** Expresar el estado de paciencia/urgencia de un vehículo en el ciclo actual.​


**Propósito:** Permitir que las políticas del sistema (p. ej., arbitraje de puentes o decisiones de


**Parámetros:** Maxed { moved: bool }; Low; Critical; Starved.​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Arrived indica que el vehículo alcanzó su destino y no requiere más movimientos. NextIsBridge comunica que el siguiente bloque del camino es un puente en la coordenada indicada, activando el manejo específico de acceso. AdvanceTo indica que el siguiente movimiento propuesto es avanzar a la coordenada objetivo que no es un puente. PatienceLevel​ movimiento) prioricen o limiten vehículos en función de su urgencia.​ Maxed representa que la paciencia ha alcanzado su límite, con un indicador de si el vehículo se movió en el ciclo. Low indica paciencia baja. Critical indica urgencia crítica. Starved representa agotamiento sin progreso, generalmente usado para retirar el vehículo de la simulación o contabilizar fallo.




### **Occupancy**

#### 


**Tipo:** Alias de tipo​


**Uso:** Mapear coordenadas a identificadores de hilo/vehículo para representar ocupación.​


**Propósito:** Mantener un registro de qué vehículo ocupa una determinada coordenada del


**Parámetros:** HashMap<Coord, ThreadId>.​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Estructura asociativa que permite conocer y actualizar qué vehículo está presente en una celda específica del mapa.




### **VehicleBase**

#### 


**Tipo:** Struct​


**Uso:** Proveer el estado y comportamiento base compartido por todas las implementaciones


**Propósito:** Centralizar datos de posición, destino, ruta, identificación de hilo, tipo de vehículo


**Parámetros:** current_position: Coord; vehicle_type: VehicleType; destination: Coord; path:


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Conserva la posición actual y el destino del vehículo, junto con una ruta opcional y un índice del próximo paso a ejecutar. Almacena el identificador de hilo asociado al vehículo y gestiona la paciencia actual con un máximo configurable, soporte




### **VehicleBase::calculate_path**

#### 


**Tipo:** Función/Método​


**Uso:** Calcular una ruta desde la posición actual hasta el destino de acuerdo con las


**Propósito:** Determinar un camino válido para el vehículo que respete restricciones del


**Parámetros:** map: &Map.​


**Retorno:** Ninguno.​


**Descripción del funcionamiento:**  Ejecuta una búsqueda en anchura  desde la posición actual, registrando predecesores y evitando revisitar nodos. Al alcanzar el destino, reconstruye la ruta desde el final hacia el origen, la invierte para obtener el orden de avance y la almacena en path; inicializa path_idx en 1 si hay al menos un siguiente paso, o en 0 si el vehículo ya está en el destino.




### **VehicleBase::thread**

#### 


**Tipo:** Función/Método​


**Uso:** Obtener el identificador de hilo asociado al vehículo.​


**Propósito:** Exponer la identidad del vehículo dentro del runtime para operaciones de


**Parámetros:** .​


**Retorno:** ThreadId.​


**Descripción del funcionamiento:**  Devuelve el identificador de hilo previamente asignado al vehículo; requiere que el vehículo haya sido inicializado con un ThreadId válido.




### **VehicleBase::plan_next**

#### 


**Tipo:** Función/Método​


**Uso:** Proponer el siguiente movimiento sin alterar el estado global.​


**Propósito:** Indicar al controlador cómo pretende avanzar el vehículo en el próximo ciclo.​


**Parámetros:** map: &Map.​


**Retorno:** MoveIntent.​


**Descripción del funcionamiento:**  Obtiene la siguiente coordenada a partir de path_idx. Consulta el tipo de bloque en esa coordenada; si es un puente, devuelve NextIsBridge con la coordenada. En caso contrario, devuelve AdvanceTo con la coordenada propuesta. Si no hay ruta, el vehículo se considera no inicializado.




### **Vehicle**

#### 


**Tipo:** Trait​


**Uso:** Definir la interfaz que deben implementar los vehículos concretos en la simulación.​


**Propósito:** Establecer el contrato común de inicialización, planificación, intento de


**Parámetros:** get_type -> &VehicleType; as_any -> &mut dyn Any; initialize(map: &Map,


**Retorno:** Según cada método.​


**Descripción del funcionamiento:**  La interfaz obliga a exponer el tipo de vehículo, permitir conversión dinámica, inicializar el vehículo en el mapa con su identificador de hilo, proponer movimientos, intentar desplazamientos condicionados por capacidad del bloque destino, consultar y modificar el estado base y calcular el nivel de paciencia. El método current devuelve la posición actual a partir de la base.




### **VehicleType**

#### 


**Tipo:** Enum


**Uso:** Identificar la clase de vehículo dentro de la simulación para aplicar políticas de tránsito,


**Propósito:** Diferenciar comportamientos y privilegios de circulación según el tipo de vehículo,


**Parámetros:** CarE; AmbulanceE; TruckE; ShipE.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Define cuatro categorías operativas de vehículos. CarE representa vehículos particulares. AmbulanceE identifica vehículos de emergencia con prioridad en accesos y salidas. TruckE corresponde a camiones de carga, susceptibles de reglas especiales como entregas críticas. ShipE representa embarcaciones que operan en rutas acuáticas y tienen excepciones de paso en puentes según la configuración del control.




### **TransportPolicy**

#### 


**Tipo:** Enum​


**Uso:** Definir qué tipos de vehículos están autorizados a transitar por un bloque del mapa.​


**Propósito:** Permitir que cada bloque imponga restricciones de paso según la clase de


**Parámetros:** NoVehicles; Car; Truck; Ship; AnyVehicle.​


**Retorno:** No aplica.​


**Descripción del funcionamiento:**  Cada variante expresa una política de acceso. NoVehicles deniega a todos. Car autoriza autos, camiones y ambulancias . Truck autoriza solo camiones. Ship autoriza únicamente embarcaciones. AnyVehicle autoriza a todos los tipos.




### **TransportPolicy::can_pass**

#### 


**Tipo:** Función/Método​


**Uso:** Consultar, si corresponde, el estado de una planta nuclear ubicada en una


**Propósito:** Exponer a capas de presentación o monitoreo el estado lógico de una planta sin


**Parámetros:** coord: Coord.​


**Retorno:** Option (estado de la planta si hay una NuclearPlant en la coordenada; None en


**Descripción del funcionamiento:**  Aplica una correspondencia directa entre la política y el permite ShipE; AnyVehicle permite todos; NoVehicles deniega todos. ​ Busca el bloque en la coordenada y devuelve su política como copia si el bloque existe. find_blocks​ nucleares o todos los puentes).​ Recorre la grilla completa y agrega a la colección todas las try_plant_status_at​ coordenada.​ requerir conocimiento del tipo concreto del bloque.​ otros casos).​ Verifica límites; obtiene el bloque; confirma que el tipo sea NuclearPlant; realiza conversión dinámica al bloque concreto y devuelve su estado si el downcast es exitoso.




### **BlockBase**

#### 


**Tipo:** Struct


**Uso:** Representar los metadatos fundamentales de un bloque en el mapa de la simulación.


**Propósito:** Proporcionar una estructura común que encapsula el identificador único del


**Parámetros:** id: usize; policy: TransportPolicy; block_type: BlockType.


**Retorno:** No aplica.


**Descripción del funcionamiento:**  Esta estructura sirve como base para todos los bloques del mapa. El campo id identifica de forma única el bloque. policy define qué tipos de vehículos pueden transitar por él. block_type indica la categoría del bloque (por ejemplo, carretera, puente, tienda, etc.).



## 4.Instrucciones para ejecutar el programa:

Antes de compilar y ejecutar el programa hay que estar seguro de tener la biblioteca libgtk
sudo apt install libgtk-4-dev
Como cargo es nuestro gestor de compilacion y ejecución
cargo clean
cargo build --release
cargo run

## 5. Actividades realizadas por estudiantes

| Estudiante           | Actividad                                                      | Horas | Fecha              |
|----------------------|---------------------------------------------------------------|-------|--------------------|
| Joshua Solís Fuentes | Preparación del repo e archivos iniciales                    | 0.5   | 20/10/25          |
| Ion                  | Formar la base del proyecto                                  | 1     | 21/10/25          |
| Joshua Solís Fuentes | Implementación muy inicial de mypthreads                     | 3.5   | 22/10/25          |
| Ion Dolanescu        | Implementación más avanzada de metodos mypthreads con firma en C | 5     | 23/10/25          |
| Joshua Solís         | Test de métodos más básicos e implementación de thread_end   | 3     | 24/10/25          |
| Ion Dolanescu        | MyAttr compatible con pthread                                | 4     | 24/10/25          |
| Joshua Solís         | Tratar de implementar scheduler                              | 2     | 26/10/25          |
| Ion Dolanescu        | Implementación de mutex                                      | 3     | 27/10/25          |
| Joshua S e Ion dolanescu | Últimos detalles de la implementación de mypthreads y Scheduler | 10    | 27/10/25 / 28/10/25 |
| Ion Dolanescu        | Esqueleto mythreadcity                                       | 3     | 29/10/25          |
| Joshua Solís         | Limpieza de ThreadCity y implementación de my_thread_chsched | 2     | 30/10             |
| Ion Dolanescu        | Implementación de bloques                                    | 4     | 30/10             |
| Joshua e Ion         | Implementación puentes                                       | 5     | 1/11/10           |
| Joshua               | Implementación temprana movimiento de carro                  | 4     | 2/11/25           |
| Ion                  | Implementación completa del movimiento de carros             | 4     | 3/11/25           |
| Joshua               | Implementación de las plantas nucleares                      | 3     | 5/11/25           |
| Ion y Joshua         | Creación del mapa e inicio del scheduler                     | 4     | 8/11              |
| Joshua               | GUI actualizable                                             | 6     | 10/11/25          |
| Ion                  | Detalles finales de la GUI                                   | 4     | 11/11/25          |
| Joshua               | Documentación                                                | 5     | 11/11/25          |
|Total| Horas totales de trabajo| 76||

## 6. Comentarios finales

El desarrollo del proyecto ThreadCity ha alcanzado un estado funcional suficiente,
cumpliendo con los objetivos principales planteados. Se logró implementar la biblioteca
mypthreads con las funciones esenciales para la gestión de hilos y sincronización mediante
mutex, así como los tres tipos de planificadores: Round-Robin, Sorteo (Lottery) y Tiempo
Real (EDF). Estos componentes se integraron de una forma medianamente exitosa en la
simulación de la ciudad, el principal problema encontrado fue como enlazar la lógica de los
hilos con el movimiento de la ciudad, los elementos están presentes pero no tan integrados
a mypthreads, principalmente por confusiones con el enunciado ya que se utiliza varios
componentes externos por ejemplo un controlador en vez del scheduler, esta es la parte del
proyecto menos exitosa, pero el comportamiento de la ciudad cumple en ser lo
suficientemente funcional.
Durante el proceso se presentaron algunos desafíos técnicos. En particular, se identificaron
situaciones donde los hilos bloqueados por mutex no eran despertados correctamente, lo
que podía generar bloqueos prolongados o condiciones de inanición. Además, la interfaz
gráfica, aunque funcional puede mejorar su claridad visual en especial con mapas más
grandes, además del salto inicial de no saber como funciona la biblioteca lo que afectó la
fluidez de la animación.
Entre las limitaciones adicionales se encuentra la ausencia de preempción real basada en
temporizadores, ya que el cambio de contexto se realiza de forma cooperativa mediante
llamadas explícitas a yield. Tampoco se implementaron primitivas adicionales como
variables de condición o semáforos, ni se realizaron pruebas de estrés bajo alta contención.
A pesar de estas limitaciones, el proyecto proporciona una base sólida con hilos en espacio
de usuario, planificación personalizada y simulación concurrente. El diseño modular permite
futuras extensiones y mejoras tanto en la lógica de simulación como en la robustez del
runtime.

## 7. Conclusiones:

El desarrollo de ThreadCity ha representado una experiencia dura pero educativa en la
comprensión de los sistemas operativos, especialmente en la gestión de hilos en espacio de
usuario, sincronización y planificación. La implementación de la biblioteca mypthreads
permitió explorar de forma práctica conceptos  de interacción entre hilos con diferentes
políticas de planificación.

## 8.Referencias

- ​ “Rust documentation.” https://doc.rust-lang.org/stable/G. Team, “The GTK Project - A free and open-source cross-platform widget toolkit,”The GTK Team. https://www.gtk.org/docs/

- ​ RHIT CSSE Department, “Userspace Threads i,” CSSE 332.
https://www.rose-hulman.edu/class/csse/csse332/2324b/labs/lab09/

- ​ Tanenbaum, A. S., & Bos, H. (2015). Modern Operating Systems (4th ed.). Pearson, Los capítulos sobre scheduling de procesos y sincronización
- ​ Silberschatz, A., Galvin, P. B., & Gagne, G. (2018). Operating System Concepts
(10th ed.). Wiley.
- ​ splitmix. (s. f.). Hackage. https://hackage.haskell.org/package/splitmix
- GeeksforGeeks. (2025, 11 octubre). CPU Scheduling in Operating Systems. GeeksforGeeks. https://www.geeksforgeeks.org/operating-systems/cpu-scheduling-in-operating-systems/

## Anexo

### Semana #1

Fechas: [20/10/2025] - [26/10/2025]
- ​ Sprint Goal (Objetivo Principal): Implementar la base de mypthreads y hacer test
sobre ellos
    - ​ Sprint Backlog (Tareas Planeadas):
  - ​ Runtime
  - ​ my_thread_create
  - ​ my_thread_end
  - ​ my_thread_yield
  - ​ my_thread_join
  - ​ my_thread_detach
  - ​ my_mutex_init
  - ​ my_mutex_destroy
  - ​ my_mutex_lock
  - ​ my_mutex_unlock
  - ​ my_mutex_trylock
  - ​ my_thread_chsched:

Tareas Completadas:
  - ​ Runtime
  - ​ my_thread_create
  - ​ my_thread_end
  - ​ my_thread_yield
  - ​ my_thread_detach
  - ​ my_mutex_init
  - ​ my_mutex_lock
  - ​ my_mutex_unlock
  - ​ my_mutex_trylock

### Semana #2

Fechas: [27/10/2025] - [02/11/2025]
- ​ Sprint Goal (Objetivo Principal): Terminar la librería Mypthreads, hacer los
schedulers, hacer el esqueleto de la ciudad
    - ​ Sprint Backlog (Tareas Planeadas):
  - ​ my_thread_join
  - ​ Esqueleto de la ciudad
  - ​ my_thread_chsched:
  - ​ RoundRobin
  - ​ Lottery
  - ​ Tiempo Real
  - ​ Bloque de ciudad
  - ​ Implementación temprana de puentes

Tareas Completadas:
  - ​ my_thread_join
    - ​ Esqueleto de la ciudad
  - ​ RoundRobin
  - ​ Lottery
  - ​ Tiempo Real
    - ​ Bloque de ciudad
      - ​ Implementación temprana de puentes

### Semana #3

Fechas: [3/11/2025] - [09/11/2025]
- ​ Sprint Goal (Objetivo Principal): Implementar la logica a la ciudad, terminar los últimos detalle e implementar  GUI
    - ​ Sprint Backlog (Tareas Planeadas):
  - ​ my_thread_chsched:
    - ​ Implementación de los distintos tipos de vehículos
    - ​ Implementación del movimiento de los carros
    - ​ Implementación puente 1
    - ​ Implementación puente 2
    - ​ Implementación puente 3
    - ​ Implementación planta nuclear
    - ​ Implementación temprana de la GUI

Tareas Completadas:
  - ​ my_thread_chsched:
        - ​ Implementación de los distintos tipos de vehículos
      - ​ Implementación del movimiento de los carros
    - ​ Implementación puente 1
    - ​ Implementación puente 2
    - ​ Implementación puente 3
      - ​ Implementación planta nuclear

### Semana #4

Fechas: [10/11/2025] - [11/11/2025]

- Sprint Goal (Objetivo Principal): Implementar la GUI y hacer un controlador de la
simulación
    - ​ Sprint Backlog (Tareas Planeadas):
    - ​ Creación del mapa
    - ​ Implementación de la GUI
    - ​ Controlador de la simulación
    - ​ Tareas Completadas:
    - ​ Creación del mapa
    - ​ Implementación de la GUI
    - ​ Controlador de la simulación
