package com.Mentats.Repositories;

//These imports are for custom method implementation.
import java.util.List;
import java.util.ArrayList;
import java.util.Date;

//From Spring Documentation on Repository Interface. Jpa repository is a specific extension of Repository.
// Central repository marker interface. Captures the domain type to manage as well as the domain type's id type. General
// purpose is to hold type information as well as being able to discover interfaces that extend this one during
// classpath scanning for easy Spring bean creation.
//Domain repositories extending this interface can selectively expose CRUD methods by simply declaring methods of the
// same signature as those declared in CrudRepository.
import org.springframework.data.jpa.repository.JpaRepository;
//list of methods inherited from other repository and CRUD repository methods can be found here:
//https://docs.spring.io/spring-data/jpa/docs/current/api/org/springframework/data/jpa/repository/JpaRepository.html
import com.Mentats.Entities.User;

public interface UserRepository extends JpaRepository<User, Integer> {
    //custom methods and queries can be implemented here
}
