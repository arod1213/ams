
# **AMS — Audio Management System**

AMS is a lightweight command-line tool that scans your file system for audio production project files. From your current working directory, AMS recursively searches for **Ableton Live**, **Pro Tools**, **Logic Pro X**, **Cubase**, and **Reaper** sessions—then returns them sorted from **most recent → oldest** with fast paging and simple commands.

----------

## **Features**

    
-   **Sorts results automatically** by last modified date
    
-   **Navigate pages** using your arrow keys
    
-   **Filter by audio files or backups**
    
-   **Open sessions instantly** via command
    
-   Zero setup inside projects — just install and run
    

----------

## **Installation**

Place the compiled `ams` binary into `/usr/local/bin`:

`sudo install ams /usr/local/bin` 

----------

## **Usage Overview**

Run AMS from **any folder** you want to search (e.g., your music folder):


```bash
cd ~/Music

ams list -n "My Song"
```

----------

## **Commands**

### **List Sessions**

Search for sessions from your current directory downward.

`ams list` 

**Options:**

Flag

Description

`-a`, `--audio`

Include audio files (bounces, renders, stems)

`-b`, `--backups`

Include DAW backup/session-backup files 
OR 
if searching for audio, show stems

`-n <NAME>`, `--name <NAME>`

Filter results by name

`-h`, `--help`

----------

##  Supported DAW Formats

AMS currently finds:

-   **Ableton Live** (`.als`)
    
-   **Pro Tools** (`.ptx`, `.pts`)
    
-   **Logic Pro X** (`.logicx`, `.logic`)
    
-   **Cubase/Nuendo** (`.cpr`)
    
-   **Reaper** (`.rpp`)
    
-   **Audio files** (optional): `.wav`, `.aiff`, `.mp3`, etc.
    
-   **Backup files** (various DAW-specific formats)
    
