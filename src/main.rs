struct Notebook {
    question: String,
    answer: Option<String>,
}

impl Notebook {
    fn write_question(&mut self, question: String) {
        self.question = question;
        self.answer = None;

        println!("Mash escreve no caderno {}", self.question)
    }

    fn write_answer(&mut self, answer: String) {
        self.answer = Some(answer);
        println!("Lance responde à pergunta no caderno: {}", self.answer.as_ref().unwrap())
    }

    fn read_answer(&self) {
        match &self.answer {
            Some(answer) => println!("Mash lê a resposta: {}", answer),
            None => println!("Ainda não há resposta.")
        }
    }
}

fn lance_answer_question(notebook: &mut Notebook) {
    println!("Caderno mágico vai para Lance.");
    let question = &notebook.question;
    println!("Lance lê a pergunta {}", question);

    let answer = format!("A reposta mágica para '{}' é 42!", question);
    notebook.write_answer(answer);
    println!("Caderno mágico volta para o Owner Mash")
}

fn main() {
    // Mash possui o caderno
    let mut mash_notebook = Notebook {
        question: String::new(),
        answer: None,
    };

    mash_notebook.write_question(String::from("Qual é a resposta pra vida o universo e tudo mais?"));

    lance_answer_question(&mut mash_notebook);

    mash_notebook.read_answer();
}
