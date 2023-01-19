use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("INPUT: {:#?}", input);
    TokenStream::from(quote!{
        use std::error::Error;
        pub struct CommandBuilder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }
        impl Command {
            pub fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
        impl CommandBuilder {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
            pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
                let e = match &self.executable{
                    Some(e) => {e},
                    None => {
                        let e: Box<dyn Error> = String::from("no exe").into();
                        return Err(e)
                    },
                };
                let a = match &self.args{
                    Some(a) => {a},
                    None => {
                        let e: Box<dyn Error> = String::from("no args").into();
                        return Err(e)
                    },
                };
                let env = match &self.env{
                    Some(e) => {e},
                    None => {
                        let e: Box<dyn Error> = String::from("no env").into();
                        return Err(e)
                    },
                };
                let cd = match &self.current_dir{
                    Some(c) => {c},
                    None => {
                        let e: Box<dyn Error> = String::from("no current_dir").into();
                        return Err(e)
                    },
                };
                Ok(Command{
                    executable: e.to_owned(),
                    args:a.to_owned(),
                    env:env.to_owned(),
                    current_dir:cd.to_owned(),
                })
            }
        }
    })
}
