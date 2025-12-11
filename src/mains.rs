use std::io::{self, Write};

#[derive(Clone)]
struct Item {
    id: u32,
    nome: String,
}

fn main() {
    let mut itens: Vec<Item> = Vec::new();
    loop {
        println!("--- Inventario CLI ---");
        println!("1 - Adicionar");
        println!("2 - Listar");
        println!("3 - Remover");
        println!("0 - Sair");
        print!("Escolha: ");
        io::stdout().flush().unwrap();

        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();
        let op = op.trim();
        match op {
            "1" => {
                let mut id_s = String::new();
                print!("ID: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_s).unwrap();
                let id: u32 = id_s.trim().parse().unwrap_or(0);
                let mut nome = String::new();
                print!("Nome: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut nome).unwrap();
                itens.push(Item { id, nome: nome.trim().to_string() });
            }
            "2" => {
                for it in &itens {
                    println!("{} - {}", it.id, it.nome);
                }
            }
            "3" => {
                let mut id_s = String::new();
                print!("ID a remover: "); io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_s).unwrap();
                let id: u32 = id_s.trim().parse().unwrap_or(0);
                itens.retain(|x| x.id != id);
            }
            "0" => break,
            _ => println!("Opcao invalida."),
        }
    }
    println!("Saindo...");
}
