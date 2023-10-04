# DesignPatterns 

  
## Designs In Use

This sprint consisted of representing a user object as well as a session object. Both would be required for the implementation of account features, and tying the main features of the app together. 
The first pattern we looked at for this prototype was Singleton. Singleton is used for creating the session and user object and giving access to a controller. The user and session object being accessible is useful for testing, but may cause issues in the future.


## Designs Planned For Use

After the user and session object are completed, the use of the Singleton for these objects will come up. Other patterns for designing multiple instances may be used, such as the factory. 
Requests for objects such as logs may fall to a Chain of Responsibility. Often times a user may request to see a game previously logged, and then another. Or a group of users may want to see a previously logged game they all took part in.

## Designs For Future Use

As we are working on the database and objects for users to access, design patterns that allow easy creation and retrieval of objects would be prioritized. The complexity of creating certain objects may need an Object Pool. 
The Command Pattern design may help with requests to a database. As the needs of our code are revealed, we plan to implement different patterns for new and recent features.