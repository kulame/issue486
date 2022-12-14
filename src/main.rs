use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Image {
    #[serde(rename = "$unflatten=MediaId")]
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase", tag="MsgType")]
pub enum Message {
    #[serde(rename_all = "PascalCase")]
    Image {
        to_user_name: String,
        from_user_name: String,
        create_time: i64,
        image: Image,
    },

    #[serde(rename_all = "PascalCase")]
    Text {
        to_user_name: String,
        from_user_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Xml {
    pub xml: Message,
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::Message;

    use super::Xml;
    
    #[test]
    fn test_standard_message() {
        let xml = r#"
            <xml>
                <ToUserName><![CDATA[gh_1e11f4ae566b]]></ToUserName>
                <FromUserName><![CDATA[o-Wld6YNY-H801WBEiW-P9s1MzaU]]></FromUserName>
                <MsgType><![CDATA[text]]></MsgType>
            </xml>
        "#;
        let event: Message = quick_xml::de::from_str(xml).unwrap();
        match event {
            Message::Text { to_user_name, .. } => {
                assert!(to_user_name == "gh_1e11f4ae566b")
            }
            _ => assert!(false),
        }
    }
}