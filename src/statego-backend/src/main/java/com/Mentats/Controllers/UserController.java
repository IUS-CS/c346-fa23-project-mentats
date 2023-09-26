package com.Mentats.Controllers;

import java.util.ArrayList;
import java.util.List;
import java.util.Date;

//This is HTTP, not HTTPS, will be implemented/changed later

//dependency injection stuff, "injects dependency into bean"
import org.springframework.beans.factory.annotation.Autowired;
//according to documentation, "Contains a basic abstraction over client/server-side HTTP. This package contains the
// HttpInputMessage and HttpOutputMessage interfaces." In this case, "Enumeration of HTTP status codes." and
//"Extension of HttpEntity that adds an HttpStatusCode status code.", where "This can also be used in Spring MVC as the
// return value from an @Controller method: "
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
//according to documentation "Annotation for permitting cross-origin requests on specific handler classes and/or handler
// methods. Processed if an appropriate HandlerMapping is configured." The optional elements handle different things,
//such as allowed credentials, headers, age, and HTTP methods.
import org.springframework.web.bind.annotation.CrossOrigin;
//following two are: "Annotation for mapping HTTP requests onto specific handler methods."
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
// described as: "Annotation which indicates that a method parameter should be bound to a URI template variable.
// Supported for RequestMapping annotated handler methods." Basically means that when finding something you can put an
//identifier in the request and find it that way.
import org.springframework.web.bind.annotation.PathVariable;
//following two are: "Annotation for mapping HTTP requests onto specific handler methods."
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
//described as: Annotation indicating a method parameter should be bound to the body of the web request. The body of the
// request is passed through an HttpMessageConverter to resolve the method argument depending on the content type of the
// request. Optionally, automatic validation can be applied by annotating the argument with @Valid.
import org.springframework.web.bind.annotation.RequestBody;
//really vague and important annotation, described as: Annotation for mapping web requests onto specific handler classes
// and/or handler methods. Provides consistent style between Servlet and Portlet environments, with the semantics
// adapting to the concrete environment.
//See: https://docs.spring.io/spring-framework/docs/3.0.x/javadoc-api/org/springframework/web/bind/annotation/RequestMapping.html
import org.springframework.web.bind.annotation.RequestMapping;
//Described as: "Annotation which indicates that a method parameter should be bound to a web request parameter. "
import org.springframework.web.bind.annotation.RequestParam;
//An annotation which tells Spring that the class is a controller, described as: A convenience annotation that is itself
// annotated with @Controller and @ResponseBody. Types that carry this annotation are treated as controllers where
// @RequestMapping methods assume @ResponseBody semantics by default.
import org.springframework.web.bind.annotation.RestController;

import com.Mentats.Entities.User;
import com.Mentats.Repositories.UserRepository;

//The Cross Origin comes from CORS (or Cross Origin Resource Sharing). The origin for a controller doesn't just have to
//be from a specific place, it can be from others too. See: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS and
//https://spring.io/guides/gs/rest-service-cors/#global-cors-configuration
@CrossOrigin(origins = "PLACEHOLDER") //THIS SHOULD BE COMPLETED LATER (coming in from frontend)
@RestController
@RequestMapping("/PLACEHOLDER") //THIS SHOULD BE COMPLETED LATER
public class UserController {

    @Autowired
    UserRepository UserRepository;

    //The code for the getAllUsers method is not intended to work right away, but to show the structure
    //of what the code should look like
    //Get Requests are used to request information from a specfied resource.
    @GetMapping("/PLACEHOLDER")
    public ResponseEntity<List<User>> getAllUsers(@RequestParam(required = false) String userName) {
        try {
            List<User> userList = new ArrayList<User>();

            //The returned content may not make sense when considering "users"
            if (userName == null)
                //double colon is also called the method reference operator
                UserRepository.findAll().forEach(userList::add);
            else
                UserRepository.findByUserContaining(userName).forEach(userList::add);

            if (userList.isEmpty()) {
                return new ResponseEntity<>(HttpStatus.NO_CONTENT);
            }

            return new ResponseEntity<>(userList, HttpStatus.OK);
        } catch (Exception e) {
            return new ResponseEntity<>(null, HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @GetMapping("/tutorials/{id}")
    public ResponseEntity<Tutorial> getTutorialById(@PathVariable("id") long id) {
        Optional<Tutorial> tutorialData = tutorialRepository.findById(id);

        if (tutorialData.isPresent()) {
            return new ResponseEntity<>(tutorialData.get(), HttpStatus.OK);
        } else {
            return new ResponseEntity<>(HttpStatus.NOT_FOUND);
        }
    }

    @PostMapping("/tutorials")
    public ResponseEntity<Tutorial> createTutorial(@RequestBody Tutorial tutorial) {
        try {
            Tutorial _tutorial = tutorialRepository
                    .save(new Tutorial(tutorial.getTitle(), tutorial.getDescription(), false));
            return new ResponseEntity<>(_tutorial, HttpStatus.CREATED);
        } catch (Exception e) {
            return new ResponseEntity<>(null, HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @PutMapping("/tutorials/{id}")
    public ResponseEntity<Tutorial> updateTutorial(@PathVariable("id") long id, @RequestBody Tutorial tutorial) {
        Optional<Tutorial> tutorialData = tutorialRepository.findById(id);

        if (tutorialData.isPresent()) {
            Tutorial _tutorial = tutorialData.get();
            _tutorial.setTitle(tutorial.getTitle());
            _tutorial.setDescription(tutorial.getDescription());
            _tutorial.setPublished(tutorial.isPublished());
            return new ResponseEntity<>(tutorialRepository.save(_tutorial), HttpStatus.OK);
        } else {
            return new ResponseEntity<>(HttpStatus.NOT_FOUND);
        }
    }

    @DeleteMapping("/tutorials/{id}")
    public ResponseEntity<HttpStatus> deleteTutorial(@PathVariable("id") long id) {
        try {
            tutorialRepository.deleteById(id);
            return new ResponseEntity<>(HttpStatus.NO_CONTENT);
        } catch (Exception e) {
            return new ResponseEntity<>(HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

    @DeleteMapping("/tutorials")
    public ResponseEntity<HttpStatus> deleteAllTutorials() {
        try {
            tutorialRepository.deleteAll();
            return new ResponseEntity<>(HttpStatus.NO_CONTENT);
        } catch (Exception e) {
            return new ResponseEntity<>(HttpStatus.INTERNAL_SERVER_ERROR);
        }

    }

    @GetMapping("/tutorials/published")
    public ResponseEntity<List<Tutorial>> findByPublished() {
        try {
            List<Tutorial> tutorials = tutorialRepository.findByPublished(true);

            if (tutorials.isEmpty()) {
                return new ResponseEntity<>(HttpStatus.NO_CONTENT);
            }
            return new ResponseEntity<>(tutorials, HttpStatus.OK);
        } catch (Exception e) {
            return new ResponseEntity<>(HttpStatus.INTERNAL_SERVER_ERROR);
        }
    }

}
