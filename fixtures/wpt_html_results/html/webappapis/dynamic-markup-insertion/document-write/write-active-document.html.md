# html/webappapis/dynamic-markup-insertion/document-write/write-active-document.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/write-active-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write only writes to active documents</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body><div id="log"></div></body>
<script>
  async_test(function(t) {
    var child = document.createElement("iframe");
    child.src = "empty.html?1";
    child.onload = t.step_func(function() {
      var child1 = child.contentDocument;
      var link = child1.createElement("a");
      link.href = "data:text/html,Clicked.";
      link.innerText = "Link.";
      child1.body.appendChild(link);
      var grandchild = child1.createElement("iframe");
      grandchild.src = "empty.html?2";
      grandchild.onload = t.step_func(function() {
        var grandchild1 = grandchild.contentDocument;
        child.onload = t.step_func(function() {
          // This is a write to an inactive document
          child1.write('WRITE HAPPENED');
          assert_equals(child1.body.lastChild.tagName, "IFRAME");
          // This is a write to an active but not fully active document
          grandchild1.write('WRITE HAPPENED');
          assert_equals(grandchild1.body.innerHTML, "WRITE HAPPENED");
          t.done();
        });
        link.click();
      });
      child1.body.appendChild(grandchild);
    });
    document.body.appendChild(child);
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 226,
        "byte_start": 218,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1337,
        "byte_start": 226,
        "col": 9,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1346,
        "byte_start": 1337,
        "col": 1,
        "line": 35
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/write-active-document.html"
}
```
