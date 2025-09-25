#! /usr/bin/env nu
use std/log;

let jid = (job spawn {|| dotnet run o+e> /tmp/discord-cs.log});
job tag $jid "Discord-C#";
log debug "Started"

watch . --glob "**/*[!log]" --debounce-ms 1000 {|op, path: string, npath|
  if (not ((file --mime-type $path) | str contains "text")) {
    return;    
  }

  log info $"Path changed ($path)";

  job list
    | where tag == "Discord-C#"
    | each { job kill $in.id };

  let jid = (job spawn {|| dotnet run o+e> /tmp/discord-cs.log});
  job tag $jid "Discord-C#";
  log debug "Restarted"
}
