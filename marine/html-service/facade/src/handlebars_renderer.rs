use std::collections::BTreeMap;
use handlebars::{ Handlebars, RenderError, handlebars_helper};
use std::path::Path;
use std::fs;
use crate::{ curl_request, partials, log, ipfs};
use crate:: { TemplateDataObject, Value, DataModel };
use crate::helpers::{vote_number};


pub fn single(mut folder_cid: String, data: TemplateDataObject, remote_ipfs_url: &String, elasticsearch_url: &String) -> String {

  let mut html: String = "".to_string();
  let mut handlebars = Handlebars::new();

  let partial_list = partials::partial_list();

  for p in partial_list {

    let partial_str = ipfs::cat(&p.cid, &remote_ipfs_url);
    handlebars.register_partial(&p.name, &partial_str).unwrap(); 
  }
  handlebars.register_helper("vote_number", Box::new(vote_number));

  folder_cid.push_str("/widget.handlebars");

  let source: String = ipfs::block_get(&folder_cid, &remote_ipfs_url);
  handlebars.register_template_string("home_template", source).unwrap();
  
  let render_response = handlebars.render("home_template", &data);

  let render_response = match render_response {

      Ok(res) => { 
          html  = res;
      },
      Err(error) => { 
          log(elasticsearch_url, &format!("Render error: {:?}", error));
      }
  };

  html

}


