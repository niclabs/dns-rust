# Demo TSIG
Esta es la demo para tsig, es un proxy que retransmite los mensajes a el server, corre con dos pc's en la ip 192.168.100.(2|3)/24, idealmente usen un router y configuren las ip's de manera estatica.
- cliente es el 100.2, con los puertos 8887 y 8890
- server es el 100.3

La llave que comparten esta en el archivo llave.key

## recv_dig
Aqui la demo funciona como un proxy, recive un mensaje del dig, lo procesa y despues recive el mensaje de bind9, lo procesa y lo retransmitee (util para testear si funciona el parseo y la verficacion de tsig)

comando para correr tsig con dig, ejecutar el main y en otra terminal ejecutar

```bash
dig -p8887 @127.0.0.1 ns1.nictest -k wena.key
```

Y luego correr el ejemplo

## recv_without_dig

Esto es la demo que se vio en la presentacion. Este genera un mensaje preguntando por ns1.nictest (dominio configurado en bind9) y luego usa las funciones sign y process para verificar el tsig, lo envia, y verifica la respuesta del servidor

Aqui es solo correr el ejemplo

```bash
cargo run
```
