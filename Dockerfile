#Un Dockerfile siempre necesita importar una imagen de alguna parte
#En este caso, importamos la imagen de rust que se encuentra en DockerHub
FROM rust

#Tenemos que utilizar este comando para copiar todos los archivos del directorio en el que estemos
#(Es decir, donde está todo el proyecto)
COPY . .

#Desde este punto, todo lo que hay que hacer es ejecutar los comandos que queramos ejecutar
#En este caso, debemos compilar y ejecutar el programa

#Despues, como tenemos los archivos copiados, podemos ejecutar el comando para que se compile todo.
RUN cargo build

#A diferencia de "RUN", "CMD" es un comando que se ejecuta, pero cuando el contenedor inicia, no cuando se crea
#La imagen.
CMD cargo run