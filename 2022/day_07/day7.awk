BEGIN { path = "/" }
/^\$ cd \.\.$/ { sub(/[a-z]+\/$/, "", path) }
/^\$ cd [a-zA-Z]/ { path = path $3 "/" }
/^[0-9]/ { 
  sizes["/"] += $1; split(path, sp, /\//); tmp = "/"
  for (i in sp) {
    if (sp[i] != "") {
      sizes[tmp sp[i] "/"] += $1
      tmp = tmp sp[i] "/"
    }
  }
}
END {
  out2 = 70000000
  for (i in sizes) {
    if (sizes[i] < 100000 ) {
      out1 += sizes[i]
    }
    if (sizes["/"] - sizes[i] <= 40000000) {
      if (sizes[i] < out2) {
        out2 = sizes[i]
      }
    }
  }
  print out1 "\n" out2
}
