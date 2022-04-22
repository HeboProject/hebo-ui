// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{html, Component, Context, Html};

pub mod bag;
pub mod benchmark;
pub mod subscription;

pub enum ConnectionsMsg {}

pub struct ConnectionsComponent {}

impl Component for ConnectionsComponent {
    type Message = ConnectionsMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Connections"}</h1>
            </div>
        }
    }
}
