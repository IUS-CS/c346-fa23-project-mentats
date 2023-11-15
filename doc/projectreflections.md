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

## Frontend, Backend, Design
### Tyler Popson

The goals of this project were a bit ambitious for the constraints of this one semester class especially with the rest of the team not having done any web development before, although I still think that this was a good project for everyone to learn new skills and get some experience working with a team.

The tech stack that we used was mainly of my choosing since I had the most experience with what we were doing, although I wasn't a huge fan of the choice of springboot to start with but thought that it would be ok since everyone in the group had at least some experience with Java but it became a big issue with lack of clear documentation of how to do even basic tasks in it so I think the switch away from it was a good choice. That being said the choice of actix was probably not the best since it being a rust framework required the team to get used to a new way of doing things(although still less opaque than springboot) and something like expressJS would have been a better choice since it has so much more of a comunity around it outside of official docs but it's not the end of the world. The frontend on the other hand has been solid and I have no regrets choosing Vue with Typescript as it is just a good dx overall and has worked just fine, and the same goes for mySQL for the database it really just works and hasn't gotten in the way at all. 

Personally this project has been a good learning experience for me in a more managerial role since I usually in the past done most of my work solo or have not been in charge of a team so I have either just had tools and architecture already chosen or I was just choosing what I liked and just changing out what I didn't like at will. neither of which really happened here as I had the most experience and I couldn't just make major changes consequence free. I also learned a good bit about the non coding parts such as CI/CD processes and things such as the handling of servers and hosting(still trying to get the stupid reverse proxy working right), and have a new appreciation to the services of backend as a service providers.

overall I think that while this project might have been a bit big for the circumstances, it was a good learning experience and I will continue to work on it going forward as it is a service that I know some people want to use and will be a great portfolio project, And I hope that this served as a good introduction for the group to the rabit hole that is web development.

## Frontend, Design
### Mason Napper

I was very lost going into this project. I made that clear to the rest of my group that I had little experience in actual coding, just the theory behind all of it. However, I still strove to learn more about frontend coding and design. It interested me, and I am glad that that is what I chose to work on in this project.

The project was definitely challenging for me. I had never written a script before, let alone dealt with the tedium of designing a simple webpage. Tyler was very experienced with this and helped me along every step of the way. The backend side of things still confuses me a little bit to be honest, but I did not have enough time to look too much into it as I was focused on the backend and designing it. I did enjoy designing webpages, as annoying and cumbersome as it can get, and Tyler was there every step of the way helping me with coding that I did not understand yet.

Personally, I have learned so much just from this one project. It has taught me the basics of web-based languages and how many different frameworks and designing CSS's there are. I am glad that I chose the role of working in the frontend, and I feel that I have learned so much in this one semester of this class. I rather enjoy designing webpages and making them my own creation, and I definitely enjoyed doing it as a team so that I had some advisement and decisions that could be made together.

Overall, this project was a great learning experience for me. It put me in the real world instead of in the classroom. It gave me opportunities and challenges that can only be had out in the workforce. It helped me learn what I enjoy, what I lacked in, and what I was actually good at. This project assisted me in the learning of my craft, and I cannot wait to do more writing of code in the future.