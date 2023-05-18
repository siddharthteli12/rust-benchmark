#[cfg(test)]
mod tests {
    use crate::{buffer, read_to_string};
    #[test]
    #[should_panic]
    fn test_buffer_with_empty_input() {
        assert_eq!(buffer(String::new(), String::new()).unwrap(), vec![]);
    }

    #[test]
    #[should_panic]
    fn test_read_with_empty_input() {
        assert_eq!(
            read_to_string(String::new(), String::new()).unwrap(),
            vec![]
        );
    }

    #[test]
    fn test_buffer_with_non_existent_pattern() {
        let pattern = "ASDasdasdas".to_string();
        let file = "test.txt".to_string();
        assert_eq!(buffer(pattern, file).unwrap(), vec![]);
    }

    #[test]
    fn test_read_with_non_existent_pattern() {
        let pattern = "ASDasdasdas".to_string();
        let file = "test.txt".to_string();
        assert_eq!(read_to_string(pattern, file).unwrap(), vec![]);
    }

    #[test]
    fn test_buffer_with_pattern() {
        let pattern = "2IBK".to_string();
        let file = "test.txt".to_string();
        let result_vec = vec![(259, "KBohAeQPRh 2IBKwIMvv6SvopeUrewceXQjhL4gf87jq9coBTd976PQU5YlmENuYAuOZ4QhWyWT575SUsqY27Tf1D54NrTEVke0G".to_string()), 
    (76931, "hoTJW35juu2CKMDtCiP0liBmzRCQzDRd0ZMMvvgwmMcJ2IBKOq42ReONGORav2yQCatQDEnCIijLNIE TaVJ kbVXQpvDh UAzdH".to_string()),
    (93028, "NabE0 8deQix1kGcwRDWy6LlOGLXnEIWYN2OjGhK19gzIGsR zQQ9sTnrwKAzqIv3fIY3Xddf6Q5SY82IBKSvoKq9732OWvjAERn".to_string())
    ];
        assert_eq!(buffer(pattern, file).unwrap(), result_vec);
    }

    #[test]
    fn test_read_with_pattern() {
        let pattern = "2IBK".to_string();
        let file = "test.txt".to_string();
        let result_vec = vec![(259, "KBohAeQPRh 2IBKwIMvv6SvopeUrewceXQjhL4gf87jq9coBTd976PQU5YlmENuYAuOZ4QhWyWT575SUsqY27Tf1D54NrTEVke0G".to_string()), 
    (76931, "hoTJW35juu2CKMDtCiP0liBmzRCQzDRd0ZMMvvgwmMcJ2IBKOq42ReONGORav2yQCatQDEnCIijLNIE TaVJ kbVXQpvDh UAzdH".to_string()),
    (93028, "NabE0 8deQix1kGcwRDWy6LlOGLXnEIWYN2OjGhK19gzIGsR zQQ9sTnrwKAzqIv3fIY3Xddf6Q5SY82IBKSvoKq9732OWvjAERn".to_string())
    ];
        assert_eq!(read_to_string(pattern, file).unwrap(), result_vec);
    }
}
