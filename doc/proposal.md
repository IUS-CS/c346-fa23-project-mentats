# Statego Proposal

For any board game player who wish to track their playing statistics in a centralized location. Statego is a web app that provides statistics tracking and data analysis. Unlike Board Games Geek our product focuses solely on keeping track of the games you play, not trying to sell you new ones. this allows Statego to offer a cleaner simpler interface that gets right to the data you want and nothing you don't.

## Team: The Mentats
| Name           | Roles                         |
| -------------- | ----------------------------- |
| Tyler Popson   | Frontend, Design, Integration |
| Mason Napper   | Frontend                      |
| Zachary Nelson | Backend, Database             |
| John Brown     | Backend, Database             |

## Goals of Statego

The Goal of Statego is to offer an easy way for users to log their board game activity and recieve analysis on things such as win rates, who was playing, average playtime, etc.

### Planned Features
 - User Accounts
    - Users should be able to create an account and log into that account 
    - A private user dashboard that offers a general overview of all games they play
    - A page for each game that the user playes that has game specific info, and all sessions of that game played
    - A form to log a new session of a game
    - Availability calender
    - Public profile page
 - Admin
    - An admin dashboard with controls over user accounts
 - Friends List
    - Add other users to a friends list
    - Link friends to sessions they played in
 - Game
    - Name of Game
    - Description
    - Avg. playtime
    - Total playtime
    - Sessions of game
    - Win/Loss % of players
    - Player notes
 - Session
    - Game played
    - Date/Time
    - Players
    - Finished yes/no
    - winner/s
    - picure of game board

## Tech Stack

### Frontend
 - VueJS
 - NPM
 - Typescript
 - TailwindCSS
 - Vitest
 - Eslint
 - Vercel

 The frontend of Statego will be a VueJS single page application where all the dependencies will be handled by npm. It will use Typescript as the language for ease of development, TailwindCSS for styling, Eslint and Prettier for code quality and formatting. Testing is being handled by Vitest, and we are deploying on Vercel which is being served to [statego.app](https://statego.app).

### Backend
 - Spring Boot
 - Java
 - Gradle
 - Linode

The backend is a Spring Boot app which is a backend framework for Java, to build and test the app we are using Gradle, and it will be deployed to a Linux server on Linode.

### Database
 - MySQL
 - PlanetScale

 Teh Database will use MySQl and is hosted on Planetscale.
