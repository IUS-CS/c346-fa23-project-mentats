# Project Reflections
## Date: 2023-11-10

Zachary Nelson
John Brown
Tyler Popson
Mason Napper

## Backend Developer, QA, Documentation
### Zachary Nelson

Throughout this project I worked on the backend, as well as testing and documentation. Initially we were using Java
with Spring boot for our backend, and we soon learned that different frameworks have their benefits and limitations, 
leading us to switch frameworks. Another important lesson from this was to consider additional tools even if they are
outside an area of expertise, such as Rust and Actix Web. 

Throughout project development, ensuring testing was an important part in deciding if a sprint's goals had been 
achieved. While using Spring boot and a remote database, it was difficult to test if SQL requests were going through due 
to framework issues; costing extra time and additional sprints to resolve the issue. One thing I would change is 
adopting a different framework earlier. Reliably testing features became an important part of our development. 

Testing naturally leads to documentation. Documentation became an important resource, even in our group of four. 
Creating detailed diagrams of software architecture ensured there was a common vision in sprint planning.  
Documentation of testing was needed to ensure reliability and compatibility of frameworks or libraries.
As the software is continuously evolving, I would expect the documentation should as well.

## Backend, Database, Testing
### John Brown

The initial planning of this project was ambitious: we had goals, features, and the destination perfectly in mind, but most of us (and that includes me) were not experienced in web development. That being said, I think the entire team gives thanks to Tyler for being knowledgeable enough to guide us towards what to work on in the project. 

In terms of my own experience:
When it came to the backend, SpringBoot seemed like a good idea as it was self-described as "production ready with minimal boilerplate", but ended up being too opaque in terms of what was actually going on as a backend. At the beginning of the semester, I didn't realize what the actual functionality of SpringBoot was in the context of a CRUD/RESTful API. There were lots of tutorials on how to get the very basics ready, but anything beyond that seemed too complex, especially authentication.

The switch to a Rust Actix-Web backend was needed because it was transparent and simple. It seems complex on the surface and takes a while to learn but at least now i know what all (okay, most) of the individual moving parts do. Funnily enough, there is a lot of boilerplate with Actix-Web! As a beginner, I'm probably not utilizing all the Actix features the right way, but I have realized that I can spend a certain amount of time saving myself from more needless work in the future. Writing the boilerplate has really helped me understand what is going on in the backend. The different layers (routes <-> persistence <-> queries) seemed over complicated when I started but I get it now because with good organization in the different layers, it's easier to see what needs to happen in each layer to fully exploit an endpoint. I was even tempted at certain points to put another layer in between routes and persistence to handle authentication (called verification), but I'm still figuring that out.

The MySQL database has been a good choice, it's been simple to configure (on both SpringBoot and Actix-Web) and even more simple to examine with a tool like MySQL Workbench. One of the biggest problems I've faced is writing queries in the backend code that satisfy the requirements of the mysql crate (Rust package). Using Postman to send http requests over local host to my dev backend server has also been a great help, although I'm not utilizing it fully with headers and parameters for the request. 

In conclusion, this project has been a really great learning experience. The pressure of learning a completely unfamiliar system that incorporates servers, builds, and tools has made me better at choosing whats important. 




