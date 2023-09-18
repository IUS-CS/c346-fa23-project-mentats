package com.example.accessingdatamysql;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;


@Entity // This tells Hibernate to make a table out of this class
public class Session {
  //Unique ID is automatically generated with the following annotations
  @Id
  @GeneratedValue(strategy=GenerationType.AUTO)
  private Integer id;

  //this variable is a placeholder, name will be associated with a superclass in following implementation
  private String name;

  //will store session time in increments of 30 minutes
  private Integer sessionTime;

  //denoting whether the session indicates the end of a campaign for a game title
  private Boolean yesnoComplete;

  //a string list passed in containing the names of players, which may differ from session to session
  //list may also be used in calculations that involve the number of players
  private List<String> namesList;

  // potentially a larger string than most strings, but capable of containing the "notes" for a session
  private String notes;

  //this variable will be discussed later, or may not be implemented at all, but will contain a link to an image for a session
  private String imageLink;


  //getter setter methods for all variables
  public Integer getId() {
    return id;
  }
  public void setId(Integer Id) {
    this.id = Id;
  }

  public String getName() {
    return name;
  }
  public void setName(String Name) {
    this.name = Name;
  }

  public Integer getSessionTime() {
    return sessionTime;
  }
  public void setSessionTime(Integer SessionTime) {
    this.sessionTime = SessionTime;
  }

  public Boolean getYesNoComplete() {
    return yesnoComplete;
  }
  public void setYesNoComplete(Boolean YesNoComplete) {
    this.yesnoComplete = YesNoComplete;
  }

  public List<String> getNamesList() {
    return namesList;
  }
  public void setNamesList(List<String> NamesList) {
    this.namesList = NamesList;
  }

  public String getNotes() {
    return notes;
  }
  public void setNotes(String Notes) {
    this.notes = Notes;
  }

  public String getImageLink() {
    return imageLink;
  }
  public void setImageLink(String ImageLink) {
    this.imageLink = ImageLink;
  }





  
}