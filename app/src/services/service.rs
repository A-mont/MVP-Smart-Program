
use sails_rs::{
    prelude::*,
    gstd::msg,
    collections::HashMap,
};

// Static variable for the contract's state
pub static mut STATE: Option<State> = None;

// Definition of the state with assigned macros
#[derive(Clone, Default)]
pub struct State {
    pub admins: Vec<ActorId>,
    pub mvps: HashMap<u32, MVP>,
}

// Struct for MVP with required derive macros
#[derive(Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct MVP {
    pub id: u32,
    pub actor_id: ActorId,
    pub project_name: String,
    pub description: String,
    pub logo: String,
    pub repository: String,
    pub video_demo: String,
    pub positive_votes: u32,
    pub voter_wallets: Vec<ActorId>
}

// Implementation of the State struct
impl State {
    // Method for creating a new State
    pub fn new(initial_admin: ActorId) -> Self {
        Self {
            admins: vec![initial_admin],
            ..Default::default()
        }
    }

    // Initialize the state
    pub fn init_state(initial_admin: ActorId) {
        unsafe {
            STATE = Some(Self::new(initial_admin));
        };
    }

  
    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }


    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub admins: Vec<ActorId>,
    pub mvps: Vec<MVP>,
}


impl From<State> for IoState {
    fn from(value: State) -> Self {
        let State { admins, mvps } = value;
        let mvps = mvps.into_iter().map(|(_, v)| v).collect();
        Self { admins, mvps }
    }
}


#[derive(Default)]
pub struct Service;


impl Service {
    
    pub fn seed(admin: ActorId) {
        State::init_state(admin);
    }
}


#[service]
impl Service {

       
    pub fn new() -> Self {
            Self
        }


    pub fn add_admin(&mut self, new_admin: ActorId) -> Result<Events, Errors> {
        let state = State::state_mut();

        
        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        if !state.admins.contains(&new_admin) {
            state.admins.push(new_admin);
        }

        Ok(Events::AdminAdded)
    }

    pub fn remove_admin(&mut self, admin_to_remove: ActorId) -> Result<Events, Errors> {
        let state = State::state_mut();

        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        state.admins.retain(|admin| *admin != admin_to_remove);

        Ok(Events::AdminRemoved)
    }

    pub fn add_mvp(&mut self, mvp: MVP) -> Result<Events, Errors> {
        let state = State::state_mut();

        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        state.mvps.insert(mvp.id, mvp);

        Ok(Events::MVPCreated)
    }

    pub fn remove_mvp(&mut self, mvp_id: u32) -> Result<Events, Errors> {
        let state = State::state_mut();

        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        state.mvps.remove(&mvp_id).ok_or(Errors::MVPNotFound)?;

        Ok(Events::MVPRemoved)
    }

    pub fn vote_for_mvp(&mut self, mvp_id: u32) -> Result<Events, Errors> {
        let state = State::state_mut();
    
        let mvp = state.mvps.get_mut(&mvp_id).ok_or(Errors::MVPNotFound)?;
    

        let voter = msg::source();
        if mvp.voter_wallets.contains(&voter) {
            return Err(Errors::AlreadyVoted);
        }
    
        mvp.positive_votes += 1;
        mvp.voter_wallets.push(voter);
    
        Ok(Events::VoteCasted)
    }
    

    pub fn update_mvp_info(&mut self, mvp_id: u32, updated_mvp: MVP) -> Result<Events, Errors> {
        let state = State::state_mut();

        let existing = state.mvps.get_mut(&mvp_id).ok_or(Errors::MVPNotFound)?;

        if existing.owner != msg::source() {
            return Err(Errors::Unauthorized);
        }

      
        existing.project_name = updated_mvp.project_name;
        existing.description = updated_mvp.description;
        existing.logo = updated_mvp.logo;
        existing.repository = updated_mvp.repository;
        existing.video_demo = updated_mvp.video_demo;

        Ok(Events::MVPUpdated)
    }

    pub fn mvps_list(&self) -> IoState {
        State::state_ref().to_owned().into()
    }

    pub fn mvps_by_actor(&self, actor_id: ActorId) -> Vec<MVP> {
        State::state_ref()
            .mvps
            .values()
            .filter(|mvp| mvp.owner == actor_id)
            .cloned()
            .collect()
    }
}


// Events to notify users
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Events {
    MVPCreated,
    VoteCasted,
    MVPRemoved,
    AdminAdded,
    AdminRemoved,
    MVPUpdated,
}

// Errors for handling different failure cases
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Errors {
    Unauthorized,
    MVPNotFound,
}
