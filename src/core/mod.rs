/* Public modules */

pub mod db;
pub mod ratelimiter;
pub mod schemas;
pub mod systems;

use chrono::Local;
use rocket::serde::json::Json;

use self::schemas::{
    AccountResponse, ActionFunction, ActionResponse, Author, ContactFunction, ContactResponse,
    DefaultResponse, DeleteAccount, GetProfile, Index, Login, ProfileResponse, QueueFunction,
    QueueResponse, Register, SetProfile, VerificationCode,
};
use self::systems::{
    delete_account, get_profile_account, login_account, register_account, set_profile_account,
    verify_account,
};

use self::db::{
    add_action_message, add_message_queue_history, contact_add_or_remove, delete_action_message,
    delete_queue_history, fetch_account, get_action_message, get_queue_history,
};

use self::ratelimiter::*;

/* Endpoints */

#[get("/")]
pub async fn index(_rate_limiter: RocketGovernor<'_, IndexRateLimiter>) -> Json<Index<'_>> {
    Json(Index {
        title: "BlackWell API Revamped",
        author: Author {
            name: "Kevin Benavides || DevCheckOG",
            github: "https://github.com/DevCheckOG",
            twitter: "https://twitter.com/DevCheckOG",
        },
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[post("/register", data = "<data>", format = "json")]
pub async fn register_acc<'a>(
    _rate_limiter: RocketGovernor<'a, RegisterRateLimiter>,
    data: Json<Register<'a>>,
) -> Json<DefaultResponse<'a>> {
    let result: DefaultResponse<'_> =
        register_account(data.username, data.email, data.password).await;

    Json(result)
}

#[post("/verify", data = "<data>", format = "json")]
pub async fn verify_acc<'a>(
    _rate_limiter: RocketGovernor<'a, VerifyRateLimiter>,
    data: Json<VerificationCode<'a>>,
) -> Json<DefaultResponse<'a>> {
    let result: DefaultResponse<'_> = verify_account(data.code).await;

    Json(result)
}

#[post("/unregister", data = "<data>", format = "json")]
pub async fn unregister_acc<'a>(
    _rate_limiter: RocketGovernor<'a, UnregisterRateLimiter>,
    data: Json<DeleteAccount<'a>>,
) -> Json<DefaultResponse<'a>> {
    let result: DefaultResponse<'_> = delete_account(data.email, data.password).await;

    Json(result)
}

#[post("/login", data = "<data>", format = "json")]
pub async fn login_acc<'a>(
    _rate_limiter: RocketGovernor<'a, LoginRateLimiter>,
    data: Json<Login<'a>>,
) -> Json<AccountResponse<'a>> {
    let result: AccountResponse<'_> = login_account(data.email, data.password).await;

    Json(result)
}

#[post("/set-profile", data = "<data>", format = "json")]
pub async fn set_profile_acc<'a>(
    _generic_rate_limiter: RocketGovernor<'a, GenericRateLimiter>,
    data: Json<SetProfile<'a>>,
) -> Json<DefaultResponse<'a>> {
    let result: DefaultResponse<'_> = set_profile_account(data.token, data.profile).await;

    Json(result)
}

#[post("/get-profile", data = "<data>", format = "json")]
pub async fn get_profile_acc<'a>(
    _generic_rate_limiter: RocketGovernor<'a, GenericRateLimiter>,
    data: Json<GetProfile<'a>>,
) -> Json<ProfileResponse<'a>> {
    let result: ProfileResponse<'_> = get_profile_account(data.token, data.contact).await;

    Json(result)
}

#[post("/action", data = "<data>", format = "json")]
pub async fn action<'a>(
    _rate_limiter: RocketGovernor<'a, ActionRateLimiter>,
    data: Json<ActionFunction<'a>>,
) -> Json<ActionResponse<'a>> {
    let valid_actions: Vec<&str> = vec!["add", "get", "delete"];

    if !valid_actions.contains(&data.action) {
        return Json(ActionResponse {
            title: "BlackWell API Revamped",
            message: "The action does not exist.",
            success: false,
            request: None,
            action: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }

    if (fetch_account(data.email, data.password).await).is_some() {
        match data.action {
            "add" => {
                if data.add.is_some() {
                    let response: bool = add_action_message(
                        data.add.clone().unwrap().to,
                        data.add.clone().unwrap().action,
                    )
                    .await;

                    return Json(ActionResponse {
                        title: "BlackWell API Revamped",
                        message: "The action was executed.",
                        success: response,
                        request: Some("add"),
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(ActionResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional add paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "get" => {
                if data.get.is_some() {
                    if let Some(actions) = get_action_message(data.get.as_ref().unwrap().to).await {
                        return Json(ActionResponse {
                            title: "BlackWell API Revamped",
                            message: "The action was found.",
                            success: true,
                            request: Some("get"),
                            action: Some((Some(actions), None)),
                            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        });
                    }

                    return Json(ActionResponse {
                        title: "BlackWell API Revamped",
                        message: "The actions was not found.",
                        success: false,
                        request: None,
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(ActionResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional get paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "delete" => {
                if data.delete.is_some() {
                    let response: bool =
                        delete_action_message(data.delete.clone().unwrap().to).await;

                    return Json(ActionResponse {
                        title: "BlackWell API Revamped",
                        message: "The delete action was executed.",
                        success: response,
                        request: Some("delete"),
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(ActionResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional delete paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }

            _ => {
                return Json(ActionResponse {
                    title: "BlackWell API Revamped",
                    message: "The action does not exist.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
        }
    }

    Json(ActionResponse {
        title: "BlackWell API Revamped",
        message: "Your credentials is not valid.",
        success: false,
        request: None,
        action: None,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[post("/queue", data = "<data>", format = "json")]
pub async fn queue<'a>(
    _rate_limiter: RocketGovernor<'a, QueueRateLimiter>,
    data: Json<QueueFunction<'a>>,
) -> Json<QueueResponse<'a>> {
    let valid_actions: Vec<&str> = vec!["add", "get", "delete"];

    if !valid_actions.contains(&data.action) {
        return Json(QueueResponse {
            title: "BlackWell API Revamped",
            message: "The action does not exist.",
            success: false,
            request: None,
            action: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }

    if (fetch_account(data.email, data.password).await).is_some() {
        match data.action {
            "add" => {
                if data.add.is_some() {
                    let response: bool = add_message_queue_history(
                        data.add.clone().unwrap().to,
                        data.add.clone().unwrap().message,
                    )
                    .await;

                    return Json(QueueResponse {
                        title: "BlackWell API Revamped",
                        message: "The action add queue was executed.",
                        success: response,
                        request: Some("add"),
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(QueueResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional add paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "get" => {
                if data.get.is_some() {
                    if let Some(actions) = get_queue_history(data.get.as_ref().unwrap().token).await
                    {
                        return Json(QueueResponse {
                            title: "BlackWell API Revamped",
                            message: "The action get queue executed.",
                            success: true,
                            request: Some("get"),
                            action: Some((Some(actions), None)),
                            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        });
                    }

                    return Json(QueueResponse {
                        title: "BlackWell API Revamped",
                        message: "The actions was not found.",
                        success: false,
                        request: None,
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(QueueResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional get paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "delete" => {
                if data.delete.is_some() {
                    let response: bool =
                        delete_queue_history(data.delete.clone().unwrap().to).await;

                    return Json(QueueResponse {
                        title: "BlackWell API Revamped",
                        message: "The action delete queue was executed.",
                        success: response,
                        request: Some("delete"),
                        action: None,
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(QueueResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional delete paramenter is not valid.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }

            _ => {
                return Json(QueueResponse {
                    title: "BlackWell API Revamped",
                    message: "The action does not exist.",
                    success: false,
                    request: None,
                    action: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
        }
    }

    Json(QueueResponse {
        title: "BlackWell API Revamped",
        message: "Your credentials is not valid.",
        success: false,
        request: None,
        action: None,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[post("/contact", data = "<data>", format = "json")]
pub async fn contact<'a>(
    _rate_limiter: RocketGovernor<'a, ContactRateLimiter>,
    data: Json<ContactFunction<'a>>,
) -> Json<ContactResponse<'a>> {
    let valid_actions: Vec<&str> = vec!["add", "get", "delete"];

    if !valid_actions.contains(&data.action) {
        return Json(ContactResponse {
            title: "BlackWell API Revamped",
            message: "The action does not exist.",
            success: false,
            request: None,
            response: None,
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        });
    }

    if let Some(_acc) = fetch_account(data.email, data.password).await {
        match data.action {
            "add" => {
                if data.add.is_some() {
                    let response: bool = contact_add_or_remove(
                        data.add.as_ref().unwrap().action,
                        data.add.as_ref().unwrap().from,
                        data.add.as_ref().unwrap().to,
                    )
                    .await;

                    return Json(ContactResponse {
                        title: "BlackWell API Revamped",
                        message: "The action add was executed.",
                        success: response,
                        request: Some("add"),
                        response: Some(response),
                        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    });
                }

                return Json(ContactResponse {
                    title: "BlackWell API Revamped",
                    message: "The optional add paramenter is not valid.",
                    success: false,
                    request: None,
                    response: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }

            _ => {
                return Json(ContactResponse {
                    title: "BlackWell API Revamped",
                    message: "The action does not exist.",
                    success: false,
                    request: None,
                    response: None,
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
        }
    }

    Json(ContactResponse {
        title: "BlackWell API Revamped",
        message: "Your credentials is not valid.",
        success: false,
        request: None,
        response: None,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

/* Catchers */
#[catch(404)]
pub fn not_found<'a>() -> Json<DefaultResponse<'a>> {
    Json(DefaultResponse {
        title: "BlackWell API Revamped",
        message: "The endpoint does not exist.",
        success: false,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[catch(429)]
pub fn too_many_requests<'a>() -> Json<DefaultResponse<'a>> {
    Json(DefaultResponse {
        title: "BlackWell API Revamped",
        message: "You are being rate-limited.",
        success: false,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

#[catch(422)]
pub fn unprocessable_entity<'a>() -> Json<DefaultResponse<'a>> {
    Json(DefaultResponse {
        title: "BlackWell API Revamped",
        message: "The request could not be processed. Due to its incorrect syntax.",
        success: false,
        date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}
