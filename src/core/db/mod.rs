use mongodb::{
    bson::doc, options::ClientOptions, results::UpdateResult, Client, Collection, Database,
};

use chrono::{Datelike, Local, NaiveDateTime};
use futures::TryStreamExt;
use tokio::time::{sleep, Duration};

pub mod schemas;
pub use schemas::*;

pub struct DB {
    client: Client,
    systems: Database,
    users: Database,
    messages: Database,
}

pub static mut MAIN_DB: Option<DB> = None;

pub async fn init() {
    if let Ok(mut client_options) = ClientOptions::parse(dotenv::var("DB").unwrap()).await {
        client_options.max_pool_size = Some(200);

        if let Ok(client) = Client::with_options(client_options) {
            unsafe {
                MAIN_DB = Some(DB {
                    client: client.clone(),
                    systems: client.clone().database("systems"),
                    users: client.clone().database("users"),
                    messages: client.clone().database("messages"),
                })
            };
            return;
        }
    }

    panic!("Failed to connect to MongoDB.");
}

/* PRIMARY DATABASE */

pub async fn secret(id: &str) -> Option<Secret> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Secret> = db.systems.collection::<Secret>("system");
    let result: Option<Secret> = collection.find_one(doc! {"_id": id}, None).await.ok()?;

    result
}

pub async fn post_account(user: Account) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(_result) = collection.insert_one(user, None).await {
        return true;
    }

    false
}

pub async fn get_account(id: &str) -> Option<Account> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    let result: Option<Account> = collection.find_one(doc! {"_id": id}, None).await.ok()?;

    result
}

pub async fn fetch_account(email: &str, password: &str) -> Option<String> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    let result: Option<Account> = collection
        .find_one(doc! { "email": email, "password": password }, None)
        .await
        .unwrap_or(None);

    if !result.is_none() {
        return Some(result.unwrap().id);
    }

    None
}

pub async fn get_account_with_email_and_password(email: &str, password: &str) -> Option<Account> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    let result: Option<Account> = collection
        .find_one(doc! { "email": email, "password": password }, None)
        .await
        .ok()?;

    result
}

pub async fn delete_acc(email: &str, password: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(_result) = collection
        .delete_one(doc! { "email": email, "password": password }, None)
        .await
    {
        return true;
    }

    false
}

pub async fn get_token_with_email_and_password(email: &str, password: &str) -> Option<Vec<String>> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(rs) = collection
        .find_one(doc! { "email": email, "password": password }, None)
        .await
    {
        if let Some(user) = rs {
            return Some(vec![user.clone().id, user.clone().username]);
        }
    }

    None
}

pub async fn get_token_with_username(username: &str) -> Option<Vec<String>> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(rs) = collection
        .find_one(doc! { "username": username }, None)
        .await
    {
        if let Some(user) = rs {
            return Some(vec![user.clone().id, user.clone().username]);
        }
    }

    None
}

pub async fn set_account_profile(id: &str, profile: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(_result) = collection
        .update_one(
            doc! { "_id": id },
            doc! { "$set": doc! { "profile": profile } },
            None,
        )
        .await
    {
        return true;
    }

    false
}

pub async fn get_account_profile(contact: &str) -> Option<String> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");

    if let Some(tk) = get_token_with_username(contact).await {
        if let Ok(rs) = collection
            .find_one(doc! { "_id": tk[0].as_str() }, None)
            .await
        {
            if let Some(user) = rs {
                return Some(user.profile);
            }
        }
    }

    None
}

pub async fn check_if_account_in_contacts(id: &str, contact: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");
    if let Ok(rs) = collection.find_one(doc! { "_id": id}, None).await {
        if let Some(user) = rs {
            return user.contacts.contains(&contact.to_string());
        }
    }

    false
}

pub async fn contact_add_or_remove(action: &str, from: &str, to: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<Account> = db.users.collection("permanent");

    if let Some(tk) = get_token_with_username(to).await {
        if check_if_account_in_contacts(&tk[0], from).await != true && action == "add" {
            let result: UpdateResult = collection
                .update_one(
                    doc! { "username": to},
                    doc! { "$push": doc! { "contacts": doc!{"username": from} } },
                    None,
                )
                .await
                .unwrap();

            return result.modified_count > 0;
        } else if check_if_account_in_contacts(&tk[0], from).await != false && action == "remove" {
            let result: UpdateResult = collection
                .update_one(
                    doc! { "username": to},
                    doc! { "$pull": doc! { "contacts": doc!{"username": from} } },
                    None,
                )
                .await
                .unwrap();

            return result.modified_count > 0;
        }

        return false;
    }

    false
}

pub async fn delete_action_message(username: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<ActionBase> = db.messages.collection("actions");

    if let Some(tk) = get_token_with_username(username).await {
        if let Ok(rs) = collection
            .delete_one(doc! { "_id": tk[0].as_str()}, None)
            .await
        {
            return rs.deleted_count > 0;
        }
    }

    false
}

pub async fn get_action_message(username: &str) -> Option<Vec<ActionMessage>> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<ActionBase> = db.messages.collection("actions");

    if let Some(tk) = get_token_with_username(username).await {
        if let Ok(rs) = collection.find_one(doc! { "_id": &tk[0]}, None).await {
            if let Some(action) = rs {
                return Some(action.actions);
            }
        }
    }

    None
}

pub async fn add_action_message(to: &str, action: ActionMessage) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<ActionBase> = db.messages.collection("actions");

    if let Some(tk) = get_token_with_username(to).await {
        if let Some(_fpa) = get_action_message(to).await {
            if let Ok(rs) = collection
                .update_one(
                    doc! {"_id" : &tk[0]},
                    doc! {"$push": doc! {"actions": doc! {

                        "title" : action.title.clone(),
                        "from" : action.from.clone(),
                        "message_id" : action.message_id.clone(),
                        "type" : action.action.clone(),
                        "date" : action.date.clone()

                    }}},
                    None,
                )
                .await
            {
                return rs.modified_count > 0;
            }
        }

        collection
            .insert_one(
                ActionBase {
                    id: tk[0].clone(),
                    actions: vec![action],
                },
                None,
            )
            .await
            .unwrap();

        return true;
    }

    false
}

/* SECUNDARY DATABASE */

pub async fn find_possible_account(username: &str, email: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let users_permanent: Collection<Account> = db.client.database("users").collection("permanent");
    let temp_users: Collection<TemporalAccount> = db.users.collection("temporal");

    if let Ok(rs_user) = users_permanent.find_one(
        doc! {"username": doc! {"$regex": username, "$options": "i"}, "email": doc! {"$regex": email, "$options": "i"}},
        None,
    )
    .await {

        if let Ok(rs_temp_user) = temp_users
        .find_one(
            doc! {"username": doc! {"$regex": username, "$options": "i"}, "email": doc! {"$regex": email, "$options": "i"}},
            None,
        )
        .await {

            return match (rs_user, rs_temp_user) {

                (None, None) => true,
                (_, _) => false

            }

        }

    }

    false
}

pub async fn post_temp_account(temp_user: TemporalAccount) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<TemporalAccount> = db.users.collection("temporal");
    if let Ok(_result) = collection.insert_one(temp_user, None).await {
        return true;
    }

    false
}

pub async fn is_valid_verification_code(verification_code: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<TemporalAccount> = db.users.collection("temporal");

    if let Ok(rs) = collection
        .find_one(doc! {"verification.code": verification_code}, None)
        .await
    {
        if let Some(tmp_usr) = rs {
            return tmp_usr.verification.code == verification_code;
        }
    }
    false
}

pub async fn get_temp_account(verification_code: &str) -> Option<TemporalAccount> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<TemporalAccount> = db.users.collection("temporal");

    if let Ok(rs) = collection
        .find_one(doc! {"verification.code": verification_code}, None)
        .await
    {
        if let Some(tmp_usr) = rs {
            if tmp_usr.verification.code == verification_code {
                return Some(tmp_usr);
            }
        }
    }

    None
}

pub async fn get_queue_history(token: &str) -> Option<Vec<Message>> {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<MessageBase> = db.messages.collection("queue");

    if let Ok(rs) = collection.find_one(doc! {"_id": token}, None).await {
        if let Some(msg) = rs {
            return Some(msg.messages);
        }
    }

    None
}

pub async fn delete_queue_history(username: &str) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<MessageBase> = db.messages.collection("queue");

    if let Some(tk) = get_token_with_username(username).await {
        if let Ok(rs) = collection
            .delete_one(doc! {"_id": tk[0].as_str()}, None)
            .await
        {
            return rs.deleted_count > 0;
        }
    }

    false
}

pub async fn add_message_queue_history(to: &str, message: Message) -> bool {
    let db: &DB = unsafe { MAIN_DB.as_ref().unwrap() };
    let collection: Collection<MessageBase> = db.messages.collection("queue");

    if let Some(tk) = get_token_with_username(to).await {
        if let Some(_fph) = get_queue_history(tk[0].as_str()).await {
            if let Ok(rs) = collection
                .update_one(
                    doc! {"_id" : tk[0].as_str()},
                    doc! {"$push": {"messages": doc! {

                        "id": message.id,
                        "type": message.type_,
                        "from": message.from,
                        "contain": message.contain,

                    }}},
                    None,
                )
                .await
            {
                return rs.modified_count > 0;
            }
        } else if let Ok(_rs) = collection
            .insert_one(
                MessageBase {
                    id: tk[0].clone(),
                    username: to.to_string(),
                    messages: vec![message],
                    date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                },
                None,
            )
            .await
        {
            return true;
        }
    }

    false
}

/* CLEANERS OF THE DATABASE */

pub async fn clear_temporal_accounts() {
    if let Ok(db) = Client::with_uri_str(dotenv::var("DB").unwrap()).await {
        let collection: Collection<TemporalAccount> = db.database("users").collection("temporal");

        loop {
            if collection.count_documents(doc! {}, None).await.unwrap() == 0 {
                continue;
            }

            if let Ok(mut rs) = collection.find(doc! {}, None).await {
                while let Ok(temp_user) = rs.try_next().await {
                    if let Some(user) = temp_user {
                        if NaiveDateTime::parse_from_str(
                            user.verification.remaining.as_str(),
                            "%Y-%m-%d %H:%M:%S",
                        )
                        .unwrap()
                        .and_utc()
                            <= NaiveDateTime::parse_from_str(
                                Local::now()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string()
                                    .as_str(),
                                "%Y-%m-%d %H:%M:%S",
                            )
                            .unwrap()
                            .and_utc()
                        {
                            let _ = collection
                                .delete_one(doc! {"_id": user.id.as_str()}, None)
                                .await;
                            continue;
                        }
                    }
                }
            }

            let _ = sleep(Duration::from_secs(60));
        }
    }

    panic!("Failed to connect to MongoDB.");
}

pub async fn clear_queue_messages() {
    if let Ok(db) = Client::with_uri_str(dotenv::var("DB").unwrap()).await {
        let collection: Collection<TemporalAccount> = db.database("messages").collection("queue");

        loop {
            let _ = sleep(Duration::from_secs(60 * 60 * 25));

            if collection.count_documents(doc! {}, None).await.unwrap() == 0 {
                continue;
            } else if Local::now().day() == 1 && Local::now().month() % 2 == 0 {
                let _ = collection.delete_many(doc! {}, None).await;
                continue;
            }
        }
    }

    panic!("Failed to connect to MongoDB.");
}
