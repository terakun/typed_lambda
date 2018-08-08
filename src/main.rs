pub mod ast;
pub mod parser;
pub mod typing;

use parser::Parser;
use typing::TypeInf;
use typing::calculate_mgu;
use typing::TypeEnv;

use std::io;

fn main() {
    let mut parser = Parser::new();
    loop {
        print!("> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let input = input.trim().to_string();
        if input.len() == 0 {
            println!("\nbye.");
            break;
        }
        let exp = parser.parse(&input).expect("error");
        let mut tf = TypeInf::new();
        let (typed_exp, _, c) = tf.type_inf(&TypeEnv::new(), &exp);
        // for con in &c {
        //     println!("{} = {}", con.0.to_string(), con.1.to_string());
        // }
        let uni = match calculate_mgu(&c) {
            Some(uni) => uni,
            None => {
                println!("untyped expression.");
                continue;
            }
        };
        let typed_exp = typed_exp.unify(&uni);
        let t = typed_exp.construct_type(&TypeEnv::new());
        println!("{} : {}", typed_exp.to_string(), t.to_string());

        let tenv = TypeEnv::new();
        println!("\nbussproofs format:");
        println!("\\begin{{prooftree}}");
        typed_exp.to_bussproofs(&tenv);
        println!("\\end{{prooftree}}");
    }
}
