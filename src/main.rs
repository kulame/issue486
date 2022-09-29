use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Image {
    #[serde(rename = "$unflatten=MediaId")]
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Message {
    #[serde(rename_all = "PascalCase")]
    ImageReplyMessage {
        to_user_name: String,
        from_user_name: String,
        create_time: i64,
        msg_type: String,
        image: Image,
    },

    #[serde(rename_all = "PascalCase")]
    StandardMessage {
        to_user_name: String,
        from_user_name: String,
        create_time: u64,
        msg_type: String,
        content: String,
        msg_id: u64,
        msg_data_id: Option<String>,
        idx: Option<i32>,
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
                <CreateTime>1664344892</CreateTime>
                <MsgType><![CDATA[text]]></MsgType>
                <Content><![CDATA[@dd]]></Content>
                <MsgId>23827690509052792</MsgId>
            </xml>
        "#;
        let event: Xml = quick_xml::de::from_str(xml).unwrap();
        match event.xml {
            Message::StandardMessage { to_user_name, .. } => {
                assert!(to_user_name == "gh_1e11f4ae566b")
            }
            _ => assert!(false),
        }
    }
}