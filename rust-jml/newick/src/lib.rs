use std::rc::Rc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Reference {
    count: i32,
    cursor: i32,
}

impl Reference {
    fn init() -> Reference {
        Reference {
            count: 0,
            cursor: 0,
        }
    }
}

struct Node {
    leftdesc: Option<Rc<Node>>,
    rightdesc: Option<Rc<Node>>,
    ancestor: Option<Rc<Node>>,
    brlength: Option<f64>,
    popsize: Option<f64>,
    species: String,
    tip: bool,
    // Used when printing trees
    number_of_visits: i32,
    isroot: bool,
}

impl Node {
    fn init() -> Node {
        Node {
            leftdesc: None,
            rightdesc: None,
            ancestor: None,
            isroot: false,
            brlength: None,
            popsize: None,
            tip: false,
            species: "".to_string(),
            number_of_visits: 0,
        }
    }

    //    fn print(&mut self) -> str {
    //        let mut newicktree = "";
    //        if self.tip {
    //            newicktree += self.species;
    //            if let brlength = self.brlength {
    //                newicktree += ":{}".fmt(brlength);
    //            }
    //            if let popsize = self.popsize {
    //                newicktree += " #".fmt(self.popsize);
    //            }
    //            newicktree += self.ancestor.print()
    //        } else {
    //            match self.number_of_visits {
    //                0 => {
    //                    self.number_of_visits = 1;
    //                    newicktree += "(";
    //                    newicktree += self.leftdesc.print();
    //                }
    //                1 => {
    //                    self.number_of_visits = 2;
    //                    newicktree += ",";
    //                    newicktree += self.rightdesc.print();
    //                }
    //                2 => {
    //                    // All descendants have been visited
    //                    self.number_of_visits = 0;
    //                    newicktree += ")";
    //                    if let brlength = self.brlength {
    //                        newicktree += ":{}".fmt(brlength);
    //                    }
    //                    if let popsize = self.popsize {
    //                        newicktree += " #".fmt(self.popsize);
    //                    }
    //                    if !self.isroot {
    //                        newicktree += self.ancestor.print_node();
    //                    } else {
    //                        newicktree += ";";
    //                    }
    //                }
    //            }
    //        }
    //
    //        newicktree
    //    }
}

struct Tree {
    root: Option<Node>,
    newicktree: String,
    // used to store the tree for printing
    cursor: i32,
    tree_string: String,
    // used when parsing the tree
    translate: bool,
    taxnumbers: Vec<String>,
    taxlabels: Vec<String>,
    heredityscalar: f64,
    locusrate: f64,
}

impl Tree {
    fn init() -> Tree {
        Tree {
            root: None,
            cursor: 0,
            newicktree: "".to_string(),
            tree_string: "".to_string(),
            translate: false,
            taxlabels: vec![],
            taxnumbers: vec![],
            heredityscalar: 1.0,
            locusrate: 1.0,
        }
    }
}
