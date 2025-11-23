use mockall::mock;

pub trait Database {
    fn get_data(&self) -> String;
}

mock! {
    pub Database {
        fn get_data(&self) -> String;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_database() {
        let mut mock_db = MockDatabase::new();
        mock_db.expect_get_data()
            .returning(|| "mocked data".to_string());
        
            assert_eq!(mock_db.get_data(), "mocked data");     
    }
}