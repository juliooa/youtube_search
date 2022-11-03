# Webapp con Rust, Webassembly, y Yew

Puedes ver el tutorial de como hice esta app aquí: https://www.youtube.com/watch?v=t6f0UOTwX4A

## Para ejecutar:

Agrega el target de webassembly:
```
$ rustup target add wasm32-unknown-unknown
```
Instala Trunk para correr la app:
```
$ cargo install trunk
```
Agrega tu api de Youtube en el archivo env.rs:
```
pub static API_KEY: &str = "TU_API_KEY";
```
Ejecuta la app:
```
$ trunk serve
```
Enjoy:
<img width="1406" alt="Screen Shot 2022-11-03 at 17 27 51" src="https://user-images.githubusercontent.com/1221345/199827827-17d0eb3b-24af-4926-b9ba-23a772997b51.png">
