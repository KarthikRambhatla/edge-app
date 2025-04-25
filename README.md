# Edge App

A modular edge computing application for sensor data collection and processing.

## Developer Notes

### Project Structure

#### Config Module
The configuration needed to send a request. For example, we want to connect to a edge device/sensor and get data, what all things we need to know - Protocol, end point, request body, may be that's all it is. All of this will be in this config. This could be from a file or nats object store or kv.

Also, we should be able to load multiple configs, multiple config sources. changes to source must be watched and notified across, without needing to restart the app.

#### Data Source Module
Once we know the protocol, we will create a data source manager or an adapter that can connect and get the data.

#### Core Module
Abstracted away from protocol we follow, how we want to handle request, we want to poll? or we want to have a call back structure, for CnC use case and publish response

Since this is not delivering messages to subsccriptions that dynamically changes, observer, delegates might not fit or performant. May be we should go with channels (crossbeam? may be normal mpsc for now). so we can somehow reuse the same connection for unique sinks, and send when received. Backpressure handling (when the actual sink is not fast enough to consume, this program channel gets filled up)


#### Data Sink Module
To whom we publish response, those specific implementation, may be like nats sink, or to cloud hub, it could be anything

### Quick Tips
- cargo clippy 
- cargo check
- cargo build
- cargo run
- cargo doc

### Naming

what should we name this application?
* sensorhub
* ApppX
* BredJ
* Edzapp
