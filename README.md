# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?

        The Observer pattern in Rust utilizes traits and structs. Traits define common behaviors for observers, while structs represent concrete observer objects. Using traits decouples subjects from observers, enhancing flexibility and extensibility. For BambangShop, choose between a trait for multiple observer types or a single struct for one type based on system requirements. Flexibility and extensibility guide the decision between using a trait or a single struct.

2. id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?

        Choosing between a Vec and DashMap in Rust depends on the operations you need. For simple iteration, a Vec suffices. For frequent lookups, updates, or deletions, DashMap is more efficient. DashMap offers concurrent reads and writes without blocking, ideal for multi-threaded contexts. Use DashMap for fast access based on unique identifiers like id or url; otherwise, a Vec may suffice, considering memory usage.

3. When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

        The Singleton pattern ensures a single instance of a class exists, useful for global access. In Rust, the lazy_static crate implements the Singleton pattern. DashMap is a thread-safe HashMap, ideal for concurrent reads and writes. Use DashMap for thread safety in multi-threaded contexts. The Singleton pattern is unnecessary for thread safety but useful for global access. Choose between DashMap and Singleton based on thread safety requirements and global access needs.

#### Reflection Publisher-2

1. In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

        Separating Service and Repository from Model enhances code modularity and maintainability. The Model focuses on data storage and business logic, while Service contains business logic and Repository handles data storage. Separating Service and Repository from Model adheres to the Single Responsibility Principle, ensuring each component has a single responsibility. This separation enhances code readability, testability, and maintainability, simplifying future changes and updates.

2. What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

        Using only the Model in Rust increases code complexity and violates the Single Responsibility Principle. The Model combines data storage and business logic, leading to bloated and complex code. Interactions between Program, Subscriber, and Notification models become convoluted, making code difficult to understand and maintain. Code complexity increases, affecting readability, testability, and maintainability. Separating Service and Repository from Model simplifies code, adhering to design principles and enhancing code quality.

3. Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

        Postman is a powerful tool for testing REST APIs, enhancing development efficiency and productivity. Postman simplifies API testing, enabling developers to send HTTP requests and view responses. Postman's features, like Collections, Environments, and Tests, streamline API testing and validation. Collections organize requests, simplifying testing workflows. Environments manage variables, enhancing testing flexibility. Tests automate validation, ensuring API functionality. Postman's features, like Collections, Environments, and Tests, enhance API testing, improving development efficiency and productivity. Postman's user-friendly interface and robust features make it an essential tool for software development projects.

#### Reflection Publisher-3

1. Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?

        In this tutorial case, we use the Push model of the Observer Pattern. The publisher notifies subscribers of changes, pushing data to them. The Push model is ideal for real-time updates, ensuring subscribers receive notifications immediately. The Push model simplifies subscriber management, enhancing system responsiveness and efficiency.

2. What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)

        Using the Pull model of the Observer Pattern in this tutorial case offers advantages and disadvantages. The Pull model requires subscribers to request data from the publisher, enhancing subscriber control over data retrieval. Subscribers pull data from the publisher, reducing unnecessary notifications and conserving resources. However, the Pull model may introduce latency, affecting real-time updates and responsiveness. Subscribers must poll the publisher for updates, increasing system complexity and resource usage. The Pull model offers subscriber control over data retrieval but may introduce latency and complexity, affecting system responsiveness and efficiency.

3. Explain what will happen to the program if we decide to not use multi-threading in the notification process.

        If we decide not to use multi-threading in the notification process, the program may experience blocking and performance issues. Without multi-threading, the program may block on notification requests, reducing responsiveness and efficiency. Multi-threading enables concurrent notification requests, enhancing system performance and scalability. Without multi-threading, the program may experience bottlenecks and delays, affecting user experience and system reliability. Multi-threading is essential for handling concurrent notification requests, ensuring system responsiveness and efficiency.