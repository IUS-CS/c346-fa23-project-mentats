package com.mentats.stategobackend;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@SpringBootApplication
@RestController
public class StategoBackendApplication {

	public static void main(String[] args) {
		SpringApplication.run(StategoBackendApplication.class, args);
	}

	@GetMapping("/hello")
    public String hello() {
      return "hello World";
    }

	@GetMapping("/getUsers")
	public String getUsers() {
		return "hi there";
	}

}
