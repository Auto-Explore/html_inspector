# html/browsers/history/joint-session-history/joint-session-history-only-fully-active.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/joint-session-history/joint-session-history-only-fully-active.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Do only fully active documents count for session history?</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
  <iframe id="child"></iframe>
</body>
<script>
  async_test(function(t) {
    var old_history_len = window.history.length;
    var child = document.getElementById("child");
    var timer = window.setInterval(t.step_func(poll), 100);
    function poll() {
      if (child.getAttribute("data-child-loaded")) {
        // Check to see how many entries have been added to the session history.
        // The spec https://html.spec.whatwg.org/multipage/#joint-session-history
        // says that only fully active documents are included in the joint session history.
        // If only fully active documents count, then the only fully active document
        // is the child, with session length 1, so the joint session length change will be 1.
        // If all documents count, then the grandchild is reachable via the session history,
        // and it has session length 1, so the joint session length change will be 2.
        assert_equals(2, window.history.length - old_history_len);
        window.clearInterval(timer);
        t.done();
      }
    }
    child.src = "joint-session-history-child1.html";
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
        "byte_end": 272,
        "byte_start": 264,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1342,
        "byte_start": 272,
        "col": 9,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1351,
        "byte_start": 1342,
        "col": 1,
        "line": 30
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
  "source_name": "html/browsers/history/joint-session-history/joint-session-history-only-fully-active.html"
}
```
