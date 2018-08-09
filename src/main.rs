#![feature(rust_2018_preview, use_extern_macros, drain_filter)]
#![warn(rust_2018_idioms)]

use colored::*;
use failure::Error;
use futures::{sync::oneshot, Future};
use grpcio::{Environment, RpcContext, RpcStatus, RpcStatusCode, ServerBuilder, UnarySink};
use lazy_static::lazy_static;
use log::{error, info, log};
use protobuf::RepeatedField;
use protos::main::{
    Contact, ContactID, ContactsList, ProccessResult, ProccessStatus, ResultMessage, Void,
};
use protos::main_grpc::{self, Contacts};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::{io, thread, time};

type Result<T> = std::result::Result<T, Error>;
type ArcVec<T> = Arc<Vec<T>>;

lazy_static! {
    static ref DB: Mutex<ArcVec<Contact>> = Mutex::new(Arc::new(Vec::new()));
    // TODO: Add rate limiting
}
#[derive(Clone)]
struct ContactsService;

impl Contacts for ContactsService {
    fn get_all_contacts(&self, ctx: RpcContext<'_>, req: Void, res: UnarySink<ContactsList>) {
        let db = DB.lock().unwrap();
        let mut contacts_list = ContactsList::new();
        let contacts = RepeatedField::from_vec(db.to_vec());
        contacts_list.set_contacts(contacts);
        let f = res
            .success(contacts_list)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn add_contact(&self, ctx: RpcContext<'_>, req: Contact, res: UnarySink<ProccessResult>) {
        let mut proccess_result = ProccessResult::new();
        let mut result_msg = ResultMessage::new();
        result_msg.set_text("Contact Added".to_string());
        proccess_result.set_message(result_msg);
        proccess_result.set_status(ProccessStatus::OK);

        let random_id: i32 = rand::random();
        let mut contact_id = ContactID::new();
        contact_id.set_value(random_id);
        let mut contact = req.clone();
        contact.set_id(contact_id);

        let mut db = DB.lock().unwrap();
        Arc::make_mut(&mut db).push(contact);

        info!("{} {}", "Added New Contact with Id: ".green(), random_id);
        let f = res
            .success(proccess_result)
            .map_err(move |e| error!("failed to reply {:?}", e));
        ctx.spawn(f)
    }

    fn edit_contact(&self, ctx: RpcContext<'_>, req: Contact, res: UnarySink<Contact>) {
        let mut db = DB.lock().unwrap();
        let db = Arc::make_mut(&mut db);
        if let Some(contact) = db.into_iter().find(|c| c.get_id() == req.get_id()) {
            if req.get_name() != "" {
                contact.set_name(req.get_name().to_string());
            }
            if req.has_phone_number() {
                contact.set_phone_number(req.get_phone_number().to_owned());
            }
            let f = res
                .success(req)
                .map_err(move |e| error!("failed to reply {:?}", e));
            ctx.spawn(f)
        } else {
            let rpc_res = RpcStatus::new(
                RpcStatusCode::NotFound,
                Some("Contact Not Found".to_string()),
            );
            let f = res
                .fail(rpc_res)
                .map_err(move |e| error!("failed to reply {:?}", e));
            ctx.spawn(f)
        }
    }

    fn delete_contact(&self, ctx: RpcContext<'_>, req: ContactID, res: UnarySink<ProccessResult>) {
        let mut db = DB.lock().unwrap();
        let db = Arc::make_mut(&mut db);
        let contacts_deleted: Vec<Contact> = db
            .drain_filter(|contact| contact.get_id().get_value() == req.get_value())
            .collect();

        if contacts_deleted.is_empty() {
            let rpc_res = RpcStatus::new(
                RpcStatusCode::NotFound,
                Some("Contact Not Found".to_string()),
            );
            let f = res
                .fail(rpc_res)
                .map_err(move |e| error!("failed to reply {:?}", e));
            ctx.spawn(f)
        } else {
            let mut proccess_result = ProccessResult::new();
            let mut result_msg = ResultMessage::new();
            result_msg.set_text("Contact Deleted".to_string());
            proccess_result.set_message(result_msg);
            proccess_result.set_status(ProccessStatus::OK);
            info!(
                "{} {}",
                "Deleted Contact with Id: ".green(),
                req.get_value()
            );
            let f = res
                .success(proccess_result)
                .map_err(move |e| error!("failed to reply {:?}", e));
            ctx.spawn(f)
        }
    }
}

fn main() -> Result<()> {
    std::env::set_var("KK_LOGS", "kontakt_server");
    let mut builder = env_logger::Builder::from_env("KK_LOGS");
    builder.format(|buf, record| writeln!(buf, " {} -- {}", record.level(), record.args()));
    builder.init();
    info!("{}", "Starting Server..".green());

    let env = Arc::new(Environment::new(2));
    let contacts_service = main_grpc::create_contacts(ContactsService);

    let mut server = ServerBuilder::new(env)
        .register_service(contacts_service)
        .bind("0.0.0.0", get_server_port())
        .build()?;

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!("{} {}:{}", "Listening on".blue(), host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        if cfg!(target_env = "musl") {
            let ten_millis = time::Duration::from_millis(1000);
            loop {
                thread::sleep(ten_millis);
            }
            // tx.send(())
        } else {
            info!("{}", "Press ENTER to exit...".blue());
            let _ = io::stdin().read(&mut [0u8]).unwrap();
            tx.send(())
        }
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
    Ok(())
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}
