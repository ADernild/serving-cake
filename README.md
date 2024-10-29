# serving-cake
A simple API for serving cake

To run this crate:

```bash
AUTH_TOKEN=<SECRET TOKEN> cargo run
```

Or run it with Docker:

```bash
docker run adernild/serving-cake:latest \
    -p 8080:8080 \
    -e AUTH_TOKEN=<SECRET TOKEN>
```
