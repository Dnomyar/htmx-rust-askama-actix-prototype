# Blog prototype with Rust + Askama + htmx
This is a prototype to try out htmx with Rust. The principle of htmx is to quit writing javascript by:
- having the backend returning the html templates 
- using `hx-*` tag for html componant to specify which http route to hit and with which trigger.

For the backend, I have been using Rust with:
- Askama: a template engine with compiled templates which works very well
- actix for the http server
- actix-session and actix identity for authentication
- In memory repositories