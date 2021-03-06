pub mod sniper {
    //To deserialize snippets from file(s)
    use serde::Deserialize;
    use std::clone::Clone;
    use std::fs;
    use std::path::PathBuf;
    use toml;
    //TODO: switch to dashmap, separate into two separate maps
    use radix_trie::Trie;
    use std::collections::HashSet;

    //these are the currently supported actions for snippets
    #[derive(Deserialize, Clone, Debug)]
    #[serde(tag = "action", content = "args")]
    pub enum Actions {
        Load(Vec<String>),
        Enable(Vec<String>),
        Disable(Vec<String>),
        //Commands, //potentially script running of commands such as making or renaming a file
        //Adjust, //shouldn't be in file, there to tell sniper to reparse at snippet launch time
    }
    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename = "type")]
    pub enum SnippetTypes {
        Shorthand,
        Statement,
        Expression,
        Template,
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Snippet {
        name: String,
        #[serde(rename = "type", default = "default_snippet_type")]
        snippet_type: SnippetTypes,
        body: Vec<String>,
        description: String,
        #[serde(default = "unconditional")]
        is_conditional: bool,
        #[serde(default = "no_action")]
        actions: Vec<Actions>,
    }

    fn default_snippet_type() -> SnippetTypes {
        SnippetTypes::Shorthand
    }

    fn unconditional() -> bool {
        false
    }

    fn no_action() -> Vec<Actions> {
        Vec::new()
    }

    #[derive(Deserialize, Clone, Debug)]
    pub struct Loader {
        #[serde(flatten, with = "tuple_vec_map")]
        snippets: Vec<(String, Snippet)>,
    }

    #[derive(Debug)]
    pub struct Sniper {
        //TODO: since config is now static, switch to passing sessions
        config: PathBuf,
        pub language: String,
        snippets: radix_trie::Trie<String, Snippet>,
        enabled_conditionals: HashSet<String>,
    }

    impl Sniper {
        pub fn new(config) -> Self {
            Self {
                config: PathBuf::from(config_path),
                language: String::new(),
                snippets: Trie::new(),
                enabled_conditionals: HashSet::new(),
            }
        }
        /*pub fn set_target(&mut self, target_file: &str) {
            self.target = PathBuf::from(target_file);

            if let Some(suffix) = self.target.extension() {
                match suffix.to_str() {
                    Some("py") => self.set_language("python"),
                    //Some(".py") => self.set_language("cpp"),
                    _ => self.set_language("unsupported"),
                }
            } else {
                self.set_language("undefined")
            }
        }*/
        pub fn set_language(&mut self, language: &str) {
            self.language = language.to_string();
            if language == "unsupported" {
                //TODO: define some way to tell everything to stop
                println!("Hmmm");
            } else if language == "undefined" {
                //TODO: set up some way of loading all templates for all languages and nothing else
                self.load("nope")
            } else {
                let templatesjoin("templates")_dir = self.config.join(self.language.clone()).;
                if templates_dir.is_dir() {
                    for entry in fs::read_dir(templates_dir).expect("unable to list") {
                        let entry = entry.expect("unable to get entry");
                        let file = entry
                            .file_name()
                            .into_string()
                            .expect("could not convert to str");
                        println!("{}", file);
                        self.load(&format!("{}{}", "templates/", &file));
                    }
                } else {
                    println!("{:?}", &templates_dir);
                }
                self.load("base.toml");
            }
        }

        pub fn load(&mut self, file: &str) {
            let snippet_file = self.config.join(self.language.clone()).join(file);
            if snippet_file.is_file() {
                println!("yeet {:?}", snippet_file);
                let snippet_data = fs::read_to_string(&snippet_file).expect("failed to load file");
                //TODO: explore using serde_json StreamDeserializer
                /*let mut temp: Loader = if file.endsWith(".toml") {
                   toml::from_str(&snippet_data).unwrap()
                } else if file.endsWith(".json") {
                   serde_json::from_str(&snippet_data).unwrap()
                }*/
                let mut temp: Loader = toml::from_str(&snippet_data).unwrap();
                while let Some(pair) = temp.snippets.pop() {
                    //self.adjust()
                    self.snippets.insert(pair.0, pair.1);
                }
            } else {
                println!("{:?}", snippet_file);
            }
        }
        //TODO: figure out what to do about editing
        //technically every one of the keywords intended output
        //can change by insertion time
        /*fn adjust(snip: &Snippet) {
            let mut count=0;
            for line in 0..snip.body.len(){
                for left_b in 0..snip.body[line].len(){
                    if snip.body[line][left_b]=="$"{

                        if snip.body[line][left_b]
                    }
                }
            }

        }*/
        pub fn get(&mut self, input: &str) -> Option<Snippet> {
            self.snippets.get_mut(input).cloned()
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("running test");
        use super::sniper;
        use std::env;
        //TODO: switch to using snippets located in $HOME/.config/sniper
        let mut dir = env::current_exe().unwrap();
        dir.pop();
        dir.pop();
        dir.pop();
        dir.pop();
        {
            println!("{:?}", dir.to_str());
            //NOTE: snippets were originally in ~/.config but move to repo
            //for others to have some snippets to test with
            let mut sniper =
                sniper::Sniper::new(&format!("{}{}", dir.to_str().unwrap(), "/snippets/"));
            sniper.set_language("python");
            //let snippets: RefCell<Vec<Snippets>> = RefCell::new(vec![]);
            //sniper.walk(|n|snippets.borrow_mut().push(n.clone()));
            let if_snip = sniper.get("if");
            println!("{:?}", if_snip);
        }
        assert_eq!(2 + 2, 4);
    }
}
