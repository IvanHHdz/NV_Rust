# _Fearless Concurrency_

Otro de los puntos fuertes de Rust es precisamente su capacidad de facilitar la programación paralela y concurrente. Ayudando a disminuir los errores que se pueden presentar en esta parte.

En un principio, el equipo de Rust creía que asegurar la seguridad de la memoria y prevenir los problemas de la concurrencia eran cosas separadas y que necesitaban resolverse por separado. Con el tiempo se dieron cuenta que esto no era así. Resulta que el [_Ownership_](../1.4%20-%20Ownership/Ownership.md) y sus reglas son de gran ayuda para manejar también los errores de concurrencia. De esta forma, muchos de los errores de concurrencia se vuelven errores en tiempo de compilación, lo que facilita su corrección. A esto se le ha dado el apodo de _fearless concurrency_ (concurrencia sin miedo).

> Nota: por ahora nos referiremos por "concurrencia" a "concurrencia y/o paralelismo".

> Nota: En esta parte vamos a asumir que usted ya sabe qué son los hilos, el multiprocesamiento, paralelismo, concurrencia, etc., junto a los problemas típicos derivados de los anteriores como condición de carrera, interbloqueo, innanición, etc. 

# Hilos



# Concurrencia

Cap. 16.3 y 16.4

# Programación asincrónica

Cap. 17