build-gin:
    cd ben-gin && go build -o ben-gin

run-gin:
    ./ben-gin/ben-gin

build-echo:
    cd ben-echo && go build -o ben-echo

run-echo:
    ./ben-echo/ben-echo

build-fiber:
    cd ben-fiber && go build -o ben-fiber

run-fiber:
    ./ben-fiber/ben-fiber

build-std:
    cd ben-std && go build -o ben-std

run-std:
    ./ben-std/ben-std

build-quarkus:
    cd ben-quarkus && ./mvnw clean package -DskipTests -Dquarkus.package.type=uber-jar

run-quarkus:
    java -jar ben-quarkus/target/ben-quarkus-1.0.0-SNAPSHOT-runner.jar

build-spring-boot:
    cd ben-spring-boot && ./gradlew clean bootJar

run-spring-boot:
    java -jar ben-spring-boot/build/libs/ben-spring-boot-0.0.1-SNAPSHOT.jar

build-actix-web:
    cd ben-actix-web && cargo build -r --target-dir=target

run-actix-web:
    ./ben-actix-web/target/release/ben-actix-web

build-axum:
    cd ben-axum && cargo build -r --target-dir=target

run-axum:
    ./ben-axum/target/release/ben-axum

build-all: build-gin build-echo build-fiber build-quarkus build-spring-boot build-actix-web build-axum

run-fastapi:
    cd ben-fastapi && uv run main.py

run-express:
    cd ben-express && pnpm start

run-fastify:
    cd ben-fastify && pnpm start

run-benchmark:
    wrk -t2 -c40 -d15s http://localhost:8080/
