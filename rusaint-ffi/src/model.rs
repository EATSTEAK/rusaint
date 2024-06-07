#[derive(uniffi::Object)]
pub struct USaintSession(rusaint::USaintSession);


pub struct USaintSessionBuilder {
    id: String,
}

impl USaintSessionBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }

    pub async fn build_with_password(password: &str) -> USaintSession {
        todo!()
    }

    pub async fn build_with_token(token: &str) -> USaintSession {
        todo!()
    }

}