use crate::widgets::message_list::{ChatMessage, ChatRole};
use chrono::Local;

pub struct MessagesStorage {
    pub entries: Vec<ChatMessage>,
}

impl MessagesStorage {
    pub fn append_message(&mut self, role: ChatRole, content: String) {
        self.entries.push(ChatMessage {
            role,
            content,
            timestamp: Local::now(),
        });
    }
}

impl Default for MessagesStorage {
    fn default() -> Self {
        Self {
            entries: vec![ChatMessage {
                content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vitae orci sed dui luctus cursus ac non odio. Etiam id faucibus lectus, sit amet tincidunt ipsum. Nunc malesuada bibendum felis id rutrum. Maecenas magna nulla, scelerisque ac augue id, fermentum interdum diam. Fusce nec laoreet lectus. Etiam id hendrerit nisi. Quisque scelerisque dui eu dictum lobortis. Fusce turpis metus, pulvinar ut justo pellentesque, faucibus convallis nulla. Fusce non porta ipsum.

Phasellus rhoncus orci urna, ac ullamcorper ipsum malesuada nec. Aenean ac malesuada lorem. Phasellus ut nulla erat. Praesent eget velit ut sapien sagittis sagittis vehicula vel turpis. Cras sed est fringilla, porta justo sit amet, dictum nisl. Quisque sodales tellus nec cursus elementum. Morbi dapibus sagittis eros, tempor varius lacus laoreet viverra. Proin lacinia nisi metus, eget vulputate quam posuere et. Quisque egestas nibh vitae pretium sodales. Phasellus convallis nec purus ut feugiat. Fusce molestie tincidunt sapien, eget tristique augue pretium sed. Curabitur imperdiet quam vel hendrerit viverra. Nam sit amet nibh eu est elementum pulvinar vitae vitae elit. Maecenas tempus rhoncus vehicula. Aenean vitae commodo dolor. Sed quis lacinia magna.

Praesent suscipit nulla eget est aliquet, vehicula rutrum nunc gravida. Etiam bibendum eget magna eu finibus. Vestibulum auctor, nunc sit amet gravida sagittis, ligula est dignissim justo, vitae cursus mi orci ut mi. Duis ac fringilla arcu. Morbi interdum felis sed diam dapibus, id congue diam posuere. Nam laoreet nisi eget porta rutrum. Nulla interdum ultrices risus sit amet porta. Fusce laoreet ex eget sem lacinia, sed aliquet quam cursus. Nunc rhoncus vel tortor non laoreet. Maecenas lobortis ligula tellus, eget vestibulum velit ultrices a. Quisque maximus erat velit, vitae vehicula magna rhoncus eget. Ut auctor, ante in fringilla pellentesque, neque libero convallis dui, a ornare nulla justo ut leo. Integer in lobortis ipsum, eget pharetra velit. Praesent aliquet placerat mattis.".to_string(),
                timestamp: Local::now(),
                role: ChatRole::App,
            },
                          ChatMessage {
                              content: "asdlkansdkjabndjkasLorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vitae orci sed dui luctus cursus ac non odio. Etiam id faucibus lectus, sit amet tincidunt ipsum. Nunc malesuada bibendum felis id rutrum. Maecenas magna nulla, scelerisque ac augue id, fermentum interdum diam. Fusce nec laoreet lectus. Etiam id hendrerit nisi. Quisque scelerisque dui eu dictum lobortis. Fusce turpis metus, pulvinar ut justo pellentesque, faucibus convallis nulla. Fusce non porta ipsum.".to_string(),
                              timestamp: Local::now(),
                              role: ChatRole::User,
                          }]
        }
    }
}
