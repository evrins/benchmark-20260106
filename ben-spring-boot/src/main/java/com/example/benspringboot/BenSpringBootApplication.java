package com.example.benspringboot;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@SpringBootApplication
public class BenSpringBootApplication {

    public static void main(String[] args) {
        SpringApplication.run(BenSpringBootApplication.class, args);
    }

    @GetMapping("/")
    public Response index() {
        return new Response();
    }
}
