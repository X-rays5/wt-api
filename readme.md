# wt-api
Basic api for war thunder. Supporting vehicle related stuff.
## API

#### Note: Almost all data is cached for 24 hours as indicated by the updated_at json member in responses

### Get specific categories from country
```http request
GET /v1/vehicles/:country/:category,:category
```
Response
```json
{
  "helicopters": {
    "updated_at": 1652028551087,
    "vehicles": [
      {
        "name": "SA 313B Alouette II (Germany)",
        "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/sa_313b.png",
        "wiki_page": "https://wiki.warthunder.com/SA_313B_Alouette_II_(Germany)"
      }
    ]
  },
  "naval": {
    "bluewater": {
      "updated_at": 1652028550699,
      "vehicles": [
        {
          "name": "Leopard",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/germ_destroyer_class1924_leopard1932.png",
          "wiki_page": "https://wiki.warthunder.com/Leopard"
        }
      ]
    },
    "coastal": {
      "updated_at": 1652028550509,
      "vehicles": [
        {
          "name": "LS 3",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/germ_ls_class.png",
          "wiki_page": "https://wiki.warthunder.com/LS_3"
        }
      ]
    },
    "updated_at": 1652028550699
  }
}
```

### Get all categories of vehicles from country
```http request
GET /v1/vehicles/:country/all
```
Response
```json
{
  "aircraft": {
    "updated_at": 1652028615147,
    "vehicles": [
      {
        "name": "Bf 109 B-1",
        "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/bf-109b_2.png",
        "wiki_page": "https://wiki.warthunder.com/Bf_109_B-1"
      }
    ]
  },
  "ground": {
    "updated_at": 1652028480237,
    "vehicles": [
      {
        "name": "Sd.Kfz.221 (s.Pz.B.41)",
        "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/germ_sdkfz_221_s_pz_b_41.png",
        "wiki_page": "https://wiki.warthunder.com/Sd.Kfz.221_(s.Pz.B.41)"
      }
    ]
  },
  "naval": {
    "bluewater": {
      "updated_at": 1652028550699,
      "vehicles": [
        {
          "name": "Leopard",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/germ_destroyer_class1924_leopard1932.png",
          "wiki_page": "https://wiki.warthunder.com/Leopard"
        }
      ]
    },
    "coastal": {
      "updated_at": 1652028550509,
      "vehicles": [
        {
          "name": "LS 3",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/germ_ls_class.png",
          "wiki_page": "https://wiki.warthunder.com/LS_3"
        }
      ]
    },
    "updated_at": 1652028550699
  }
}
```

### Get categories of vehicle from all countries
```http request
GET /v1/vehicles/all/:category,:category
```
Response
```json
{
  "ground": {
    "sweden": {
      "updated_at": 1652028727834,
      "vehicles": [
        {
          "name": "Strv m/31",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/sw_strv_m31.png",
          "wiki_page": "https://wiki.warthunder.com/Strv_m/31"
        }
      ]
    }
  },
  "helicopters": {
    "sweden": {
      "updated_at": 1652028728267,
      "vehicles": [
        {
          "name": "HKP3C",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/hkp3c.png",
          "wiki_page": "https://wiki.warthunder.com/HKP3C"
        }
      ]
    }
  }
}
```

### Get all categories from all countries
```http request
GET /v1/vehicles/all/all
```
Response
```json
{
  "ground": {
    "sweden": {
      "updated_at": 1652028727834,
      "vehicles": [
        {
          "name": "Strv m/31",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/sw_strv_m31.png",
          "wiki_page": "https://wiki.warthunder.com/Strv_m/31"
        }
      ]
    }
  },
  "helicopters": {
    "sweden": {
      "updated_at": 1652028728267,
      "vehicles": [
        {
          "name": "HKP3C",
          "thumbnail_img_url": "https://encyclopedia.warthunder.com/slots/hkp3c.png",
          "wiki_page": "https://wiki.warthunder.com/HKP3C"
        }
      ]
    }
  }
}
```

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
GET /v1/categories/has/:category,:category/:country
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
GET /v1/categories/countries
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
GET /v1/countries/have/:category
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