
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
}

// Implementation of the State struct
impl State {
    // Method for creating a new State
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Initialize the state
    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        };
    }

    // Method to get a mutable reference to the state
    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Method to get a static reference to the state
    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

// Struct for reading the state with required derive macros
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub admins: Vec<ActorId>,
    pub mvps: Vec<MVP>,
}

// Implementation for converting State to IoState
impl From<State> for IoState {
    fn from(value: State) -> Self {
        let State { admins, mvps } = value;
        let mvps = mvps.into_iter().map(|(_, v)| v).collect();
        Self { admins, mvps }
    }
}

// Service struct
#[derive(Default)]
pub struct Service;

// Implementation for the service
impl Service {
    // Method for initializing the service
    pub fn seed() {
        State::init_state();
    }
}

// Service definition with methods
#[service]
impl Service {
    // New service instance constructor
    pub fn new() -> Self {
        Self
    }

    // Method to add an MVP
    pub fn add_mvp(&mut self, mvp: MVP) -> Result<Events, Errors> {
        let state = State::state_mut();

        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        state.mvps.insert(mvp.id, mvp);

        Ok(Events::MVPCreated)
    }

    // Method to vote for an MVP
    pub fn vote_for_mvp(&mut self, mvp_id: u32) -> Result<Events, Errors> {
        let state = State::state_mut();

        let mvp = state.mvps.get_mut(&mvp_id).ok_or(Errors::MVPNotFound)?;

        mvp.positive_votes += 1;

        Ok(Events::VoteCasted)
    }

    // Method to remove an MVP by an admin
    pub fn remove_mvp(&mut self, mvp_id: u32) -> Result<Events, Errors> {
        let state = State::state_mut();

        if !state.admins.contains(&msg::source()) {
            return Err(Errors::Unauthorized);
        }

        state.mvps.remove(&mvp_id).ok_or(Errors::MVPNotFound)?;

        Ok(Events::MVPRemoved)
    }

    // Query for the list of all MVPs
    pub fn mvps_list(&self) -> IoState {
        State::state_ref().to_owned().into()
    }

    // Query for MVPs by actor_id
    pub fn mvps_by_actor(&self, actor_id: ActorId) -> Vec<MVP> {
        State::state_ref()
            .mvps
            .values()
            .filter(|mvp| mvp.actor_id == actor_id)
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
}

// Errors for handling different failure cases
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Errors {
    Unauthorized,
    MVPNotFound,
}
