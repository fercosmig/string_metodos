fn main()
{
    {
        let minha_string = String::from("Oi, meu nome é Fernando");
        println!("{}", minha_string);
        println!("{}", minha_string.replace(" ", "_"));
        println!("{}", minha_string.to_uppercase());
    }

    {
        let minha_string = String::from("Fui hoje\nao mercado\ncomprar arroz");
        println!("{}", minha_string);

        for linha in minha_string.lines()
        {
            println!("({})", linha);
        }
    }

    {
        let minha_string = String::from("O rato roeu a roupa do rei de Roma");
        let palavras: Vec<&str> = minha_string.split(" ").collect();

        println!("{:?}", palavras);
        println!("{}", palavras[1]);
    }

    {
        let minha_string = String::from("                   Meu nome é Fernando           ");

        println!(">>{}<<", minha_string);
        println!(">>{}<<", minha_string.trim());
    }

    {
        let minha_string = String::from("A casa está rodeada de cobras");
        println!("{}", minha_string);
        match minha_string.chars().nth(7)
        {            
            Some(c) => println!("Sucesso! O caracter da posição 7 é {}", c),
            None => println!("Caracter não encontrado!")
        }
    }
}
