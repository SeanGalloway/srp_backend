# SRP Backend

SRP Backend is the server-side application for a simple authentication application

## Stack

- Actix
- Sea Orm
- Tokio
- Team City

## Authentication Scheme

Username/password to login
JWT with 2-hour expiration upon successful login

## Project result
Results were mixed. 
The app is fully functional running locally, but I failed to get it completely running in AWS.

The app was deployed to EKS, but there was a network issue preventing connectivity to the database, and EKS costs got too high before I could fix the issue.

Next steps:
- get backend running and connected to db in EKS
- Deploy UI to a subdomain of simplebrilliance.org