/*
Existem 3 formas de compilar/rodar o programa:
1- Transforme o arquivo .rs em .exe com rustc <nome_do_arquivo.rs> e depois .\<nome_do_arquivo.exe>
2- Crie o debug com a ferramenta cargo digitando cargo build - com isso crie todos os executáveis necessários no projeto.
Para executar use .\target\debug\<nome_do_arquivo.exe>
3- Com o cargo run rode sem compilar
*/

fn main() {
    println!("Hello! My name is Jônatas, but you can call me Jay!");
    print!("I'm 39 years old.");
    print!("I love music and technology.");
}
