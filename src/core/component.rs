use actix::prelude::*;
use crate::core::ItemId;
use crate::core::sub::Error;
use crate::core::function::FunctionActor;

#[derive(Debug)]
pub struct ComponentActor {
	pub id: ItemId,
	pub name: String,
	pub description: String,
	subs: Vec<(ItemId, Addr<ComponentActor>)>,
	implementees: Vec<ItemId>,
}

impl ComponentActor {
	pub fn start_new(id: &str, name: &str, description: &str) -> Addr<ComponentActor> {
		ComponentActor {
			id: ItemId::new(id),
			name: name.to_string(),
			description: description.to_string(),
			subs: Vec::new(),
			implementees: Vec::new(),
		}
		.start()
	}
}

impl Actor for ComponentActor {
	type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct AddComponentSub(pub ItemId, pub Addr<ComponentActor>);

#[derive(Message)]
#[rtype(result = "Result<Addr<ComponentActor>, Error>")]
pub struct RemoveComponentSub(pub ItemId);

#[derive(Message)]
#[rtype(result = "Option<Addr<ComponentActor>>")]
pub struct GetComponentSub(pub ItemId);

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct ImplementFunction(pub ItemId, pub Addr<FunctionActor>);

impl Handler<AddComponentSub> for ComponentActor {
	type Result = Result<(), Error>;

	fn handle(&mut self, msg: AddComponentSub, _ctx: &mut Self::Context) -> Self::Result {
		let AddComponentSub(id, addr) = msg;
		if self.subs.iter().any(|(existing_id, _)| existing_id == &id) {
			return Err(Error::ItemIdAlreadyRegistered);
		}
		self.subs.push((id, addr));
		Ok(())
	}
}

impl Handler<RemoveComponentSub> for ComponentActor {
	type Result = Result<Addr<ComponentActor>, Error>;

	fn handle(&mut self, msg: RemoveComponentSub, _ctx: &mut Self::Context) -> Self::Result {
		let RemoveComponentSub(id) = msg;
		if let Some(pos) = self.subs.iter().position(|(existing_id, _)| existing_id == &id) {
			let (_id, addr) = self.subs.remove(pos);
			Ok(addr)
		} else {
			Err(Error::ItemIdNotFound)
		}
	}
}

impl Handler<GetComponentSub> for ComponentActor {
	type Result = Option<Addr<ComponentActor>>;

	fn handle(&mut self, msg: GetComponentSub, _ctx: &mut Self::Context) -> Self::Result {
		let GetComponentSub(id) = msg;
		self.subs.iter().find(|(existing_id, _)| existing_id == &id).map(|(_, addr)| addr.clone())
	}
}

impl Handler<ImplementFunction> for ComponentActor {
	type Result = Result<(), String>;

	fn handle(&mut self, msg: ImplementFunction, _ctx: &mut Self::Context) -> Self::Result {
		let ImplementFunction(func_id, _func_addr) = msg;
		if self.implementees.contains(&func_id) {
			return Err("Function already implemented by this component".to_string());
		}
		self.implementees.push(func_id);
		Ok(())
	}
}

