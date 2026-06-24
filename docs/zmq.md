# QuaSAR Schema ZeroMQ Flow

The protobuf schema describes payloads and socket roles only. Endpoints, topics,
high-water marks, authentication, and reconnect policy are runtime
configuration.

Socket roles are documented twice:

- as human-readable comments near each transmitted message;
- as protobuf descriptor options from `quasar/zmq.proto`.

## Nav Telemetry

| Direction          | Socket pattern | Message                   |
|--------------------|----------------|---------------------------|
| SAR to subscribers | `PUB` -> `SUB` | `quasar.pb.nav.Telemetry` |

`Telemetry` carries the latest full navigation state. Subscribers should treat each
message as a complete sample, not a delta.

## SAR Commands

| Direction             | Socket pattern       | Message                 |
|-----------------------|----------------------|-------------------------|
| Ground/control to SAR | `DEALER` -> `ROUTER` | `xlink.pb.sar.Request`  |
| SAR to ground/control | `ROUTER` -> `DEALER` | `xlink.pb.sar.Response` |

`Request.Header.request_id` correlates responses with requests.`retry_count` is carried in
the message so request senders and routers can preserve retry intent across
transport reconnects.

`Response.header` repeats the original request header.