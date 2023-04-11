use std::*;

#[derive(Debug, PartialEq, Clone)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: i32,
}

impl User {
    fn get_id(&self) -> i32 {
        return self.id;
    }
    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn get_email(&self) -> &String {
        return &self.email;
    }
    fn get_age(&self) -> i32 {
        return self.age;
    }

    fn set_name(&mut self, name: String) {
        if self.name.eq(&name) {
            panic!("User name provided is same as the user already has");
        }
        self.name = name;
    }

}

/**
 * User repository
 */
pub struct UserRepository {
    users: Vec<User>,
}
/*
 * User repository implementation
 */
impl UserRepository {
    fn new() -> UserRepository {
        let users: Vec<User> = vec![];
        return UserRepository { users };
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn get_user(&self, user_id:i32) -> Option<&User>{
        for usr in self.users.iter() {
            if usr.get_id() == user_id {
                return Some(usr);
            }
        }
        return None;
    }
    fn count(&self) -> usize{
        return self.users.len();
    }
}

fn main() {
    let mut repository = UserRepository::new();
    let user = User {
        id: 1,
        name: String::from("TestUser"),
        email: String::from("testUser@test.com"),
        age: 32
    };
    let user2 = User {
        id: 2,
        name: String::from("TestUser2"),
        email: String::from("testUser2@test.com"),
        age: 30
    };
    repository.add_user(user);
    repository.add_user(user2);
    print!("number of users in the repo : {} \n", repository.count());
   
    let result = repository.get_user(5);
    match result {
        Some(user) => {
            print!("found user with Id: {}", user.get_id());
        },
        None => println!("Not found"),
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_prop() {
        let user1 = User {
            id: 1,
            name: String::from("Test1"),
            email: String::from("test1@test.com"),
            age: 28,
        };

        assert_eq!(user1.get_name(), "Test1");
        assert_eq!(user1.get_email(), "test1@test.com");
        assert_eq!(user1.get_age(), 28);
    }

    #[test]
    fn add_user_to_repository() {
        let mut repository = UserRepository::new();
        let user = User {
            id: 2,
            name: String::from("TestUser"),
            email: String::from("testUser@test.com"),
            age: 32
        };
        repository.add_user(user);
        assert_eq!(repository.count(), 1);
    }

    #[test]
    fn check_if_user_exists_by_id() {
        let mut repository = UserRepository::new();
        let user = User {
            id: 1,
            name: String::from("TestUser"),
            email: String::from("testUser@test.com"),
            age: 32
        };
        repository.add_user(user);
        let result = repository.get_user(1);
        assert!(result.is_some());
    }

    #[test]
    fn check_if_does_not_exist_by_id() {
        let mut repository = UserRepository::new();
        let user3 = User {
            id: 2,
            name: String::from("TestUser3"),
            email: String::from("testUser3@test.com"),
            age: 25
        };
        repository.add_user(user3);
        let result = repository.get_user(3);
        assert!(result.is_none());
    }
    #[test]
    fn get_user_by_id() {
        let mut repository = UserRepository::new();
        let user3 = User {
            id: 2,
            name: String::from("TestUser3"),
            email: String::from("testUser3@test.com"),
            age: 25
        };
        repository.add_user(user3);
        let result = repository.get_user(2);
        if result.is_some() {
            let user = result.unwrap();
            assert_eq!(user.get_id(), 2);
            assert_eq!(user.get_name(), "TestUser3");
        }
    }

    #[test]
    #[should_panic]
    fn update_user() {
        let mut repository = UserRepository::new();
        let mut user3 = User {
            id: 2,
            name: String::from("TestUser3"),
            email: String::from("testUser3@test.com"),
            age: 25
        };
        user3.set_name(String::from("TestUser3"));
        repository.add_user(user3);
    }
}
