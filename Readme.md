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
