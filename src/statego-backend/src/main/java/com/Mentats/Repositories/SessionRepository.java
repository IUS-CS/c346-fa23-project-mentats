package com.Mentats.Repositories;

import org.springframework.data.repository.CrudRepository;

import com.Mentats.Entities.Session;

// This will be AUTO IMPLEMENTED by Spring into a Bean called userRepository
// CRUD refers Create, Read, Update, Delete

// <User, Integer> is detailing the ID autogenerated in the Session entity.
// Apparently this is all the code that is need, but it can also be extended
// with custom implementation of queries.
public interface SessionRepository extends CrudRepository<Session, Integer> {

}