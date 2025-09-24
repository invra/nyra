#! /usr/bin/env nu
use std/log;

watch . {|op, path, npath|
  if (not ((file --mime-type $path) | str contains "text")) {
    return;    
  }

  log info $"Path changed ($path)";

  job list
    | where tag == "Discord-C#"
    | each { job kill $in.id };

  let jid = (job spawn {|| dotnet run});
  job tag $jid "Discord-C#";

  log debug "reloaded"
}
