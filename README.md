# SolarmanLogger

## Work In Progress

![image](https://user-images.githubusercontent.com/85478566/170827903-9ce4ba94-a6a2-4015-af2e-dae8b62c2a7b.png)

### Notes

- `Origin not allowed` behind nginx reverse proxy

Add the following line above the `proxy_pass`
```
proxy_set_header Host $http_host;
```
