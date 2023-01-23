use crate::{ ContactInfo, DataModel };


pub fn get(index: u16) -> DataModel {

    let mut models: Vec<DataModel> = vec!();

    let contact1 = ContactInfo {
        url: String::from("ipld.public-record.eth")
    };

    let model1 = DataModel {
        name: String::from("Governance"),
        slug: String::from("governance"),
        domain: String::from("ipld.public-record.eth"),
        schema: String::from(""),
        query: String::from(""),
        contact: contact1   
     };

    models.push(model1);

    models[index as usize].clone()

}