# User content api

#### Note: The user content data is cached for ~1 hour as indicated by the updated_at timestamp

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
  "feed": {
    "data": {
      "link": "/feed/models/",
      "list": [
        {
          "author": {
            "avatar": "https://cdn-live.warthunder.com/uploads/89/8c/69/a7cd9ebf031f11e0f88664a000535176cf_lq/MojoJojo2.png",
            "id": 32069646,
            "nickname": "Sinister_Monkee"
          },
          "comments": 1,
          "created": 1726029501,
          "description": "\u003Cp\u003EVERSION 2.39 - \"Dance of Dragons\" - French Coastal ships added!!\u003C/p\u003E\u003Cp\u003EThis is an Excel spreadsheet that I developed over the last couple of years to help keep track of the following things.\u003C/p\u003E\u003Cp\u003E- Track each vehicle and its condition (status).\u003Cbr /\u003E\n- Track the crew assignments for each vehicle.\u003Cbr /\u003E\n- Arrange the vehicles by battle rating.\u003Cbr /\u003E\n- Arrange the vehicles by name.\u003Cbr /\u003E\n- Keep a running total of researched vehicles and their projected cost.\u003Cbr /\u003E\n- View a summary of all vehicles owned by nation and rank.\u003C/p\u003E\u003Cp\u003EWith the large scope of aircraft, ground vehicles, and ships in War Thunder, it is easy to get overwh...\u003C/p\u003E",
          "doubt": false,
          "downloads": 2,
          "featured": false,
          "file": {
            "id": 2293774,
            "link": "https://live.warthunder.com/dl/84730728f6bc1553cb90c91f4f84f24bc63b1a36/",
            "name": "WarThunderVehicles_v2.39.zip",
            "size": 1174870,
            "type": "application/zip"
          },
          "id": 1164490,
          "images": [
            {
              "height": 151,
              "id": 1870730,
              "ratio": 2.55999994277954,
              "src": "https://cdn-live.warthunder.com/uploads/81/72/b2/a6fa27e0b548229e19618f7cc96ec74575_lq/US+Army06.jpg",
              "type": "image/jpeg",
              "width": 386
            }
          ],
          "inverted_roughness": false,
          "isMarketSuitable": true,
          "isPinned": false,
          "isSpecial": false,
          "lang_group": 1115922,
          "language": "en",
          "likes": 1,
          "pbr_ready": true,
          "type": "model",
          "video_info": null,
          "views": 36,
          "visible": true
        }
      ],
      "pageTitle": "WT Live // Best models for the past 2147483647 days"
    },
    "status": "OK"
  },
  "updated_at": 1726235711912
}
```