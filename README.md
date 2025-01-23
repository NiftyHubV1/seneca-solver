# Seneca Solver

A simple program that automatically solves your assignments in Seneca (and also gets you a bunch of free XP 🔥). Note that this is still early in development and may have bugs.

It should work on most assignments but probably will not complete every single one. This is due to the fact that not all question types are supported at the moment, although this is currently being worked on. In addition, it **will not work** on the AI-marked "Exam Questions" at the end of assignments. There is very little that can be done about that since that's marked on Seneca's servers.

## How to set up

### Method 1: Executable (recommended)

Download the executable from the releases page to the side, making sure to download the latest release. If on a UNIX-based system (i.e. not using Windows or TempleOS), add the execute permission to the executable using `chmod +x <name-of-executable>`. Move on to the "Usage" section below. If your OS/system architecture is unsupported, use Method 2.

### Method 2: Compile it yourself

#### Requirements

You must have the following installed:

- Cargo (any relatively new version will do)

#### Steps

1. Run `git clone https://github.com/ArcaEge/seneca-solver.git` to clone the repository to somewhere convenient.

1. Open the repository directory in a terminal and run `cargo build --release`. This will compile the program and place the executable in `target/release/seneca-solver`.

## How to use

1. Run the executable. If you ran it from your file manager, it should've spawned a terminal window.

1. At this point it should be asking for an access key. Here's how to obtain one:
    1. Go to the Seneca dashboard page in your browser and open the inspect menu.
    1. Navigate to the Network tab.
    1. Reload the page. Requests should start appearing in the Network tab.
    1. There should be a filter bar to filter the URLs. Type `authentication.app.senecalearning.com` into the filter bar.
    ![Image showing what this should look like](docs/images/network_filter.png)
    1. Select the latest `POST` request displayed (example: selected request in image above) to view its headers.
    1. In the info panel that opens, scroll down to "Request Headers". This might have a different name in your browser.
    1. Copy the value of the HTTP header named `access-key` (making sure to copy the whole thing) and paste it into the script. **Note:** Some browsers (mainly Firefox-based ones at the time of writing) truncate the access key in the middle since it is very long. Right click it and click copy instead of selecting everything and copying.
    ![Image showing what this should look like](docs/images/network_access_key.png)

1. Your assignments will be printed to the terminal. Use the arrow keys to navigate between them. Press `Enter` to start solving an assignment.

1. The script should now start solving the assignment. This may 5 or more minutes depending on the length of the assignment. Also note that your access key will expire every hour, meaning you will need to repeat the process to obtain the key every hour. Using refresh tokens instead of access keys is on the To Do list.

## Troubleshooting/common errors

### Expired access key

``` rust
reqwest::Error { kind: Status(401) ... }
```

Your access key has expired, obtain a new one.

### Ratelimit

``` rust
reqwest::Error { kind: Status(403) ... }
```

You have been rate-limited by Seneca, wait for a few minutes and try again.
