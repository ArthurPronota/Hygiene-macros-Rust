/*
Это макрос без аргументов: matcher () означает, что вызов должен 
быть my_once!() — с пустыми скобками и без входных токенов. 
Transcriber подставляет два let-выражения.
my_once!()
// ↓ разворачивается в:
{ let x = 1; let _ = x; }
*/
macro_rules! my_once {
 () => { let x = 1; let _ = x; };
}


fn main() {
    let mut x = 10 ;

    my_once!() ;

    println!("x: {}", x) ;
} /* Out:
x: 10
*/
