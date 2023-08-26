## How to Use

1. **Compilation**: Ensure you have Rust installed on your system. Clone the repository and navigate to the project directory.

2. **Build the Daemon**: Use the following command to build the daemon:

   ```sh
   cargo build --release
   ```

3. **Run the Daemon**: To run the daemon, provide the required parameters `tun_device_name` and `tun_device_ip`. For example:

   ```sh
   ./target/release/tun-dev-daemon tun0 192.168.1.2
   ```

   Replace `tun0` with the actual TUN device name and `192.168.1.2` with the desired IP address.

4. **Workflow**: The daemon follows the workflow outlined in the ASCII art diagram above. It initializes, checks the TUN device, stores old routes, sets specific and default routes, monitors the TUN device, and reverts changes if needed.

5. **Stopping the Daemon**: To stop the daemon, press Ctrl+C in the terminal where it is running.

Please ensure that you have necessary permissions to modify routes and interact with network configuration. The daemon will handle routing changes as per the provided workflow.


# Tun Device Daemon Workflow

```sql
  +-------+             +-------------------+
  | Start |------------>| Initialize Daemon |
  +-------+             +-------------------+
                                   |
                                   v
           +--------------------------------------+
           | Receive 'tun device name' and 'tun   |
           | device connection IP address'        |
           +--------------------------------------+
                                   |
                                   v
                        +------------------------+
                        | Check Tun Device        |
                        | Existence              |
                        +------------------------+
                                   |
                          +--------+---------+
                          |                  |
                          |                  |
                          v                  v
              +-------------------+    +-----------------+
              | Store Old Default |    | Log an error and|
              | Route             |    | exit or retry   |
              +-------------------+    +-----------------+
                          |
                          v
              +-------------------+
              | Set Specific Route|
              +-------------------+
                          |
                          v
              +-------------------+
              | Set Default Route |
              +-------------------+
                          |
                          v
              +-------------------+
              | Monitor Tun Device|
              +-------------------+
                          |
                +---------+----------+
                |                    |
                v                    v
    +----------------------+   +------------------+
    | Revert to Old Default|   | Keep Monitoring  |
    | Route                |   +------------------+
    +----------------------+
                |
                v
    +----------------------+
    | Remove Specific Route |
    +----------------------+
                |
                v
    +----------------------+
    |         Stop         |
    +----------------------+

```