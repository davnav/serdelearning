

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
    foo:Vec<&'a str>,
    count:i32,
    //#[serde(borrow)]
    data: User<'a>,
    authorized:bool,
}

#[derive(Debug,Deserialize)]
pub struct User<'a>{
    user:&'a str,
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
}
