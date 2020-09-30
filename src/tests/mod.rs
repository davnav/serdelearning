//Test Module!
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;
     #[test]

     ///learning1:
     /// how to construct a rust struct and deserialize it .
    fn json_test_exampl1() {

        let deserialized:JsonStruct = serde_json::from_str(json1).unwrap();
        println!{"{:?},{},{}",deserialized.foo[0],deserialized.authorized,deserialized.data.user};
    }
    #[test]

    ///learning2:
    /// How to deserialize json with renamed fields
    fn json_test_rename_field() {

        let deserialized:JsonStruct2 = serde_json::from_str(json1).unwrap();
        println!("{:?},{},{},{}",deserialized.fun[0],deserialized.authorized,deserialized.data.user,deserialized.count);
    }

    #[test]

    ///learning 3:
    /// How to deserialize json and skip a field
    fn json_test_skip_field() {

        let deserialized:JsonStruct3 = serde_json::from_str(&json1).unwrap();
        println!("{:?},{},{},{}",deserialized.fun[0],deserialized.authorized,deserialized.data.user,deserialized.count);
    }

    #[test]

    ///learning 4 
    /// How to rename all fields and deserialize a json data
     fn json_test_rename_all_fields() {

        let deserialized:JsonStruct4 = serde_json::from_str(&json2).unwrap();
        println!("{:?},{},{}",deserialized.foo[0],deserialized.AUTHORIZED,deserialized.DATA.user);
    }

    #[test]


    ///learning 5:
    ///How to use enum for different json data
    /// how to internally tag json while serializing 
    /// 
    /// refer - https://serde.rs/enum-representations.html
     fn json_test_enum_tag() {

        let mes = Message::Request{id:"1".to_string(),method:"post".to_string(),params:Params{users:"US".to_string(),name:"naveen".to_string()}};
        let serialized = serde_json::to_string(&mes).unwrap();
        ///internal representation of json will have type as key 
        println!("{:?}",serialized);

        let deserialized:Message = serde_json::from_str(&serialized).unwrap();
        ///deserialized json get printed
        println!("{:?}",deserialized);
    }


}

