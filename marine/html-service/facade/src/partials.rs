use crate::PartialConfig;

pub fn partial_list() -> Vec<PartialConfig>{

    let mut list = vec![];

    let p1 = PartialConfig {
        name: String::from("logo"),
        cid: String::from("QmdiBiVqELQ9cbDJGDar4gPBSbSpzrokWs3svKPmBhQ9t7")
    };

    list.push(p1);

    list    

}