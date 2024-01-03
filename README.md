# Blog prototype with Rust + Askama + htmx
This is a prototype to try out htmx with Rust. The principle of htmx is to describe the application state as html (restful). Htmx allows to specify what parts of the page to request from the server by using html attributes.

For instance, the following `<div hx-get="/posts" hx-trigger="load"></div>` will trigger a http get request on page load and replace the div with the content of the response. That's mostly what htmx is about.

| What was I trying out                | What technology did I use               | What went well                                                                                                                                                                                                                                                                                                                     | What did not go well                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------ | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Dynamic frontend / react alternative | htmx (dynamic pages without writing JS) | It took a bit of time to understand the concept but once i got used to the hx-* attributes, it went pretty well. The event system seems really powerful. It is possible to trigger an event when a http response is received. For instance, when the login response is received, an event is emited that reloads parts of the page. | I did not quite figure out the testing part yet. It sounds like the best approach is to do classical ui testing with a headless browser but that's very cumbersome. I found that I was forgetting specify the hx-target (where to put the content of the http reponse) and ended up with a blog post within the button for instance. I am still not too sure how to test that without browser testing. |
| Templating in Rust                   | Askama                                  | Templates are checked at compile time (compilation will fail if the variable in the templates don't exist)                                                                                                                                                                                                                         | No particular struggle                                                                                                                                                                                                                                                                                                                                                                                 |
| Web server in Rust                   | Actix (session, identity)               | Actix ecosystem has multiple small libraries that work together for authentication for instance. It was quite easy to setup.                                                                                                                                                                                                       | No particular struggle                                                                                                                                                                                                                                                                                                                                                                                 |
| UI browser testing                   | playwright                              | It was able to setup the test fairly easily.                                                                                                                                                                                                                                                                                       | Sometimes the test fails for no reason. UI testing with a headless browser seem always seems brittle. It is very heavy and it looks hard to acheive any TDD with this technology.                                                                                                                                                                                                                      |

htmx seems like a great technology and was very pleasant to work with. However, the approach to testing is still unclear to me.
