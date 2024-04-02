use super::db::*;
use super::schemas::*;

use chrono;
use rand::Rng;
use regex::Regex;
use uuid;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use chrono::Local;

/* EMAIL SYSTEM */

pub async fn send_email(to: &str, message: &str) -> bool {
    if !to.ends_with("@gmail.com") {
        return false;
    } else if let Some(code) = secret("gmail code").await {
        if let Some(gmail) = secret("gmail").await {
            let email: Message = Message::builder()
                .from(gmail.secret.as_str().parse().unwrap())
                .to(to.parse().unwrap())
                .subject("BlackWell API | Verification Code")
                .header(ContentType::TEXT_PLAIN)
                .body(message.to_string())
                .unwrap();

            let creds: Credentials =
                Credentials::new(gmail.secret.to_owned(), code.secret.to_owned());

            let mail: SmtpTransport = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            return mail.send(&email).is_ok();
        }
    }

    false
}

/* ACCOUNT SYSTEM */

pub async fn register_account<'a>(
    username: &'a str,
    email: &'a str,
    password: &'_ str,
) -> DefaultResponse<'a> {
    let random_code: String = gen_random_six_number().await;

    let tmp_usr: TemporalAccount = TemporalAccount {
        id: uuid::Uuid::new_v4().to_string(),
        profile: "".to_string(),
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        created_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        verification: Verification {
            remaining: (chrono::Local::now() + chrono::Duration::minutes(3))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            code: random_code.clone().to_string(),
        },
        contacts: vec![],
    };

    let fpu: bool = find_possible_account(username, email).await;

    if !fpu {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The user already exists.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    let send_email: bool = send_email(email, random_code.clone().as_str()).await;

    if !send_email {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The mensage to the email could not be send.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    let ptu_rs: bool = post_temp_account(tmp_usr).await;

    if !ptu_rs {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The temporary user could not be created.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    return DefaultResponse {
        title: "BlackWell API Revamped",
        message: "You have 3 minutes to check the email.",
        success: true,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
}

pub async fn verify_account(code: &str) -> DefaultResponse {
    let is_valid: bool = is_valid_verification_code(code).await;

    if !is_valid {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The verification code is invalid.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    } else if let Some(tmp_usr) = get_temp_account(code).await {
        let usr: Account = Account {
            id: tmp_usr.id.clone(),
            profile: tmp_usr.profile.clone(),
            username: tmp_usr.username.clone(),
            email: tmp_usr.email.clone(),
            password: tmp_usr.password.clone(),
            created_at: tmp_usr.created_at.clone(),
            contacts: tmp_usr.contacts.clone(),
        };

        let result: bool = post_account(usr).await;

        if !result {
            return DefaultResponse {
                title: "BlackWell API Revamped",
                message: "The user could not be created.",
                success: false,
                date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            };
        }

        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The user has been created.",
            success: true,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    DefaultResponse {
        title: "BlackWell API Revamped",
        message: "Unknown error.",
        success: false,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }
}

pub async fn delete_account<'a>(email: &'a str, password: &'a str) -> DefaultResponse<'a> {
    let result: bool = delete_acc(email, password).await;

    if !result {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The account could not be deleted.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    return DefaultResponse {
        title: "BlackWell API Revamped",
        message: "The account has been deleted.",
        success: true,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
}

pub async fn login_account<'a>(email: &'a str, password: &'a str) -> AccountResponse<'a> {
    let result: Option<Account> = get_account_with_email_and_password(email, password).await;

    if result.is_none() {
        return AccountResponse {
            title: "BlackWell API Revamped",
            message: "The account could not be found.",
            success: false,
            account: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    return AccountResponse {
        title: "BlackWell API Revamped",
        message: "The account has been found.",
        success: true,
        account: Some(result.unwrap()),
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
}

/* MODIFY DATA OF THE ACCOUNTS */

pub async fn set_profile_account<'a>(token: &'a str, profile: &'a str) -> DefaultResponse<'a> {
    let hex: bool = is_hex(profile).await;

    if !hex {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The profile is invalid; it must be hexadecimal string.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    let size: bool = check_img_or_video_size(profile.as_bytes()).await;

    if !size {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The profile is invalid; it must be less than 5 MB.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    let result: bool = set_account_profile(token, profile).await;

    if !result {
        return DefaultResponse {
            title: "BlackWell API Revamped",
            message: "The profile could not be updated.",
            success: false,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    return DefaultResponse {
        title: "BlackWell API Revamped",
        message: "The profile has been updated.",
        success: true,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
}

/* GET INFO OF AN DETERMINED ACCOUNT */

pub async fn get_profile_account<'a>(token: &'a str, contact: &'a str) -> ProfileResponse<'a> {
    let contacts: bool = check_if_account_in_contacts(token, contact).await;

    if !contacts {
        return ProfileResponse {
            title: "BlackWell API Revamped",
            message: "The contact does not formulate in the account.",
            success: false,
            profile: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    let result: Option<String> = get_account_profile(contact).await;

    if result.is_none() {
        return ProfileResponse {
            title: "BlackWell API Revamped",
            message: "The profile could not be found.",
            success: false,
            profile: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
    }

    return ProfileResponse {
        title: "BlackWell API Revamped",
        message: "The profile has been found.",
        success: true,
        profile: Some(result.unwrap()),
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };
}

/* UTIL TOOLS */

async fn gen_random_six_number() -> String {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let nums: (i32, i32) = (rng.gen_range(1000..=9999), rng.gen_range(1000..=9999));
    nums.0.to_string() + &nums.1.to_string()
}

async fn is_hex(string: &str) -> bool {
    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    re.is_match(string)
}

async fn check_img_or_video_size(img_or_video: &[u8]) -> bool {
    if (img_or_video.len() as f64 / 1048576.0).round() <= 5.0 {
        return true;
    }

    false
}
