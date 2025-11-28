
## Market Movers

### `actives`

Get most active stocks.

# TODO: Add volume to the logic of the movers.

**Example Query:**
```graphql
query {
  actives {
    symbol
    name
    price
    change
    percentChange
  }
}
```

**Response**
```json
  "data": {
    "actives": [
      {
        "symbol": "NVDA",
        "name": "NVIDIA Corporation",
        "price": "180.26",
        "change": "2.44",
        "percentChange": "1.37%"
      },
      {
        "symbol": "ONDS",
        "name": "Ondas Holdings Inc.",
        "price": "8.24",
        "change": "-0.20",
        "percentChange": "-2.37%"
      },
      {
        "symbol": "OPEN",
        "name": "Opendoor Technologies Inc.",
        "price": "7.78",
        "change": "0.04",
        "percentChange": "0.52%"
      },
      {
        "symbol": "PLUG",
        "name": "Plug Power Inc.",
        "price": "1.9800",
        "change": "0.0300",
        "percentChange": "1.54%"
      },
      {
        "symbol": "BBAI",
        "name": "BigBear.ai Holdings, Inc.",
        "price": "6.02",
        "change": "-0.17",
        "percentChange": "-2.75%"
      },
      {
        "symbol": "NIO",
        "name": "NIO Inc.",
        "price": "5.46",
        "change": "-0.04",
        "percentChange": "-0.73%"
      },
      {
        "symbol": "CLSK",
        "name": "CleanSpark, Inc.",
        "price": "13.45",
        "change": "1.63",
        "percentChange": "13.79%"
      },
      {
        "symbol": "BMNR",
        "name": "Bitmine Immersion Technologies, Inc.",
        "price": "31.74",
        "change": "2.83",
        "percentChange": "9.79%"
      },
      {
        "symbol": "MARA",
        "name": "MARA Holdings, Inc.",
        "price": "11.11",
        "change": "-0.06",
        "percentChange": "-0.54%"
      },
      {
        "symbol": "WULF",
        "name": "TeraWulf Inc.",
        "price": "14.84",
        "change": "0.90",
        "percentChange": "6.46%"
      },
      {
        "symbol": "INTC",
        "name": "Intel Corporation",
        "price": "36.81",
        "change": "0.98",
        "percentChange": "2.74%"
      },
      {
        "symbol": "SNDK",
        "name": "Sandisk Corporation",
        "price": "215.04",
        "change": "-5.46",
        "percentChange": "-2.48%"
      },
      {
        "symbol": "GOOGL",
        "name": "Alphabet Inc.",
        "price": "319.95",
        "change": "-3.49",
        "percentChange": "-1.08%"
      },
      {
        "symbol": "NU",
        "name": "Nu Holdings Ltd.",
        "price": "17.25",
        "change": "0.64",
        "percentChange": "3.85%"
      },
      {
        "symbol": "HOOD",
        "name": "Robinhood Markets, Inc.",
        "price": "128.20",
        "change": "12.63",
        "percentChange": "10.93%"
      },
      {
        "symbol": "KVUE",
        "name": "Kenvue Inc.",
        "price": "17.22",
        "change": "0.19",
        "percentChange": "1.12%"
      },
      {
        "symbol": "AMD",
        "name": "Advanced Micro Devices, Inc.",
        "price": "214.24",
        "change": "8.11",
        "percentChange": "3.93%"
      },
      {
        "symbol": "DNN",
        "name": "Denison Mines Corp.",
        "price": "2.5600",
        "change": "0.0900",
        "percentChange": "3.64%"
      },
      {
        "symbol": "F",
        "name": "Ford Motor Company",
        "price": "13.19",
        "change": "0.02",
        "percentChange": "0.15%"
      },
      {
        "symbol": "WBD",
        "name": "Warner Bros. Discovery, Inc.",
        "price": "23.88",
        "change": "0.92",
        "percentChange": "4.01%"
      },
      {
        "symbol": "AMZN",
        "name": "Amazon.com, Inc.",
        "price": "229.16",
        "change": "-0.51",
        "percentChange": "-0.22%"
      },
      {
        "symbol": "RIG",
        "name": "Transocean Ltd.",
        "price": "4.3000",
        "change": "0.2500",
        "percentChange": "6.17%"
      },
      {
        "symbol": "RIVN",
        "name": "Rivian Automotive, Inc.",
        "price": "16.18",
        "change": "0.62",
        "percentChange": "3.98%"
      },
      {
        "symbol": "GRAB",
        "name": "Grab Holdings Limited",
        "price": "5.32",
        "change": "0.08",
        "percentChange": "1.53%"
      },
      {
        "symbol": "BBD",
        "name": "Banco Bradesco S.A.",
        "price": "3.6800",
        "change": "0.1400",
        "percentChange": "3.95%"
      },
      {
        "symbol": "BTG",
        "name": "B2Gold Corp.",
        "price": "4.4700",
        "change": "0.1400",
        "percentChange": "3.23%"
      },
      {
        "symbol": "PLTR",
        "name": "Palantir Technologies Inc.",
        "price": "165.77",
        "change": "2.22",
        "percentChange": "1.36%"
      },
      {
        "symbol": "HBAN",
        "name": "Huntington Bancshares Incorporated",
        "price": "16.27",
        "change": "0.09",
        "percentChange": "0.56%"
      },
      {
        "symbol": "AGNC",
        "name": "AGNC Investment Corp.",
        "price": "10.56",
        "change": "0.16",
        "percentChange": "1.54%"
      },
      {
        "symbol": "BULL",
        "name": "Webull Corporation",
        "price": "9.26",
        "change": "0.56",
        "percentChange": "6.44%"
      },
      {
        "symbol": "ABEV",
        "name": "Ambev S.A.",
        "price": "2.5100",
        "change": "0.0000",
        "percentChange": "0.00%"
      },
      {
        "symbol": "RGTI",
        "name": "Rigetti Computing, Inc.",
        "price": "25.57",
        "change": "-0.51",
        "percentChange": "-1.96%"
      },
      {
        "symbol": "ACHR",
        "name": "Archer Aviation Inc.",
        "price": "7.49",
        "change": "0.11",
        "percentChange": "1.49%"
      },
      {
        "symbol": "PFE",
        "name": "Pfizer Inc.",
        "price": "25.71",
        "change": "-0.01",
        "percentChange": "-0.04%"
      },
      {
        "symbol": "UPWK",
        "name": "Upwork Inc.",
        "price": "19.58",
        "change": "-0.11",
        "percentChange": "-0.56%"
      },
      {
        "symbol": "VALE",
        "name": "Vale S.A.",
        "price": "12.51",
        "change": "0.32",
        "percentChange": "2.63%"
      }
    ]
  }
}
```

### `gainers`

Get top gaining stocks.

**Example Query:**
```graphql
query {
  gainers {
    symbol
    name
    price
    change
    percentChange
  }
}
```

### `losers`

Get top losing stocks.

**Example Query:**
```graphql
query {
  losers {
    symbol
    name
    price
    change
    percentChange
  }
}
```