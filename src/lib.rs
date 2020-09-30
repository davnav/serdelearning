

use serde_json::{Value};
use serde::{Serialize,Deserialize};
mod tests;

///Sample json data
///It has a array of elements,integer,inner struct,bool types
static json1:&str= r#"{
       "foo":["bar","baz"],
        "count":32,
        "data":{"user":"Naveen","age":34},
        "authorized":true 
    
}"#;

static json2:&str= r#"{
       "FOO":["bar","baz"],
        "COUNT":32,
        "DATA":{"user":"Naveen","age":34},
        "AUTHORIZED":true 
    
}"#;

///adding the attribute #serde(tag="type") shows the json internally tagged
#[derive(Serialize,Deserialize,Debug)]
#[serde(tag="type")]
pub enum Message{
    Request{ id:String,method:String,params:Params},
    Response{id:String,result:Value},
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Params{
    users:String,
    name:String,    

} 
///For the example1 json data can be represented as below in rust struct.
/// Since &str inside the vector require lifetime, this needs to explicitly borrowed.
/// Note the json field key values and struct type should match
#[derive(Debug,Deserialize)]
pub struct JsonStruct<'a>{
    #[serde(borrow)]
    pub foo:Vec<&'a str>,
    count:i32,
    //#[serde(borrow)]
    pub data: User<'a>,
    pub authorized:bool,
}

///if you want to define a field which has different name
/// #[serde(rename="original json key")]
/// the above attribute needs to be used.
/// below struct has field "fun" for the jason key "foo"
#[derive(Debug,Deserialize)]
pub struct JsonStruct2<'a>{
    #[serde(borrow)]
    #[serde(rename="foo")]
    pub fun:Vec<&'a str>,
    count:i32,
    //#[serde(borrow)]
    pub data: User<'a>,
    pub authorized:bool,
}
///if you want to define a field which has different name
/// below struct has field "fun" for the jason key "foo"
///   
///JsonStruct3 example , we are dealing with skipping some fields
///Some fields are in the json data might not needed for us, so we can simply
/// skip them using #[serde(skip)] attribute
#[derive(Debug,Deserialize)]
pub struct JsonStruct3<'a>{
    #[serde(borrow)]
    #[serde(rename="foo")]
    pub fun:Vec<&'a str>,
   // #[serde(skip)]
    count:i32,
    //#[serde(borrow)]
    pub data: User<'a>,
    pub authorized:bool,
}

///rename all fields with some specifics
/// in this case json data should have rust struct's fields 
/// uppercase corresponding keys.Our data is below and keys are uppercase
/// static json2:&str= r#"{
///             "FOO":["bar","baz"],
///             "COUNT":32,          
///             "DATA":{"user":"Naveen","age":34},
///             "AUTHORIZED":true 
/// }"#;
/// 
#[derive(Debug,Deserialize)]
#[serde(rename_all="UPPERCASE")]
#[serde(deny_unknown_fields)]
pub struct JsonStruct4<'a>{
    #[serde(borrow)]
    pub foo :Vec<&'a str>,
    count:i32,
    //#[serde(borrow)]
    pub DATA : User<'a>,
    pub AUTHORIZED:bool,
}


///if you want to define a field which has different name
/// below struct has field "fun" for the jason key "foo"
///   
#[derive(Debug,Deserialize)]
pub struct User<'a>{
    pub user:&'a str,
    age:i32,
}


