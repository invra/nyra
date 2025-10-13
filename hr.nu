#   SPDX-License-Identifier: Unlicense
#   Project: Nyra
#   File: hr.nu
#   Authors: Invra
#   Notes: Hot-reload Nushell script

#!/usr/bin/env nu 
use std/log

let bname = "Nyra"
let log_p: string = if ($nu.os-info.name == "windows") {
  $"($env.TEMP)\\discord-cs.log"
} else {
  "/tmp/discord-cs.log"
}

print "Press R to reload and Q to quit"

loop {
  let jid = job spawn -t "Spawner Thread" {||
    let _ = (job spawn { dotnet run o+e>> $log_p } -t "Discord-C#");
    
    watch . --glob "**/*[!log]" --debounce-ms 1000 -q {|op, path: string, npath|
      pkill $bname
      if (not ((file --mime-type $path) | str contains "text")) {
        return
      }

      job list
        | where tag == "Discord-C#"
        | each { job kill $in.id }

      let _ = (job spawn { dotnet run o+e>> $log_p } -t "Discord-C#");
    }
  }

  mut key: string =  (input listen --types [key]).code;
  while not ($key in [q r]) {
    $key =  (input listen --types [key]).code;
  }

  pkill $bname
  job kill $jid

  let  _ = (ps -l | where ($it.cwd == (pwd) and $it.name =~ "dotnet") | each { kill $in.pid });
  if $key == "q" {
     break;   
  }
  print "reloaded"
}

print "Exiting…"
