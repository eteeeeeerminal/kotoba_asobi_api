docker build -t rusttextapi .
docker run --name api_dev -it -v ${PWD}:/app/project -p 8080:8080 rusttextapi