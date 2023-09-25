package com.Mentats;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import java.util.Date;


@Entity // This tells Hibernate to make a table out of this class
public class User {
    //Unique ID is automatically generated with the following annotations
    @Id
    @GeneratedValue(strategy=GenerationType.AUTO)
    private Integer id;

    //contains the userName of the User
    private String userName;

    //contains the associated email of the User
    private String email;

    //contains the private password of the User
    private String password;

    //denotes whether the User is "enabled" (will be expanded later)
    private Boolean enabledStatus;

    //contains information on the last date that the user logged in
    private Date lastLogin;



    //getter setter methods for all variables
    public Integer getId() {
        return id;
    }
    public void setId(Integer Id) {
        this.id = Id;
    }

    public String getUserName() {
        return userName;
    }
    public void setUserName(String Username) {
        this.userName = Username;
    }

    public String getEmail() {
        return email;
    }
    public void setEmail(String Email) {
        this.email = Email;
    }

    public String getPassword() {
        return password;
    }
    public void setPassword(String Password) {
        this.password = Password;
    }

    public Boolean getEnabledStatus() {
        return EnabledStatus;
    }
    public void setEnabledStatus(Boolean EnabledStatus) {
        this.enabledStatus = EnabledStatus;
    }

    public Date getLastLogin(){
        return lastLogin;
    }
    public void setLastLogin(Date LastLogin){
        this.lastLogin = LastLogin;
    }








}