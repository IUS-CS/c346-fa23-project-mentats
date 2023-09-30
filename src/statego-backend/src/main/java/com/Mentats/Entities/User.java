package com.Mentats;

import jakarta.persistence.*;
import java.util.Date;


@Entity // This tells Hibernate to make a table out of this class
@Table(name = "User Table")
public class User {
    //Unique ID is automatically generated with the following annotations
    @Id
    @GeneratedValue(strategy=GenerationType.AUTO)
    private Integer id;

    //contains the userName of the User
    @Column(name = "Username")
    private String userName;

    //contains the associated email of the User
    @Column(name = "Email")
    private String email;

    //contains the private password of the User
    @Column(name = "Password")
    private String password;

    //denotes whether the User is "enabled" (will be expanded later)
    @Column(name = "Enabled Status")
    private Boolean enabledStatus;

    //contains information on the last date that the user logged in
    @Column(name = "Last Login")
    private Date lastLogin;

    //contains information on the date of account creation
    @Column(name = "First Logged In")
    private Date firstLogin;

    //contains a public user generated biography of the User
    @Column(name = "Biography")
    private String biography;

    //contains a link to a profile picture for the User
    @Column(name = "Profile Picture Link")
    private String profilePicLink;

    //no arg constructor required for JPA
    public User(){

    }
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

    public Date getFirstLogin() {
        return firstLogin;
    }

    public void setFirstLogin(Date firstLogin) {
        this.firstLogin = firstLogin;
    }

    public String getBiography() {
        return biography;
    }

    public void setBiography(String biography) {
        this.biography = biography;
    }

    public String getProfilePicLink() {
        return profilePicLink;
    }

    public void setProfilePicLink(String profilePicLink) {
        this.profilePicLink = profilePicLink;
    }






}