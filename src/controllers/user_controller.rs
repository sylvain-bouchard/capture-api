mod models;

#[derive(Clone)]
pub struct UserController {
    user_store: Arc<Mutex<Vec<Option<User>>>>,
}

impl UserController {
    pub async fn new() -> Result<self> {
        Ok(self {
            user_store: Arc::default(),
        })
    }
}
impl UserController {
    pub async fn create_user(&self, user: UserForCreate) -> Result<User> {
        let mut store = self.user_store.lock().unwrap();

        let username = "paul";
        let user = User { username };

        store.push(Some(user.clone()));

        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let store = self.user_store.lock().unwrap();

        let users = store.iter().filter_map(|u| u.clone()).collect();

        Ok(users)
    }

    pub async fn delete_user(&self, username: String) -> Result<User> {
        let mut store = self.user_store.lock().unwrap();

        let user = store.get_mut(username).and_then(|u| u.take());

        user.ok_or(Error::UserNotFound { username })
    }
}
