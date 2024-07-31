use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct UserData {
    nicname: String,
    avatar_url: Option<String>,
}

impl UserData {
    pub fn new(nicname: String) -> Self {
        Self {
            nicname,
            avatar_url: None,
        }
    }
}
