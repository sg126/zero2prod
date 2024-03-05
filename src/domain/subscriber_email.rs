use validator::validate_email;  // TODO: upgrade to V0.17

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        }
        else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// TODO: fix the email property testing
#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    // use rand::prelude::*;
    use claims::assert_err;
    // use fake::faker::internet::en::SafeEmail;
    // use fake::Fake;
    // use proptest::prelude::*;
    // use proptest_derive::Arbitrary;

    // #[derive(Debug, Arbitrary)]
    // struct ValidEmailFixture(pub String);
    //
    // impl Default for ValidEmailFixture {
    //     fn default() -> Self {
    //         // TODO: rand crate might not be the best solution here
    //         let mut rng = thread_rng();
    //         let email = SafeEmail.fake_with_rng();
    //         Self(email)
    //     }
    // }
    //
    // proptest! {
    //     #[test]
    //     fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
    //         SubscriberEmail::parse(valid_email.0).is_ok()
    //     }
    // }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}


