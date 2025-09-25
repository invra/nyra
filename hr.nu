#!/usr/bin/env nu 
use std/log

let log_p: string = if ($nu.os-info.name == "windows") {
  $"($env.TEMP)\\discord-cs.log"
} else {
  "/tmp/discord-cs.log"
}

print "Press r to reload and q to quit"

loop {
  let jid = job spawn -t "Spawner Thread" {||
    job spawn { dotnet run o+e>> $log_p } -t "Discord-C#"
    
    watch . --glob "**/*[!log]" --debounce-ms 1000 -q {|op, path: string, npath|
      if (not ((file --mime-type $path) | str contains "text")) {
        return
      }

      job list
        | where tag == "Discord-C#"
        | each { job kill $in.id }

      job spawn { dotnet run o+e>> $log_p } -t "Discord-C#"
    }

  }
  mut key: string =  (input listen --types [key]).code;
  while not ($key in [q r]) {
    $key =  (input listen --types [key]).code;
  }
  job kill $jid
  ps -l | where ($it.cwd == (pwd) and $it.name =~ "dotnet") | each { kill $in.pid }
  if $key == "q" {
     break;   
  }
  print "reloaded"
}

print "Exiting..."
