use crate::HelperConfig;
use crate::BigInt; 
use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
use serde_json::Value;


pub fn helper_list() -> Vec<HelperConfig>{

    let mut list = vec![];

    let p1 = HelperConfig {
        name: String::from("logo"),
        cid: String::from("QmdiBiVqELQ9cbDJGDar4gPBSbSpzrokWs3svKPmBhQ9t7")
    };

    list.push(p1);

    list    

}

fn remove_escaped_quote(string: String) -> String {

    string.replace("\"","").to_string()
    
}

pub fn vote_number (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    
    let data = remove_escaped_quote(h.param(0).unwrap().value().to_string());
    
    let b = data.parse::<BigInt>().unwrap();

    let d = "1000000000000000000".parse::<BigInt>().unwrap();
    
    let n = (b / d).to_str_radix(10);

    let v : Value = serde_json::from_str(&n).unwrap();

    out.write(v.render().as_ref())?;
  //  out.write(param.value().render().as_ref())?;
    Ok(())
}
