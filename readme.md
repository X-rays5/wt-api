# wt-api
Basic api for war thunder. Supporting vehicle related stuff.
## API

#### Note: All data is cached for 24 hours as indicated by the updated_at json member in responses

### Get all existing categories
```http request
GET /v1/categories
```
Response
```json
{
  "categories": [
    "aircraft",
    "ground",
    "naval"
  ],
  "updated_at": 1651944857040
}
```

### Check if country has categories
```http request
GET v1/categories/has/:category,:category/:country
```
Response
```json
{
  "ground": true,
  "naval": false
}
```

### Get all categories a country has
```http request
GET v1/categories/countries
```
Response
```json
{
  "britain": {
    "aircraft": true,
    "ground": true,
    "helicopters": true,
    "naval": true
  },
  "china": {
    "aircraft": true,
    "ground": true,
    "helicopters": false,
    "naval": false
  },
  "france": {
    "aircraft": true,
    "ground": true,
    "helicopters": true,
    "naval": false
  }
}
```

### Get all existing countries
```http request
GET /v1/countries
```
Response
```json
{
  "countries": [
    "britain",
    "china",
    "france",
    "germany",
    "israel",
    "italy",
    "japan",
    "sweden",
    "usa",
    "ussr"
  ],
  "updated_at": 1651940214074
}
```

### Get all countries with certain category
```http request
GET v1/countries/have/:category
```
Response
```json
{
  "britain": true,
  "china": false,
  "france": false,
  "germany": true,
  "israel": false,
  "italy": true,
  "japan": true,
  "sweden": false,
  "usa": true,
  "ussr": true
}
```