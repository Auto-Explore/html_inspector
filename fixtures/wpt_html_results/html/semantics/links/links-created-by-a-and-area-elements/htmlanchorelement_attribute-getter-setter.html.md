# html/semantics/links/links-created-by-a-and-area-elements/htmlanchorelement_attribute-getter-setter.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/htmlanchorelement_attribute-getter-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
<head>
<title>HTMLAnchorElement getters and setters</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<a>anchor</a>
<script>
function test_gettersetter(property, oldresult, newval, newresult, oldurl, newurl) {
  var a = document.querySelector('a');
  a.href = oldurl;
  var r1 = a[property];
  assert_equals(r1, oldresult);
  a[property] = newval;
  var r2 = a[property];
  assert_equals(r2, newresult);
  var r3 = a.href;
  assert_equals(r3, newurl);
}

//Elements for each test: [property, oldresult, newvalue, newresult, oldurl, newurl]
//                         [0]       [1]        [2]       [3]        [4]     [5]
tests = [
  ["hash", "#somehash", "someother", "#someother",
   "http://google.com/index.html#somehash",
   "http://google.com/index.html#someother"],
  ["hash", "#somehash", "#someother", "#someother",
   "http://google.com/index.html#somehash",
   "http://google.com/index.html#someother"],
  ["host", "google.com:1234", "github.com:4444", "github.com:4444",
   "http://google.com:1234/somedir",
   "http://github.com:4444/somedir"],
  ["hostname", "google.com", "github.com", "github.com",
   "http://google.com:1234/somedir",
   "http://github.com:1234/somedir"],
  ["href", "http://google.com:1234/somedir", "http://goo-gle.com:1234/other/x.html", "http://goo-gle.com:1234/other/x.html",
   "http://google.com:1234/somedir",
   "http://goo-gle.com:1234/other/x.html"],
  ["password", "flabada", "blubb", "blubb",
   "https://anonymous:flabada@developer.mozilla.org/en-US/docs/",
   "https://anonymous:blubb@developer.mozilla.org/en-US/docs/"],
  ["pathname", "/somedir/someotherdir/index.html", "/newpath/x.txt", "/newpath/x.txt",
   "http://google.com:1234/somedir/someotherdir/index.html",
   "http://google.com:1234/newpath/x.txt"],
  ["port", "1234", "4444", "4444", "http://google.com:1234/somedir", "http://google.com:4444/somedir"],
  ["protocol", "http:", "ftp:", "ftp:", "http://google.com/somedir", "ftp://google.com/somedir"],
  ["protocol", "http:", "ftp", "ftp:", "http://google.com/somedir", "ftp://google.com/somedir"],
  ["search", "?ho", "?hi", "?hi", "http://google.com/q.php?ho", "http://google.com/q.php?hi"],
  ["search", "?ho", "hi", "?hi", "http://google.com/q.php?ho", "http://google.com/q.php?hi"],
  ["search", "?ho", "?hi", "?hi", "http://google.com/?ho", "http://google.com/?hi"],
  ["search", "?ho", "hi", "?hi", "http://google.com/?ho", "http://google.com/?hi"],
  ["username", "anonymous", "wellknown", "wellknown",
   "https://anonymous:pwd@developer.mozilla.org:1234/en-US/",
   "https://wellknown:pwd@developer.mozilla.org:1234/en-US/"]
];

for (var i = 0; i < tests.length; i++) {
    test(function() {
        test_gettersetter(tests[i][0], tests[i][1], tests[i][2], tests[i][3], tests[i][4], tests[i][5])
    }, "Getter and setter for attribute of anchor element(" + i + "):" + tests[i][0] );
}
</script>
</head>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “head”.",
      "severity": "Error",
      "span": {
        "byte_end": 2985,
        "byte_start": 2978,
        "col": 1,
        "line": 64
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/htmlanchorelement_attribute-getter-setter.html"
}
```
