{
  x[$3]++;
  # y[$3] = $0;
  if (y[$3]) {
    y[$3] = y[$3] "\n" $0
  } else {
    y[$3] = $0
  }
} END {
  for (i in x) {
    target="on_"  tolower(i)  ".sql";
    if (x[i] == 1) {
      # print y[i] > target;
    } else {
      print "------------------------"
      print target ": " x[i] ": \n" y[i]; 
    }
  }
}
