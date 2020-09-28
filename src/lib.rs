

use serde_json::{Value};
use serde::{Serialize,Deserialize};
///Sample json data
///It has a array of elements,integer,inner struct,bool types
static json1:&str= r#"{
       "foo":["bar","baz"],
        "count":32,
        "data":{"user":"Naveen","age":34},
        "authorized":true 
    
}"#;


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
    #[serde(skip)]
    count:i32,
    //#[serde(borrow)]
    pub data: User<'a>,
    pub authorized:bool,
}
///if you want to define a field which has different name
/// below struct has field "fun" for the jason key "foo"
///   
#[derive(Debug,Deserialize)]
pub struct User<'a>{
    pub user:&'a str,
    age:i32,
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn json_test_exampl1() {

        let deserialized:JsonStruct = serde_json::from_str(json1).unwrap();
        println!{"{:?},{},{}",deserialized.foo[0],deserialized.authorized,deserialized.data.user};
    }
    #[test]
    fn json_test_rename_field() {

        let deserialized:JsonStruct2 = serde_json::from_str(json1).unwrap();
        println!("{:?},{},{},{}",deserialized.fun[0],deserialized.authorized,deserialized.data.user,deserialized.count);
    }

    #[test]
    fn json_test_skip_field() {

        let deserialized:JsonStruct3 = serde_json::from_str(json1).unwrap();
        println!("{:?},{},{},{}",deserialized.fun[0],deserialized.authorized,deserialized.data.user,deserialized.count);
    }



}
