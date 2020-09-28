use serde_study::*;
static json1:&str= r#"{
    "foo":["bar","baz"],
     "count":32,
     "data":{"user":"Naveen","age":34},
     "authorized":true 
 
}"#;


fn main(){

    let deserialized:JsonStruct = serde_json::from_str(json1).unwrap();
    println!{"{:?},{},{}",deserialized.foo[0],deserialized.authorized,deserialized.data.user};
}