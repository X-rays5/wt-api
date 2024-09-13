# User content api

### Get content from the War Thunder live feed
```http request
GET /v1/usercontent
The following options can be set as a query parameter:
content: all | image | video | camouflage | sight | mission | sight | mission | location | model | sound | controls
sort: created | rating | comments | downloads
user: string
period: number of days to look back
searchString: Array of # searches separated by spaces
page: number
```
Response
```json
{
  "data": {
    "link": "/feed/sights/",
    "list": [
      {
        "author": {
          "avatar": "https://cdn-live.warthunder.com/uploads/7f/04/3a/6bca2319dd552daf691b813adbc6fa5de9_lq/530095061%D1%8B%D0%B2%D1%8B%D0%B2%D1%8B2587960840.jpg",
          "id": 144838991,
          "nickname": "Devvsan0_0"
        },
        "comments": 0,
        "created": 1726146291,
        "description": "<p><a href=\"//live.warthunder.com/?q=%23bluearchive\" target=\"_blank\" class=\"WTL-Embed-Hashtag\">#bluearchive</a> <a href=\"//live.warthunder.com/?q=%23shiroko\" target=\"_blank\" class=\"WTL-Embed-Hashtag\">#shiroko</a> <a href=\"//live.warthunder.com/?q=%23hoshino\" target=\"_blank\" class=\"WTL-Embed-Hashtag\">#hoshino</a> <a href=\"//live.warthunder.com/?q=%23anime\" target=\"_blank\" class=\"WTL-Embed-Hashtag\">#anime</a></p>",
        "doubt": false,
        "downloads": 8,
        "featured": false,
        "file": {
          "id": 2294544,
          "link": "https://live.warthunder.com/dl/ffa20fab6daeea397cd206ecda6e7c1a649cb5c3/",
          "name": "HxS.zip",
          "size": 130054,
          "type": "application/zip"
        },
        "id": 1164640,
        "images": [
          {
            "height": 405,
            "id": 2294546,
            "ratio": 0.949999988079071,
            "src": "https://cdn-live.warthunder.com/uploads/9c/04/78/d91bddaca12a277cdb277d354fb0502ee1_lq/im124151531age.png",
            "type": "image/png",
            "width": 386
          },
          {
            "height": 347,
            "id": 2294547,
            "ratio": 1.1100000143051147,
            "src": "https://cdn-live.warthunder.com/uploads/9f/3c/d4/8d89e29c632475b3a9101adb96bcf6e0e7_lq/imag12414515e.png",
            "type": "image/png",
            "width": 386
          },
          {
            "height": 240,
            "id": 2294548,
            "ratio": 1.6100000143051147,
            "src": "https://cdn-live.warthunder.com/uploads/64/20/72/9fd7cafd1baf5f3f0d6bcea3e7e9370dbd_lq/ima3434ge.png",
            "type": "image/png",
            "width": 386
          }
        ],
        "inverted_roughness": false,
        "isMarketSuitable": true,
        "isPinned": false,
        "isSpecial": false,
        "lang_group": 1116065,
        "language": "ru",
        "likes": 13,
        "pbr_ready": false,
        "type": "sight",
        "video_info": null,
        "views": 100,
        "visible": true
      }
    ],
    "pageTitle": "WT Live // Best sights for the past 2147483647 days"
  },
  "status": "OK"
}
```