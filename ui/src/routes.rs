use dioxus::prelude::*;
use crate::views::{Dashboard, Profile, Resumes, Jobs, Applications};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/profile")]
    Profile {},
    #[route("/resumes")]
    Resumes {},
    #[route("/jobs")]
    Jobs {},
    #[route("/applications")]
    Applications {},
}