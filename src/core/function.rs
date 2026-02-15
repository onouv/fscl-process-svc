use actix::prelude::*;
use crate::core::ItemId;
use crate::core::sub::Error;

#[derive(Debug)]
pub struct FunctionActor {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    subs: Vec<(ItemId, Addr<FunctionActor>)>,
}

impl FunctionActor {
    pub fn start_new(id: &str, name: &str, description: &str) -> Addr<FunctionActor> {
        FunctionActor {
            id: ItemId::new(id),
            name: name.to_string(),
            description: description.to_string(),
            subs: Vec::new(),
        }
        .start()
    }
}

impl Actor for FunctionActor {
    type Context = Context<Self>;
}

/// Message to add a sub-function (provide its id and address)
#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct AddSub(pub ItemId, pub Addr<FunctionActor>);

#[derive(Message)]
#[rtype(result = "Result<Addr<FunctionActor>, Error>")]
pub struct RemoveSub(pub ItemId);

#[derive(Message)]
#[rtype(result = "Option<Addr<FunctionActor>>")]
pub struct GetSub(pub ItemId);

impl Handler<AddSub> for FunctionActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: AddSub, _ctx: &mut Self::Context) -> Self::Result {
        let AddSub(id, addr) = msg;
        if self.subs.iter().any(|(existing_id, _)| existing_id == &id) {
            return Err(Error::ItemIdAlreadyRegistered);
        }
        self.subs.push((id, addr));
        Ok(())
    }
}

impl Handler<RemoveSub> for FunctionActor {
    type Result = Result<Addr<FunctionActor>, Error>;

    fn handle(&mut self, msg: RemoveSub, _ctx: &mut Self::Context) -> Self::Result {
        let RemoveSub(id) = msg;
        if let Some(pos) = self.subs.iter().position(|(existing_id, _)| existing_id == &id) {
            let (_id, addr) = self.subs.remove(pos);
            Ok(addr)
        } else {
            Err(Error::ItemIdNotFound)
        }
    }
}

impl Handler<GetSub> for FunctionActor {
    type Result = Option<Addr<FunctionActor>>;

    fn handle(&mut self, msg: GetSub, _ctx: &mut Self::Context) -> Self::Result {
        let GetSub(id) = msg;
        self.subs.iter().find(|(existing_id, _)| existing_id == &id).map(|(_, addr)| addr.clone())
    }
}
